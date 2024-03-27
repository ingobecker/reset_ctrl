use crate::ui::Backend;

const IN_MEMORY_BACKEND_ADC_SIZE: usize = 4;
const IN_MEMORY_BACKEND_INPUT_SIZE: usize = 16;

pub struct InMemoryBackend {
    adc_buffer: [u16; IN_MEMORY_BACKEND_ADC_SIZE],
    adc_buffer_pos: usize,
    input_buffer: [bool; IN_MEMORY_BACKEND_INPUT_SIZE],
    input_buffer_pos: usize,
}

impl Backend for InMemoryBackend {
    async fn read_adc(&mut self) -> u16 {
        let result = self.adc_buffer.get(self.adc_buffer_pos);
        self.adc_buffer_pos += 1;

        match result {
            Some(data) => *data,
            None => {
                self.adc_buffer_pos = 1;
                *self
                    .adc_buffer
                    .get(0)
                    .expect("should have at least one value")
            }
        }
    }

    fn read_input(&mut self) -> bool {
        let result = self.input_buffer.get(self.input_buffer_pos);
        self.input_buffer_pos += 1;

        match result {
            Some(data) => *data,
            None => {
                self.input_buffer_pos = 1;
                *self
                    .input_buffer
                    .get(0)
                    .expect("should have at least one value")
            }
        }
    }
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            adc_buffer: [0; IN_MEMORY_BACKEND_ADC_SIZE],
            adc_buffer_pos: 0,
            input_buffer: [false; IN_MEMORY_BACKEND_INPUT_SIZE],
            input_buffer_pos: 0,
        }
    }

    pub fn set_adc_buffer(&mut self, data: &[u16]) {
        if data.len() > self.adc_buffer.len() {
            panic!("Maximum data size: {IN_MEMORY_BACKEND_ADC_SIZE}");
        }
        for (i, v) in data.iter().enumerate() {
            self.adc_buffer[i] = *v;
        }
    }

    pub fn set_input_buffer(&mut self, data: &[bool]) {
        if data.len() > self.input_buffer.len() {
            panic!("Maximum data size: {IN_MEMORY_BACKEND_INPUT_SIZE}");
        }
        for (i, v) in data.iter().enumerate() {
            self.input_buffer[i] = *v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[async_std::test]
    async fn read_adc_wraps_around() {
        let mut b = InMemoryBackend::new();
        let data: [u16; 4] = [42, 23, 5, 1];

        b.set_adc_buffer(&data);

        let expected: [u16; 6] = [42, 23, 5, 1, 42, 23];
        let mut actual: [u16; 6] = [0; 6];

        for v in &mut actual {
            *v = b.read_adc().await;
        }

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "needs changes of buffersize"]
    fn read_input_wraps_around() {
        let mut b = InMemoryBackend::new();
        let data: [bool; 4] = [true, false, false, true];

        b.set_input_buffer(&data);

        let expected: [bool; 6] = [true, false, false, true, true, false];
        let mut actual: [bool; 6] = [false; 6];

        for v in &mut actual {
            *v = b.read_input();
        }

        assert_eq!(expected, actual);
    }
}
