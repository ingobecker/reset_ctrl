use crate::output::{MidiMsgCc, OutputData};
use crate::ui::input::{Encoder, EncoderDirection};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EncoderHandler {
    Dummy,
    MidiRel(MidiRel),
    MidiAbs(MidiAbs),
    //MidiNote(MidiNote),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiRel {
    pub channel: u8,
    pub control: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiAbs {
    channel: u8,
    control: u8,
    value: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MidiNote {
    channel: u8,
    key: u8,
}

impl MidiRel {
    pub fn run(&mut self, ev: EncoderDirection) -> OutputData {
        let v = match ev {
            EncoderDirection::CW => 63,
            EncoderDirection::CCW => 65,
        };

        OutputData::MidiMsgCc(MidiMsgCc {
            channel: self.channel,
            control: self.control,
            value: v,
        })
    }
}

impl MidiAbs {
    pub fn run(&mut self, ev: EncoderDirection) -> OutputData {
        match ev {
            EncoderDirection::CW => self.inc(),
            EncoderDirection::CCW => self.dec(),
        }

        OutputData::MidiMsgCc(MidiMsgCc {
            channel: self.channel,
            control: self.control,
            value: self.value,
        })
    }

    fn inc(&mut self) {
        if self.value < 0x7f {
            self.value += 1;
        }
    }

    fn dec(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::input::EncoderDirection;

    #[test]
    fn hander_abs_cw() {
        let mut handler = MidiAbs {
            channel: 5,
            control: 23,
            value: 0,
        };

        for i in 1..3 {
            if let OutputData::MidiMsgCc(m) = handler.run(EncoderDirection::CW) {
                assert_eq!(m.channel, 5);
                assert_eq!(m.control, 23);
                assert_eq!(m.value, i);
            } else {
                panic!("Wrong output data returned");
            }
        }
    }

    #[test]
    fn hander_abs_cw_limit() {
        let mut handler = MidiAbs {
            channel: 5,
            control: 23,
            value: 127,
        };

        if let OutputData::MidiMsgCc(m) = handler.run(EncoderDirection::CW) {
            assert_eq!(m.value, 127);
        }
    }

    #[test]
    fn hander_abs_ccw_limit() {
        let mut handler = MidiAbs {
            channel: 5,
            control: 23,
            value: 0,
        };

        if let OutputData::MidiMsgCc(m) = handler.run(EncoderDirection::CCW) {
            assert_eq!(m.value, 0);
        }
    }

    #[test]
    fn hander_abs_ccw() {
        let mut handler = MidiAbs {
            channel: 5,
            control: 23,
            value: 3,
        };

        for i in 2..0 {
            if let OutputData::MidiMsgCc(m) = handler.run(EncoderDirection::CW) {
                assert_eq!(m.channel, 5);
                assert_eq!(m.control, 23);
                assert_eq!(m.value, i);
            } else {
                panic!("Wrong output data returned");
            }
        }
    }
}
