use crate::handler::EncoderHandler;
use crate::output::OutputData;
use crate::ui::Backend;
use crate::ui::Input;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Encoder {
    #[serde(skip)]
    pattern: u8,
    pub handler: EncoderHandler,
}

impl Input for Encoder {
    fn update(&mut self, backend: &mut impl Backend) -> bool {
        let pattern = self.read(backend);
        if (self.pattern & 0b11u8) == pattern {
            return false;
        }

        self.pattern <<= 2;
        self.pattern &= 0xfu8;
        self.pattern |= pattern;

        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EncoderDirection {
    CW,
    CCW,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            pattern: 0,
            handler: EncoderHandler::Dummy,
        }
    }

    pub fn init(&mut self, backend: &mut impl Backend) {
        self.read(backend);
    }

    pub fn attach_handler(&mut self, handler: EncoderHandler) {
        self.handler = handler;
    }

    pub fn run_handler(&mut self) -> OutputData {
        let v = self.value();
        match &mut self.handler {
            EncoderHandler::MidiRel(h) => h.run(v),
            EncoderHandler::MidiAbs(h) => h.run(v),
            EncoderHandler::Dummy => OutputData::Dummy,
        }
    }

    fn read(&self, backend: &mut impl Backend) -> u8 {
        let a: u8 = if backend.read_input() { 1 } else { 0 };
        let b: u8 = if backend.read_input() { 1 } else { 0 };

        (a << 1) | b
    }

    pub fn value(&self) -> EncoderDirection {
        match self.pattern {
            0b1101u8 | 0b0100u8 | 0b0010u8 | 0b1011u8 => EncoderDirection::CW,
            0b1110u8 | 0b0111u8 | 0b0001u8 | 0b1000u8 => EncoderDirection::CCW,
            _ => EncoderDirection::CW, // should never happen, maybe panic?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::ui::backend::Backend;
    use crate::ui::backend::InMemoryBackend;

    #[test]
    fn encoder_turn_cw() {
        let data_cw = [
            false, false,
            /*
             * A: 1100
             * B: 0110
             *    ⮍
             */
            true, false, true, true,
            /*
             * A: 1100
             * B: 0110
             *     ⮍
             */
            false, true,
            /*
             * A: 1100
             * B: 0110
             *      ⮍
             */
            false, false,
            /*
             * A: 11001
             * B: 01100
             *       ⮍
             */
            true, false,
        ];

        let mut b = InMemoryBackend::new();
        b.set_input_buffer(&data_cw);

        let mut encoder = Encoder::new();
        encoder.init(&mut b);

        for _ in 0..5 {
            assert!(encoder.update(&mut b));
            assert_eq!(encoder.value(), EncoderDirection::CW);
        }
    }

    #[test]
    fn encoder_turn_ccw() {
        let data_cw = [
            false, false,
            /*
             *      ⮏
             * A: 1100
             * B: 0110
             */
            false, true,
            /*
             *     ⮏
             * A: 1100
             * B: 0110
             */
            true, true,
            /*
             *    ⮏
             * A: 1100
             * B: 0110
             */
            true, false,
            /*
             *       ⮏
             * A: 11001
             * B: 01100
             */
            false, false,
        ];

        let mut b = InMemoryBackend::new();
        b.set_input_buffer(&data_cw);

        let mut encoder = Encoder::new();
        encoder.init(&mut b);

        for _ in 0..4 {
            assert!(encoder.update(&mut b));
            assert_eq!(encoder.value(), EncoderDirection::CCW);
        }
    }

    #[test]
    fn encoder_turn_back_n_forth() {
        let data_cw = [
            false, false,
            /*
             *      ⮏
             * A: 1100
             * B: 0110
             */
            false, true,
            /*
             *     ⮏
             * A: 1100
             * B: 0110
             */
            false, false,
            /*
             * A: 1100
             * B: 0110
             *      ⮍
             */
            true, false,
        ];

        let mut b = InMemoryBackend::new();
        b.set_input_buffer(&data_cw);

        let mut encoder = Encoder::new();
        encoder.init(&mut b);

        assert!(encoder.update(&mut b));
        assert_eq!(encoder.value(), EncoderDirection::CCW);

        assert!(encoder.update(&mut b));
        assert_eq!(encoder.value(), EncoderDirection::CW);
    }

    #[test]
    fn encoder_no_turn() {
        let data_cw = [
            /*
             * A: 0110
             * B: 0011
             *    ⮍
             */
            false, false, true, false, true, false,
        ];

        let mut b = InMemoryBackend::new();
        b.set_input_buffer(&data_cw);

        let mut encoder = Encoder::new();
        encoder.init(&mut b);

        assert!(encoder.update(&mut b));
        assert_eq!(encoder.value(), EncoderDirection::CW);
        assert!(!encoder.update(&mut b), "Returned update");
        assert_eq!(encoder.value(), EncoderDirection::CW);
    }
}
