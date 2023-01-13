use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use druid::{Data, Lens};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};

use super::dv::DerailValley;

#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct Config {
    pub derail_valley: DerailValley,
    pub volume: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            derail_valley: DerailValley::new(),
            volume: 0.25,
        }
    }
}

impl Config {
    fn app_dirs() -> Option<AppDirs> {
        AppDirs::new(Some(env!("CARGO_PKG_NAME")), false)
    }

    pub fn config_dir() -> PathBuf {
        Self::app_dirs()
            .map(|dirs| dirs.config_dir)
            .expect("Failed to find config path")
    }

    pub fn data_dir() -> PathBuf {
        Self::app_dirs()
            .map(|dirs| dirs.data_dir)
            .expect("Failed to find data path")
    }

    fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    pub fn setup_dirs() {
        fs::create_dir_all(Self::config_dir()).expect("Failed to create config directory");
        fs::create_dir_all(Self::data_dir()).expect("Failed to create data directory");
    }

    pub fn load() -> Option<Config> {
        let path = Self::config_path();
        match File::open(path) {
            Ok(file) => Some(serde_json_lenient::from_reader(file).expect("Failed to load config")),
            Err(_) => None,
        }
    }

    pub fn save(&self) {
        let dir = &Self::config_dir();
        fs::create_dir_all(dir).expect("Failed to create config directory");

        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);

        let path = &Self::config_path();
        let file = options.open(path).expect("Failed to create/open config");

        serde_json_lenient::to_writer_pretty(file, self).expect("Failed to write config");
    }
}
