use serde::Deserialize;
use std::error::Error;
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

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let json_file_path = Path::new(path);
    let file = File::open(json_file_path)?;
    let config: Config = serde_json::from_reader(file)?;

    Ok(config)
}
