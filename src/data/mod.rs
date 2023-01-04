use std::fs;
use std::sync::Arc;

use druid::{Data, Lens};
use log::error;

use crate::{
    mods::ccl::{self, Car},
    Config,
};

use self::nav::Nav;

pub mod config;
pub mod nav;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub nav: Nav,
    pub cars: Arc<Vec<Car>>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        let mut state = Self {
            cars: Arc::new(Vec::new()),
            nav: Nav::default(),
            config,
        };
        state.update_cars();
        state
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
                            let car = match Car::new(path) {
                                Ok(res) => res,
                                Err(err) => {
                                    error!("Failed to read car configuration: {}", err);
                                    todo!("alert");
                                    // return;
                                }
                            };
                            cars.push(car)
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
