use crate::ui::Backend;
use crate::ui::Input;

pub struct Potentiometer {
    value: u8,
}

impl Input for Potentiometer {
    fn update(&mut self, backend: &mut impl Backend) -> bool {
        let data = (backend.read_adc() >> 5) as u8;
        if self.value != data {
            self.value = data;
            return true;
        }
        false
    }
}

impl Potentiometer {
    fn value(&self) -> u8 {
        self.value
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
