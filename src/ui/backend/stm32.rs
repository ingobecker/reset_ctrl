use embassy_stm32::gpio::{Flex, Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals;

use crate::ui::Backend;

pub struct Stm32Backend {
    addr: u8,
    out_a: Output<'static>,
    out_b: Output<'static>,
    out_c: Output<'static>,
    flex_com: Flex<'static>,
}

impl Backend for Stm32Backend {
    fn read_adc(&mut self) -> u16 {
        0 as u16
    }

    fn read_input(&mut self) -> bool {
        self.flex_com.set_as_input(Pull::Up);
        let level = self.flex_com.is_high();
        self.next();
        level
    }
}

impl Stm32Backend {
    pub fn new(
        out_a: peripherals::PA0,
        out_b: peripherals::PA1,
        out_c: peripherals::PA2,
        flex_com: peripherals::PA3,
    ) -> Self {
        Self {
            addr: 0,
            out_a: Output::new(out_a, Level::Low, Speed::Low),
            out_b: Output::new(out_b, Level::Low, Speed::Low),
            out_c: Output::new(out_c, Level::Low, Speed::Low),
            flex_com: Flex::new(flex_com),
        }
    }

    fn next(&mut self) {
        self.addr = (self.addr + 1) & 1; // Limit to two inputs, use & 0b111; later
        self.set_addr();
    }

    fn set_addr(&mut self) {
        let mut addr_tmp = self.addr;

        if (addr_tmp & 1) == 1 {
            self.out_a.set_high();
        } else {
            self.out_a.set_low();
        }
        addr_tmp >>= 1;

        if (addr_tmp & 1) == 1 {
            self.out_b.set_high();
        } else {
            self.out_b.set_low();
        }
        addr_tmp >>= 1;

        if (addr_tmp & 1) == 1 {
            self.out_c.set_high();
        } else {
            self.out_c.set_low();
        }
        addr_tmp >>= 1;
    }
}
