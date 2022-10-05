use std::fs;
use std::path::{PathBuf};

use druid::{Data, Lens};

use crate::Config;
use crate::data::ccl::{Car};

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
            cars: collect_cars(&config),
            config,
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

fn collect_cars(config: &Config) -> Vec<Car> {
    if config.dv_install_dir.is_empty() {
        Vec::new()
    } else {
        let mut cars = Vec::new();
        for path in fs::read_dir(ccl::cars_path(config)).unwrap() {
            if ccl::dir_contains_car(&path.as_ref().unwrap().path()) {
                cars.push(Car::new(path.unwrap().path()))
            }
        }
        cars
    }
}
