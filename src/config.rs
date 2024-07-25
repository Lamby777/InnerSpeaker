use std::path::Path;

use serde::{Deserialize, Serialize};

use super::*;

pub fn user_data_dir() -> PathBuf {
    dirs::data_dir().unwrap().join(APP_ID)
}

pub fn config_file_path() -> PathBuf {
    user_data_dir().join("state.json")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub bpm: u32,
}

impl Config {
    pub fn load_or_create() -> Self {
        let config_path = config_file_path();

        if !config_path.exists() {
            let config = Self::default();
            config.save();
            config
        } else {
            Self::load(&config_path)
        }
    }

    pub fn load(path: &Path) -> Self {
        let config_file = fs::read_to_string(path).unwrap();
        match serde_json::from_str(&config_file) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Error reading config file, creating new one.");
                let new = Self::default();
                new.save();
                new
            }
        }
    }

    pub fn save(&self) {
        let config_file = serde_json::to_string(self).unwrap();
        fs::write(config_file_path(), config_file).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { bpm: DEFAULT_BPM }
    }
}
