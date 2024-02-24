use crate::output::OutputData;

#[cfg(target_os = "none")]
use defmt::info;

pub struct StdOut {}

#[cfg(target_os = "linux")]
impl StdOut {
    pub fn run(&self, data: &OutputData) {
        match data {
            OutputData::MidiMsgCc(m) => println!(
                "[Midi CC| Channel: {}, Control: {}, value: {}]",
                m.channel, m.control, m.value
            ),
            OutputData::MidiMsgNote(m) => println!(
                "[Midi Note: Channel: {}, Key: {}, Velocity: {}]",
                m.channel, m.key, m.velocity
            ),
            _ => println!("Unknown output data"),
        }
    }
}

#[cfg(target_os = "none")]
impl StdOut {
    pub fn run(&self, data: &OutputData) {
        match data {
            OutputData::MidiMsgCc(m) => info!(
                "[Midi CC| Channel: {}, Control: {}, value: {}]",
                m.channel, m.control, m.value
            ),
            OutputData::MidiMsgNote(m) => info!(
                "[Midi Note: Channel: {}, Key: {}, Velocity: {}]",
                m.channel, m.key, m.velocity
            ),
            _ => info!("Unknown output data"),
        }
    }
}
