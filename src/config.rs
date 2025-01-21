use crate::error::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: f64,
}

impl Config {
    pub fn new() -> Self {
        Config {
            version: 0.0
        }
    }
}

pub fn read_config(project_dirs: &ProjectDirs) -> Result<Config> {
    let config_path = project_dirs.config_dir().join("config.txt");
    let result = fs::read_to_string(config_path)?;
    let config:Config = serde_json::from_str(&result)?;
    Ok(config)
}

pub fn write_to_config(project_dirs: &ProjectDirs, config: &Config) -> Result<()>{
    let config_path = project_dirs.config_dir().join("config.txt");

    let mut  file = OpenOptions::new()
        .write(true)
        .open(config_path)?;

    let content = serde_json::to_string(&config)?;

    Ok(file.write_all(content.as_bytes())?)
}
