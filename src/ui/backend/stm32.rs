use embassy_stm32::gpio::{Input, Level, Pull, Speed};
use embassy_stm32::peripherals;

use crate::ui::Backend;

pub struct Stm32Backend {
    pingpong: bool,
    in_a: Input<'static>,
    in_b: Input<'static>,
}

impl Backend for Stm32Backend {
    fn read_adc(&mut self) -> u16 {
        0 as u16
    }

    fn read_input(&mut self) -> bool {
        if self.pingpong {
            self.pingpong = false;
            return self.in_a.is_high();
        } else {
            self.pingpong = true;
            return self.in_b.is_high();
        }
    }
}

impl Stm32Backend {
    pub fn new(pin_a: peripherals::PA0, pin_b: peripherals::PA1) -> Self {
        Self {
            pingpong: true,
            in_a: Input::new(pin_a, Pull::Up),
            in_b: Input::new(pin_b, Pull::Up),
        }
    }
}
