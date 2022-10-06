use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use druid::{Data, Lens};

use crate::{
    mods::ccl::{self, Car},
    Config,
};

pub mod config;

const DV_EXE: &str = "DerailValley.exe";

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub cars: Arc<Vec<Car>>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            cars: Arc::new(collect_cars(&config)),
            config,
        }
    }

    pub fn attempt_set_install_dir(&mut self, path: PathBuf) {
        if !path.is_dir() {
            panic!("TODO: not a dir")
        }

        let mut paths = fs::read_dir(&path).unwrap();
        if paths.any(|path| {
            path.unwrap()
                .file_name()
                .to_string_lossy()
                .to_string()
                .eq(DV_EXE)
        }) {
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
