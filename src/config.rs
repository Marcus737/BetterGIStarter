use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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

pub fn read_config(project_dirs: &ProjectDirs) -> Result<Config, String> {
    let config_path = project_dirs.config_dir().join("config.txt");
    let result = fs::read_to_string(config_path);
    let s = match result {
        Ok(s) => {s}
        Err(e) => {
            return Err(e.to_string())
        }
    };
    let config:Config = serde_json::from_str(&s).unwrap();
    Ok(config)
}

pub fn write_to_config(project_dirs: &ProjectDirs, config: &Config) -> Result<(), String>{
    let config_path = project_dirs.config_dir().join("config.txt");
    let mut file = match OpenOptions::new()
        .write(true)
        .open(config_path) {
        Ok(f) => {f}
        Err(e) => {
            return Err(format!("打开配置文件失败：{}", e.to_string()));
        }
    };
    let content = serde_json::to_string(&config).unwrap();
    match file.write_all(content.as_bytes()) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(format!("写入配置文件失败：{}", e.to_string()))
        }
    }
}
