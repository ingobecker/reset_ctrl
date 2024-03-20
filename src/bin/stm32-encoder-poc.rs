#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input as GpioInput, Level, Pull, Speed};
use embassy_stm32::Config;
use embassy_time::Timer;
use heapless::Vec;

use reset_ctrl::device::Device;
use reset_ctrl::handler::{EncoderHandler, MidiAbs};
use reset_ctrl::output::{MidiMsgCc, OutputData, OutputType, StdOut};
use reset_ctrl::ui::backend::Stm32Backend;
use reset_ctrl::ui::input::{Encoder, EncoderDirection};
use reset_ctrl::ui::Backend;
use reset_ctrl::ui::{Input, InputType};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let mut b = Stm32Backend::new(p.PA0, p.PA1);

    let mut encoder = Encoder::new();
    let mut handler = EncoderHandler::MidiAbs(MidiAbs {
        channel: 0,
        control: 4,
        value: 0,
    });
    encoder.attach_handler(handler);

    let mut input = InputType::Encoder(encoder);
    let mut device = Device::new();

    let mut outputs: Vec<OutputType, 1> = Vec::new();
    outputs.push(OutputType::StdOut(StdOut {}));

    // setup
    device.add_input(input);
    device.init_inputs(&mut b);

    // operation
    device.update(&mut b);
    device.run_handler(&outputs);

    info!("Starting update loop");
    loop {
        Timer::after_millis(1).await;

        device.update(&mut b);
        device.run_handler(&outputs);
    }
}
