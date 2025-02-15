mod demo;
mod nvidia;
use nvml_wrapper::Nvml;

use crate::backends::demo::Demo;
use crate::backends::nvidia::Nvidia;

pub trait Backend {
    fn get_text(&self) -> String;

    fn get_memory_usage(&self) -> &[(u64, f64)];
    fn update_memory_samples(&mut self);
}

pub enum Device<'a> {
    Nvidia(Box<Nvidia<'a>>),
    Demo(Box<Demo>),
    None,
}

impl<'a> Device<'a> {
    pub fn nvidia(nvml: &'a Nvml) -> Self {
        Self::Nvidia(Box::new(Nvidia::new(&nvml)))
    }

    pub fn demo() -> Self {
        Self::Demo(Box::new(Demo::new()))
    }

    pub fn get_text(&self) -> String {
        match self {
            Self::Nvidia(nvidia) => nvidia.get_text(),
            Self::Demo(demo) => demo.get_text(),
            Self::None => "Error".to_string(),
        }
    }

    pub fn update_memory_samples(&mut self) {
        match self {
            Self::Nvidia(nvidia) => nvidia.update_memory_samples(),
            Self::Demo(demo) => demo.update_memory_samples(),
            Self::None => (),
        }
    }
    pub fn get_memory_usage(&self) -> &[(u64, f64)] {
        match self {
            Self::Nvidia(nvidia) => &nvidia.get_memory_usage(),
            Self::Demo(demo) => demo.get_memory_usage(),
            Self::None => &[],
        }
    }
}
