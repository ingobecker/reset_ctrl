const MIDI_MSG_STATUS_CC: u8 = 0b1011u8 << 4;
const MIDI_MSG_STATUS_CHANNEL_MASK: u8 = 0xf;
const MIDI_MSG_CC_VAL_MASK: u8 = !(1 << 7);
const MIDI_MSG_CC_CTRL_MASK: u8 = !(1 << 7);

const MIDI_MSG_STATUS_NOTE_ON: u8 = 0b1001u8 << 4;
const MIDI_MSG_STATUS_NOTE_OFF: u8 = 0b1000u8 << 4;
const MIDI_MSG_NOTE_VEL_MASK: u8 = !(1 << 7);
const MIDI_MSG_NOTE_KEY_MASK: u8 = !(1 << 7);

pub struct MidiOut {}

#[derive(Eq, PartialEq, Debug)]
pub struct MidiMsgCc {
    pub channel: u8,
    pub control: u8,
    pub value: u8,
}

impl MidiMsgCc {
    pub fn to_bytes(&self) -> [u8; 3] {
        [
            MIDI_MSG_STATUS_CC | (MIDI_MSG_STATUS_CHANNEL_MASK & self.channel),
            MIDI_MSG_CC_CTRL_MASK & self.control,
            MIDI_MSG_CC_VAL_MASK & self.value,
        ]
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct MidiMsgNote {
    pub channel: u8,
    pub key: u8,
    pub on: bool,
    pub velocity: u8,
}

impl MidiMsgNote {
    pub fn to_bytes(&self) -> [u8; 3] {
        let status_note_on: u8 = if self.on {
            MIDI_MSG_STATUS_NOTE_ON
        } else {
            MIDI_MSG_STATUS_NOTE_OFF
        };

        [
            status_note_on | (MIDI_MSG_STATUS_CHANNEL_MASK & self.channel),
            MIDI_MSG_NOTE_KEY_MASK & self.key,
            MIDI_MSG_NOTE_VEL_MASK & self.velocity,
        ]
    }
}
