use std::fs::{create_dir_all, File, OpenOptions};
use std::path::PathBuf;

use druid::{Data, Lens};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize, Default)]
pub struct Config {
    pub dv_install_dir: String,
}

impl Config {
    fn app_dirs() -> Option<AppDirs> {
        AppDirs::new(Some("custom-car-manager"), false)
    }

    fn config_dir() -> Option<PathBuf> {
        Self::app_dirs().map(|dirs| dirs.config_dir)
    }

    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.json"))
    }

    pub fn load() -> Option<Config> {
        let path = Self::config_path().expect("Failed to find config path");
        match File::open(&path) {
            Ok(file) => { Some(serde_json::from_reader(file).expect("Failed to load config")) }
            Err(_) => None
        }
    }

    pub fn save(&self) {
        let dir = &Self::config_dir().expect("Failed to find config directory");
        create_dir_all(dir).expect("Failed to create config directory");

        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);

        let path = &Self::config_path().expect("Failed to find config path");
        let file = options.open(path).expect("Failed to create/open config");

        serde_json::to_writer_pretty(file, self).expect("Failed to write config");
    }
}
