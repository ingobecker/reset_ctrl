use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcPin, InterruptHandler};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Flex, Input, Level, Output, Pull, Speed};
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::Driver;
use embassy_stm32::{adc, bind_interrupts, peripherals, usb, Config, Peripheral};
use embassy_time::{Delay, Timer};
use embassy_usb::class::midi::{MidiClass, Sender};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;

use defmt::info;
use static_cell::StaticCell;

use crate::output::CHANNEL;
use crate::ui::Backend;

bind_interrupts!(struct ADCIrqs {
        ADC1_2 => adc::InterruptHandler<peripherals::ADC1>;
});

bind_interrupts!(struct USBIrqs {
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

type USBDriver = Driver<'static, peripherals::USB>;
type USBMidiClass = MidiClass<'static, USBDriver>;

pub struct Stm32Backend {
    addr: u8,
    out_a: Output<'static>,
    out_b: Output<'static>,
    out_c: Output<'static>,
    flex_com: Flex<'static>,
    adc: Adc<'static, peripherals::ADC1>,
    adc_pin: peripherals::PA4,
    usb_builder: Option<Builder<'static, USBDriver>>,
    usb_midi_class: Option<USBMidiClass>,
    spi: Spi<'static, peripherals::SPI1, NoDma, NoDma>,
    rclk: Output<'static>,
}

impl Backend for Stm32Backend {
    async fn read_adc(&mut self) -> u16 {
        self.set_addr();
        self.flex_com.set_as_input(Pull::None);
        let v = self.adc.read(&mut self.adc_pin).await;
        self.next();
        v
    }

    fn read_input(&mut self) -> bool {
        self.set_addr();
        self.flex_com.set_as_input(Pull::Up);
        let level = self.flex_com.is_high();
        self.next();
        level
    }

    fn rewind(&mut self) {
        self.addr = 0;
    }
}

#[embassy_executor::task(pool_size = 1)]
pub async fn usb_task(mut usb: embassy_usb::UsbDevice<'static, USBDriver>) -> ! {
    usb.run().await;
    unreachable!("usb task ended")
}

#[embassy_executor::task(pool_size = 1)]
pub async fn midi_task(mut class: USBMidiClass) -> ! {
    class.wait_connection().await;
    let mut buf = [0; 64];
    loop {
        if let data = CHANNEL.receive().await {
            buf[0] = data[0] >> 4;
            buf[1..4].copy_from_slice(&data);
            class.write_packet(&buf[..4]).await;
        }
    }
}

impl Stm32Backend {
    pub async fn new() -> Self {
        let mut config = Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hse = Some(Hse {
                freq: Hertz(8_000_000),
                // Oscillator for bluepill, Bypass for nucleos.
                mode: HseMode::Oscillator,
            });
            config.rcc.pll = Some(Pll {
                src: PllSource::HSE,
                prediv: PllPreDiv::DIV1,
                mul: PllMul::MUL9,
            });
            config.rcc.sys = Sysclk::PLL1_P;
            config.rcc.ahb_pre = AHBPrescaler::DIV1;
            config.rcc.apb1_pre = APBPrescaler::DIV2;
            config.rcc.apb2_pre = APBPrescaler::DIV1;
        }
        let mut p = embassy_stm32::init(config);

        {
            // BluePill board has a pull-up resistor on the D+ line.
            // Pull the D+ pin down to send a RESET condition to the USB bus.
            // This forced reset is needed only for development, without it host
            // will not reset your device when you upload new firmware.
            let _dp = Output::new(&mut p.PA12, Level::Low, Speed::Low);
            Timer::after_millis(10).await;
        }
        // Create the driver, from the HAL.
        let driver = Driver::new(p.USB, USBIrqs, p.PA12, p.PA11);

        // Create embassy-usb Config
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
        config.manufacturer = Some("reset-ctrl");
        config.product = Some("reset-ctrl PoC");
        config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = 64;

        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;

        static DEVICE_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
        static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

        let device_descriptor: &'static mut [u8; 256] = DEVICE_DESCRIPTOR.init([0; 256]);
        let config_descriptor: &'static mut [u8; 256] = CONFIG_DESCRIPTOR.init([0; 256]);
        let bos_descriptor: &'static mut [u8; 256] = BOS_DESCRIPTOR.init([0; 256]);
        let control_buf: &'static mut [u8; 64] = CONTROL_BUF.init([0; 64]);

        let mut builder = Builder::new(
            driver,
            config,
            device_descriptor,
            config_descriptor,
            bos_descriptor,
            &mut [], // no msos descriptors
            control_buf,
        );

        let mut class = MidiClass::new(&mut builder, 1, 1, 64);

        // default = MODE_0(CPOL = 0, CPHA = 0)
        let mut spi_config = SpiConfig::default();
        spi_config.frequency = Hertz(1_000_000);

        let mut spi = Spi::new_txonly(p.SPI1, p.PA5, p.PA7, NoDma, NoDma, spi_config);

        Self {
            addr: 0,
            out_a: Output::new(p.PA0, Level::Low, Speed::Low),
            out_b: Output::new(p.PA1, Level::Low, Speed::Low),
            out_c: Output::new(p.PA2, Level::Low, Speed::Low),
            flex_com: Flex::new(p.PA3),
            adc: Adc::new(p.ADC1, &mut Delay),
            adc_pin: p.PA4,
            usb_builder: Some(builder),
            usb_midi_class: Some(class),
            spi: spi,
            rclk: Output::new(p.PB0, Level::Low, Speed::Low),
        }
    }

    pub fn spawn_usb(&mut self, spawner: Spawner) {
        if let Some(b) = self.usb_builder.take() {
            spawner.spawn(usb_task(b.build())).unwrap();
        }
    }

    pub fn spawn_midi(&mut self, spawner: Spawner) {
        if let Some(m) = self.usb_midi_class.take() {
            spawner.spawn(midi_task(m)).unwrap();
        }
    }

    fn next(&mut self) {
        self.addr = self.addr + 1;
    }

    fn set_addr(&mut self) {
        let mut buf = [0u8; 1];
        buf[0] = self.addr;
        self.spi.blocking_write(&buf);
        self.rclk.set_high();
        self.rclk.set_low();
    }
}
