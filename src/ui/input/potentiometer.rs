use crate::handler::PotentiometerHandler;
use crate::output::OutputData;
use crate::ui::Backend;
use crate::ui::Input;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Potentiometer {
    #[serde(skip)]
    value: u8,
    pub handler: PotentiometerHandler,
}

impl Input for Potentiometer {
    async fn update(&mut self, backend: &mut impl Backend) -> bool {
        let data = backend.read_adc().await;
        let data = (data >> 5) as u8;
        if self.value != data {
            self.value = data;
            return true;
        }
        false
    }
}

impl Potentiometer {
    pub fn new() -> Self {
        Self {
            value: 0,
            handler: PotentiometerHandler::Dummy,
        }
    }

    fn value(&self) -> u8 {
        self.value
    }

    pub fn attach_handler(&mut self, handler: PotentiometerHandler) {
        self.handler = handler;
    }

    pub fn run_handler(&mut self) -> OutputData {
        let v = self.value();
        match &mut self.handler {
            PotentiometerHandler::MidiAbs(h) => h.run(v),
            PotentiometerHandler::Dummy => OutputData::Dummy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::backend::InMemoryBackend;

    #[test]
    fn potentiometer_low_definition() {
        let data: [u16; 1] = [1 << 12];
        let mut b = InMemoryBackend::new();
        b.set_adc_buffer(&data);

        let mut pot = Potentiometer { value: 0 };
        assert!(pot.update(&mut b));
        assert_eq!(pot.value(), 1 << 7);
    }
}
