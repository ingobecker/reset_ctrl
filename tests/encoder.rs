use reset_ctrl::device::Device;
use reset_ctrl::handler::{EncoderHandler, MidiRel};
use reset_ctrl::output::{MidiMsgCc, OutputData, OutputType, StdOut};
use reset_ctrl::ui::backend::InMemoryBackend;
use reset_ctrl::ui::input::{Encoder, EncoderDirection};
use reset_ctrl::ui::{Input, InputType};

use heapless::Vec;
use serde::{Deserialize, Serialize};

#[async_std::test]
async fn cw_to_midi() {
    let data_cw = [false, false, true, false, true, true];
    let mut b = InMemoryBackend::new();
    b.set_input_buffer(&data_cw);

    let mut encoder = Encoder::new();
    encoder.init(&mut b).await;
    let mut handler = EncoderHandler::MidiRel(MidiRel {
        channel: 0,
        control: 4,
    });
    encoder.attach_handler(handler);
    assert!(encoder.update(&mut b).await);
    assert_eq!(encoder.value(), EncoderDirection::CW);

    let expected_output = MidiMsgCc {
        channel: 0,
        control: 4,
        value: 63,
    };
    let handler_output = encoder.run_handler();

    if let OutputData::MidiMsgCc(m) = &handler_output {
        assert_eq!(m, &expected_output);
    }
}

#[test]
fn device() {
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

    //let yaml = serde_yaml::to_string(&device).expect("Unable to serialize");
}

#[test]
fn load_from_config() {
    let yaml = "
        inputs:
        - !Encoder
          handler: !MidiRel
            channel: 0
            control: 4
    ";

    let mut device = Device::from_config(&yaml);
    let data_cw = [false, false, true, false, true, true];

    let mut b = InMemoryBackend::new();
    b.set_input_buffer(&data_cw);

    let mut outputs: Vec<OutputType, 1> = Vec::new();
    outputs.push(OutputType::StdOut(StdOut {}));

    // setup
    device.init_inputs(&mut b);

    // operation
    device.update(&mut b);
    device.run_handler(&outputs);
}
