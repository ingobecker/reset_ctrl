use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

use crate::output::OutputData;

pub static CHANNEL: Channel<ThreadModeRawMutex, u8, 4> = Channel::new();

pub struct UsbOut {}

#[cfg(target_os = "none")]
impl UsbOut {
    pub async fn run(&self, data: &OutputData) {
        match data {
            OutputData::MidiMsgCc(m) => CHANNEL.send(m.value).await,
            OutputData::MidiMsgNote(m) => CHANNEL.send(m.velocity).await,
            _ => CHANNEL.send(0).await,
        }
    }
}
