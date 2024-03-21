mod midi;
mod stdout;
#[cfg(target_os = "none")]
mod usb;

pub use self::{midi::MidiMsgCc, midi::MidiMsgNote, midi::MidiOut, stdout::StdOut};
#[cfg(target_os = "none")]
pub use self::{usb::UsbOut, usb::CHANNEL};

pub enum OutputData {
    MidiMsgCc(MidiMsgCc),
    MidiMsgNote(MidiMsgNote),
    Dummy,
}

pub enum OutputType {
    StdOut(StdOut),
    MidiOut(MidiOut),
    #[cfg(target_os = "none")]
    UsbOut(UsbOut),
}
