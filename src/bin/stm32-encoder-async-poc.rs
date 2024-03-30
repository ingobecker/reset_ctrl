#![no_std]
#![no_main]

use futures::future::join;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input as GpioInput, Level, Pull, Speed};
use embassy_stm32::Config;
use embassy_time::{with_timeout, Duration, Timer};
use heapless::Vec;

use reset_ctrl::device::Device;
use reset_ctrl::handler::{EncoderHandler, MidiAbs};
use reset_ctrl::output::{MidiMsgCc, OutputData, OutputType, StdOut, UsbOut, CHANNEL};
use reset_ctrl::ui::backend::Stm32Backend;
use reset_ctrl::ui::input::{Encoder, EncoderDirection};
use reset_ctrl::ui::Backend;
use reset_ctrl::ui::{Input, InputType};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let mut encoder = Encoder::new();
    let mut handler = EncoderHandler::MidiAbs(MidiAbs {
        channel: 0,
        control: 4,
        value: 0,
    });
    encoder.attach_handler(handler);

    let mut input = InputType::Encoder(encoder);
    let mut device = Device::new();

    let inputs = device.inputs();
    let mut b = Stm32Backend::new(inputs, p.PA0, p.PA1, p.PA2, p.PA3, p.ADC1, p.PA4);

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
    let reset_intput_processing = async {
        loop {
            Timer::after_millis(1).await;

            device.update(&mut b);
            device.run_handler(&outputs).await;
        }
    };

    let dummy_processing = async {
        loop {
            if let Ok(data) = with_timeout(Duration::from_secs(1), CHANNEL.receive()).await {
                info!("dummy processing, data: {}", data);
            } else {
                info!("dummy processing, no data");
            }
        }
    };

    join(reset_intput_processing, dummy_processing).await;
}
