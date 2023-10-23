use log::error;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;


#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub addr: String,
}

impl RuntimeConfig {
    pub fn from_toml<T: AsRef<Path>>(path: T) -> Option<Self> {
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                error!("Something went wrong reading the runtime config file.");
                return None;
            }
        };
        let config: RuntimeConfig = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(_) => {
                error!("Something went wrong reading the runtime config file.");
                return None;
            }
        };
        Some(config)
    }
}
