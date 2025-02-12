mod nvidia;
use crate::backends::nvidia::Nvidia;

#[derive(Debug)]
pub enum SampleValue {
    F64(f64),
    U32(u32),
    U64(u64),
    I64(i64),
}

#[derive(Debug)]
pub struct Sample {
    /// CPU timestamp in μs
    pub timestamp: u64,
    pub value: SampleValue,
}

pub enum Backend {
    Nvidia(Box<Nvidia>),
    None,
}

impl Backend {
    pub fn nvidia() -> Self {
        Self::Nvidia(Box::new(Nvidia::new()))
    }

    pub fn get_text(&self) -> String {
        match self {
            Self::Nvidia(nvidia) => nvidia.get_text().unwrap_or("Error".to_string()),
            Self::None => "Error".to_string(),
        }
    }

    pub fn update_memory_samples(&mut self) {
        match self {
            Self::Nvidia(nvidia) => nvidia.update_memory_samples(),
            Self::None => (),
        }
    }
    pub fn get_memory_samples(&self) -> &[Sample] {
        match self {
            Self::Nvidia(nvidia) => &nvidia.memory_samples,
            Self::None => &[],
        }
    }
}
