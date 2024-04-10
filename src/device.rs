use crate::output::{OutputData, OutputType};
use crate::ui::backend::InMemoryBackend;
use crate::ui::Backend;
use crate::ui::{Input, InputType};

use heapless::Vec;
use serde::{Deserialize, Serialize};

const DEVICE_INPUTS_MAX: usize = 2;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    inputs: Vec<InputType, DEVICE_INPUTS_MAX>,
    #[serde(skip)]
    updated: Vec<usize, 4>,
}

impl Device {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            updated: Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn from_config(config: &str) -> Self {
        let mut device: Self = serde_yaml::from_str(config).unwrap();
        device
    }

    pub fn add_input(&mut self, input: InputType) -> Result<(), InputType> {
        self.inputs.push(input)
    }

    pub async fn init_inputs(&mut self, backend: &mut impl Backend) {
        for (idx, input) in self.inputs.iter_mut().enumerate() {
            match input {
                InputType::Encoder(i) => i.init(backend).await,
                InputType::Potentiometer(i) => i.init(backend).await,
                _ => (),
            };
        }
        backend.rewind();
    }

    pub async fn update(&mut self, backend: &mut impl Backend) {
        for (idx, input) in self.inputs.iter_mut().enumerate() {
            let was_updated = match input {
                InputType::Encoder(i) => i.update(backend).await,
                InputType::Potentiometer(i) => i.update(backend).await,
            };

            if was_updated {
                self.updated.push(idx);
            }
        }
        backend.rewind();
    }

    pub async fn run_handler(&mut self, outputs: &[OutputType]) {
        while let Some(i) = self.updated.pop() {
            let input = self
                .inputs
                .get_mut(i)
                .expect("Can't dispatch non existing input");
            let output_data = match input {
                InputType::Encoder(i) => i.run_handler(),
                InputType::Potentiometer(i) => i.run_handler(),
            };

            for ot in outputs {
                match ot {
                    OutputType::StdOut(o) => o.run(&output_data).await,
                    #[cfg(target_os = "none")]
                    OutputType::UsbOut(o) => o.run(&output_data).await,
                    _ => (),
                }
            }
        }
        self.updated.clear();
    }
}
