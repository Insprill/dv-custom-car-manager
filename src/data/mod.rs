use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use druid::{Data, Lens};
use log::error;

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
        let mut state = Self {
            cars: Arc::new(Vec::new()),
            config,
        };
        state.update_cars();
        state
    }

    pub fn attempt_set_install_dir(&mut self, path: &PathBuf) -> Result<bool, io::Error> {
        if !path.is_dir() {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }

        let mut paths = match fs::read_dir(&path) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        if paths.any(|path| match path {
            Ok(path) => path.file_name().to_string_lossy().to_string().eq(DV_EXE),
            Err(_) => false,
        }) {
            self.config.dv_install_dir = path.to_string_lossy().to_string();
            self.config.save();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn update_cars(&mut self) {
        if self.config.dv_install_dir.is_empty() {
            self.cars = Arc::new(Vec::new());
        } else {
            let mut cars = Vec::new();

            let dirs = match fs::read_dir(ccl::cars_path(&self.config)) {
                Ok(res) => res,
                Err(err) => {
                    error!("Error while updating cars list: {}", err.to_string());
                    todo!("alert");
                }
            };

            for dir in dirs {
                let path = match dir {
                    Ok(path) => path,
                    Err(err) => {
                        error!("Error while reading cars directory: {}", err.to_string());
                        todo!("alert");
                        // continue;
                    }
                }
                .path();
                match ccl::dir_contains_car(&path) {
                    Ok(contains_car) => {
                        if contains_car {
                            cars.push(Car::new(path))
                        }
                    }
                    Err(err) => {
                        error!(
                            "Failed to check if directory \"{}\" contains car: {}",
                            path.to_string_lossy().to_string(),
                            err.to_string()
                        );
                        todo!("alert");
                        // return;
                    }
                };
            }
            self.cars = Arc::new(cars);
        }
    }
}
