use crate::output::{MidiMsgCc, OutputData};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PotentiometerHandler {
    Dummy,
    // MidiRel(MidiRel),
    MidiAbs(MidiAbs),
    //MidiNote(MidiNote),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiAbs {
    pub channel: u8,
    pub control: u8,
    pub value: u8,
}

impl MidiAbs {
    pub fn run(&mut self, v: u8) -> OutputData {
        self.value = v;

        OutputData::MidiMsgCc(MidiMsgCc {
            channel: self.channel,
            control: self.control,
            value: self.value,
        })
    }
}
