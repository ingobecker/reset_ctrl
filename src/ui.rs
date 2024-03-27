pub mod input {
    mod encoder;
    mod potentiometer;
    pub use self::{encoder::Encoder, encoder::EncoderDirection, potentiometer::Potentiometer};
}

pub mod backend {
    mod memory;
    #[cfg(target_os = "none")]
    mod stm32;
    pub use self::memory::InMemoryBackend;

    #[cfg(target_os = "none")]
    pub use self::stm32::Stm32Backend;
}

use crate::ui::input::Encoder;
use crate::ui::input::Potentiometer;

use serde::{Deserialize, Serialize};

pub trait Input {
    /*
     * Updates its state using the given HAL backend.
     * If its state changed, returns true otherwise false.
     */
    async fn update(&mut self, backend: &mut impl Backend) -> bool;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputType {
    Encoder(Encoder),
    Potentiometer(Potentiometer),
}

pub trait Backend {
    async fn read_adc(&mut self) -> u16;
    fn read_input(&mut self) -> bool;
}
