#![no_std]
#![no_main]

use defmt::{panic, *};
use embassy_executor::Spawner;
use embassy_time::Timer;

use heapless::Vec;

use reset_ctrl::device::Device;
use reset_ctrl::handler::{EncoderHandler, MidiAbs, PotMidiAbs, PotentiometerHandler};
use reset_ctrl::output::{MidiMsgCc, OutputData, OutputType, StdOut, UsbOut, CHANNEL};
use reset_ctrl::ui::backend::Stm32Backend;
use reset_ctrl::ui::input::{Encoder, EncoderDirection, Potentiometer};
use reset_ctrl::ui::Backend;
use reset_ctrl::ui::{Input, InputType};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // setup device
    // encoder
    let mut encoder = Encoder::new();
    let mut handler = EncoderHandler::MidiAbs(MidiAbs {
        channel: 0,
        control: 4,
        value: 0,
    });
    encoder.attach_handler(handler);
    let mut input = InputType::Encoder(encoder);

    // potentiometer
    let mut pot = Potentiometer::new();
    let mut pot_handler = PotentiometerHandler::MidiAbs(PotMidiAbs {
        channel: 5,
        control: 42,
        value: 23,
    });
    pot.attach_handler(pot_handler);
    let mut pot_input = InputType::Potentiometer(pot);

    let mut device = Device::new();
    let mut outputs: Vec<OutputType, 2> = Vec::new();
    outputs.push(OutputType::StdOut(StdOut {}));
    outputs.push(OutputType::UsbOut(UsbOut {}));

    // setup
    device.add_input(input);
    device.add_input(pot_input);

    info!("Setting up Stm32Backend...");
    // reset_ctrl setup
    let mut b = Stm32Backend::new().await;
    info!("Stm32Backend setup completed!");

    device.init_inputs(&mut b);

    b.spawn_usb(spawner);
    b.spawn_midi(spawner);

    // operation
    info!("Starting update loop");
    let reset_ctrl_fut = async {
        loop {
            Timer::after_micros(500).await;

            device.update(&mut b).await;
            device.run_handler(&outputs).await;
        }
    };

    reset_ctrl_fut.await;
}
