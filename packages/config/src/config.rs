use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
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
    pub devices: [Option<Device>],
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let json_file_path = Path::new(path);
    let file = File::open(json_file_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    Ok(config)
}
