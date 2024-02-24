mod midi;
mod stdout;
pub use self::{midi::MidiMsgCc, midi::MidiMsgNote, midi::MidiOut, stdout::StdOut};

pub enum OutputData {
    MidiMsgCc(MidiMsgCc),
    MidiMsgNote(MidiMsgNote),
    Dummy,
}

pub enum OutputType {
    StdOut(StdOut),
    MidiOut(MidiOut),
}
