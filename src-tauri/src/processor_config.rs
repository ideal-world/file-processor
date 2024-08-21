use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub const DOMAIN_CODE: &str = "processor";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ProcessorConfig {
    pub concurrent: usize,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        ProcessorConfig { concurrent: 5 }
    }
}
