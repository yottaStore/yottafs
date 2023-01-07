use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub path: String,
    pub start_sector: Option<u64>,
    pub end_sector: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub devices: Vec<Device>,
}

#[derive(Debug)]
pub struct ConfigError {
    pub message: String,
}

pub fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let json_file_path = Path::new(path);

    let file = match File::open(json_file_path) {
        Ok(file) => file,
        Err(err) => {
            return Err(ConfigError {
                message: format!("Error opening config file: {}", err),
            });
        }
    };

    let config = match serde_json::from_reader(file) {
        Ok(config) => config,
        Err(err) => {
            return Err(ConfigError {
                message: format!("Error parsing config file: {}", err),
            });
        }
    };

    Ok(config)
}
