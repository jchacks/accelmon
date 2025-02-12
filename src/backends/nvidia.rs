use nvml_wrapper::Nvml;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

use super::{Sample, SampleValue};

impl From<nvml_wrapper::enums::device::SampleValue> for SampleValue {
    fn from(value: nvml_wrapper::enums::device::SampleValue) -> Self {
        match value {
            nvml_wrapper::enums::device::SampleValue::F64(x) => Self::F64(x),
            nvml_wrapper::enums::device::SampleValue::U32(x) => Self::U32(x),
            nvml_wrapper::enums::device::SampleValue::U64(x) => Self::U64(x),
            nvml_wrapper::enums::device::SampleValue::I64(x) => Self::I64(x),
        }
    }
}

impl From<nvml_wrapper::struct_wrappers::device::Sample> for Sample {
    fn from(value: nvml_wrapper::struct_wrappers::device::Sample) -> Self {
        Self {
            timestamp: value.timestamp,
            value: value.value.into(),
        }
    }
}

pub struct Nvidia {
    nvml: Nvml,
    pub memory_samples: Vec<Sample>,
}

impl Nvidia {
    pub fn new() -> Self {
        let nvml = Nvml::init().expect("Failed to init nvml");
        let device_count = nvml.device_count().expect("Failed to get device count");
        Self {
            nvml,
            memory_samples: Vec::new(),
        }
    }

    pub fn update_memory_samples(&mut self) {
        let device = self.nvml.device_by_index(0).expect("Failed to get device");
        let mut new_samples: Vec<Sample> = device
            .samples(
                nvml_wrapper::enum_wrappers::device::Sampling::MemoryUtilization,
                self.memory_samples.last().map(|x| x.timestamp),
            )
            .expect("Failed to sample memory usage")
            .into_iter()
            .map(Sample::from)
            .collect();
        self.memory_samples.append(&mut new_samples);
    }

    pub fn get_text(&self) -> Result<String, nvml_wrapper::error::NvmlError> {
        let nvml = &self.nvml;
        // Get the first `Device` (GPU) in the system
        let device = nvml.device_by_index(0)?;

        let brand = device.brand()?;
        let arch = device.architecture()?;
        let fan_speed = device.fan_speed(0)?;
        let power_limit = device.enforced_power_limit()?;
        let encoder_util = device.encoder_utilization()?;
        let memory_info = device.memory_info()?;

        Ok(format!(
        "{brand:?} {arch:?} fan={fan_speed:?} {power_limit:?}mW {encoder_util:?} {memory_info:?}B"
    ))
    }
}
