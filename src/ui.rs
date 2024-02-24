pub mod input {
    mod encoder;
    mod potentiometer;
    pub use self::{encoder::Encoder, encoder::EncoderDirection, potentiometer::Potentiometer};
}

pub mod backend {
    mod memory;
    pub use self::memory::InMemoryBackend;
}

use crate::ui::input::Encoder;

use serde::{Deserialize, Serialize};

pub trait Input {
    /*
     * Updates its state using the given HAL backend.
     * If its state changed, returns true otherwise false.
     */
    fn update(&mut self, backend: &mut impl Backend) -> bool;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Encoder(Encoder),
    //Potentiometer(Potentiometer),
}

pub trait Backend {
    fn read_adc(&mut self) -> u16;
    fn read_input(&mut self) -> bool;
}
