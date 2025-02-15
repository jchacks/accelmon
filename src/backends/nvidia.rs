use nvml_wrapper::{enums::device::SampleValue, Nvml};

use super::Backend;

pub struct Nvidia<'a> {
    nvml_device: nvml_wrapper::Device<'a>,
    pub memory_samples: Vec<(u64, f64)>,
}

impl<'a> Nvidia<'a> {
    pub fn new(nvml: &'a Nvml) -> Self {
        let device_count = nvml.device_count().expect("Failed to get device count");
        let nvml_device: nvml_wrapper::Device<'a> =
            nvml.device_by_index(0).expect("Failed to get NVML device");
        Self {
            nvml_device,
            memory_samples: Vec::new(),
        }
    }
}

fn extract_value(value: SampleValue) -> f64 {
    match value {
        SampleValue::F64(x) => x as f64,
        SampleValue::U32(x) => x as f64,
        SampleValue::U64(x) => x as f64,
        SampleValue::I64(x) => x as f64,
    }
}

impl Backend for Nvidia<'_> {
    fn get_text(&self) -> String {
        let name = self
            .nvml_device
            .name()
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let brand = self
            .nvml_device
            .brand()
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let arch = self
            .nvml_device
            .architecture()
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let fan_speed = self
            .nvml_device
            .fan_speed(0)
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let power_limit = self
            .nvml_device
            .enforced_power_limit()
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let encoder_util = self
            .nvml_device
            .encoder_utilization()
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));
        let memory_info = self
            .nvml_device
            .memory_info()
            .map(|x| format!("{:?}", x))
            .unwrap_or_else(|err| format!("Unknown - {:}", err));

        format!(
            "{name:?} {brand:?} {arch:?} fan={fan_speed:?} {power_limit:?}mW {encoder_util:?} {memory_info:?}B"
        )
    }
    fn update_memory_samples(&mut self) {
        let mut new_samples: Vec<(u64, f64)> = self
            .nvml_device
            .samples(
                nvml_wrapper::enum_wrappers::device::Sampling::MemoryUtilization,
                self.memory_samples.last().map(|x| x.0),
            )
            .expect("Failed to sample memory usage")
            .into_iter()
            .map(|x| (x.timestamp, extract_value(x.value)))
            .collect();

        self.memory_samples.append(&mut new_samples);
    }
    fn get_memory_usage(&self) -> &[(u64, f64)] {
        &self.memory_samples
    }
}
