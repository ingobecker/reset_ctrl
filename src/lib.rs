#![cfg_attr(target_os = "none", no_std)]

pub mod device;
pub mod output;
pub mod ui;

pub mod handler {

    mod encoder;
    pub use self::{encoder::EncoderHandler, encoder::MidiRel};
}

use device::Device;
use handler::{EncoderHandler, MidiRel};
use output::{MidiMsgCc, OutputData, OutputType, StdOut};
use ui::backend::InMemoryBackend;
use ui::input::{Encoder, EncoderDirection};
use ui::{Input, InputType};

use heapless::Vec;

// TODO: simple poc code to demonstrate that the code runs on
//       both targets
pub fn run() {
    let data_cw = [false, false, true, false, true, true];

    let mut b = InMemoryBackend::new();
    b.set_input_buffer(&data_cw);

    let mut encoder = Encoder::new();
    let mut handler = EncoderHandler::MidiRel(MidiRel {
        channel: 0,
        control: 4,
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
}
