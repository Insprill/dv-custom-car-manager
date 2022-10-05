use std::fs;
use std::path::{PathBuf};

use druid::{Data, Lens};

use crate::Config;
use crate::data::ccl::Car;

pub mod config;
pub mod ccl;

const DV_EXE: &str = "DerailValley.exe";

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    #[data(ignore)]
    pub cars: Vec<Car>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            cars: Vec::new(),
        }
    }

    pub fn attempt_set_install_dir(&mut self, path: PathBuf) {
        if !path.is_dir() {
            panic!("TODO: not a dir")
        }

        let mut paths = fs::read_dir(&path).unwrap();
        if paths.any(|path| path.unwrap().file_name().to_string_lossy().to_string().eq(DV_EXE)) {
            self.config.dv_install_dir = path.to_string_lossy().to_string();
            self.config.save();
        } else {
            panic!("TODO: invalid dir")
        }
    }
}
