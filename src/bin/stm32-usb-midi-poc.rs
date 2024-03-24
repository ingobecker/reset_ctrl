#![no_std]
#![no_main]

use defmt::{panic, *};
use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::{Driver, Instance};
use embassy_stm32::{bind_interrupts, peripherals, usb, Config};
use embassy_time::Timer;
use embassy_usb::class::midi::MidiClass;
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;

use heapless::Vec;

use reset_ctrl::device::Device;
use reset_ctrl::handler::{EncoderHandler, MidiAbs};
use reset_ctrl::output::{MidiMsgCc, OutputData, OutputType, StdOut, UsbOut, CHANNEL};
use reset_ctrl::ui::backend::Stm32Backend;
use reset_ctrl::ui::input::{Encoder, EncoderDirection};
use reset_ctrl::ui::Backend;
use reset_ctrl::ui::{Input, InputType};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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

    info!("Hello World!");

    {
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let _dp = Output::new(&mut p.PA12, Level::Low, Speed::Low);
        Timer::after_millis(10).await;
    }

    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);

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

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no msos descriptors
        &mut control_buf,
    );

    // Create classes on the builder.
    let mut class = MidiClass::new(&mut builder, 1, 1, 64);

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    // reset_ctrl setup
    let mut b = Stm32Backend::new(p.PA0, p.PA1, p.PA2, p.PA3);

    let mut encoder = Encoder::new();
    let mut handler = EncoderHandler::MidiAbs(MidiAbs {
        channel: 0,
        control: 4,
        value: 0,
    });
    encoder.attach_handler(handler);

    let mut input = InputType::Encoder(encoder);
    let mut device = Device::new();

    let mut outputs: Vec<OutputType, 2> = Vec::new();
    outputs.push(OutputType::StdOut(StdOut {}));
    outputs.push(OutputType::UsbOut(UsbOut {}));

    // setup
    device.add_input(input);
    device.init_inputs(&mut b);

    // operation
    device.update(&mut b);
    device.run_handler(&outputs);

    info!("Starting update loop");
    let reset_ctrl_fut = async {
        loop {
            Timer::after_millis(1).await;

            device.update(&mut b);
            device.run_handler(&outputs).await;
        }
    };

    // Do stuff with the class!
    let echo_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            let _ = echo(&mut class).await;
            info!("Disconnected");
        }
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join3(usb_fut, echo_fut, reset_ctrl_fut).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn echo<'d, T: Instance + 'd>(
    class: &mut MidiClass<'d, Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        if let data = CHANNEL.receive().await {
            buf[0] = data[0] >> 4;
            buf[1..4].copy_from_slice(&data);
            //info!("usb CHANNEL received: {:x}", data);
            //info!("usb class.write_packet {:x}", &buf[..4]);
            class.write_packet(&buf[..4]).await?;
        }
    }
}
