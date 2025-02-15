use crate::backends::Backend;
use std::time;
pub struct Demo {
    memory_samples: Vec<(u64, f64)>,
}

impl Demo {
    pub fn new() -> Self {
        Self {
            memory_samples: Vec::new(),
        }
    }
}

impl Backend for Demo {
    fn get_text(&self) -> String {
        "".into()
    }

    fn get_memory_usage(&self) -> &[(u64, f64)] {
        &[(3, 0.1), (4, 0.2), (5, 0.3), (6, 0.4)]
    }

    fn update_memory_samples(&mut self) {}
}
