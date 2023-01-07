use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use druid::{Data, Lens};
use fs_extra::dir::{self, CopyOptions};
use log::{error, info};
use serde::Deserialize;

use crate::data::AppState;
use crate::Config;

use super::Installable;

pub const CONFIG_NAME: &str = "car.json";
const CARS_PATH: [&str; 3] = ["Mods", "DVCustomCarLoader", "Cars"];
const DISABLED_CARS_PATH: [&str; 2] = ["ccl", "disabled"];

#[derive(Clone, Data, Lens)]
pub struct Car {
    pub config: CarConfig,
    #[data(ignore)]
    pub directory: PathBuf,
    pub enabled: bool,
}

impl Car {
    pub fn new(directory: &Path, enabled: bool) -> Result<Self, String> {
        let config_path = directory.join(CONFIG_NAME);
        let file = match File::open(config_path) {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        let cnfg = match CarConfig::new(&file) {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        Ok(Self {
            config: cnfg,
            directory: directory.to_path_buf(),
            enabled,
        })
    }

    pub fn delete(&self) -> Result<(), trash::Error> {
        trash::delete(&self.directory)
    }

    pub fn enable(&self, state: &mut AppState) {
        let path = cars_path(&state.config);
        Self::move_car(self, state, &path);
    }

    pub fn disable(&self, state: &mut AppState) {
        let disabled_dir = Config::data_dir().join("ccl").join("disabled");
        Self::move_car(self, state, &disabled_dir);
    }

    fn move_car(&self, state: &mut AppState, new_dir: &Path) {
        let self_dir = &self.directory;
        match fs::create_dir_all(new_dir) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to create dir \"{}\": {}",
                    new_dir.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
                // return;
            }
        };
        match dir::move_dir(self_dir, new_dir, &CopyOptions::new()) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to copy car from \"{}\" to \"{}\": {}",
                    self_dir.to_string_lossy().to_string(),
                    new_dir.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
                // return;
            }
        };
        state.ccl.update(&state.config);
    }
}

#[derive(Clone, Data, Lens, Deserialize)]
pub struct CarConfig {
    pub identifier: String,
}

impl CarConfig {
    fn new(file: &File) -> Result<Self, serde_json_lenient::Error> {
        serde_json_lenient::from_reader(file)
    }
}

pub fn cars_path(config: &Config) -> PathBuf {
    config.relative_to_install(CARS_PATH)
}

pub fn disabled_cars_path() -> PathBuf {
    Config::data_dir().join(DISABLED_CARS_PATH.iter().collect::<PathBuf>())
}

#[derive(Clone, Data, Lens)]
pub struct CustomCarLoader {
    pub cars: Arc<Vec<Car>>,
}

impl Installable for CustomCarLoader {
    fn is_file_of_interest(name: &str) -> bool {
        name.eq(CONFIG_NAME)
    }

    fn update(&mut self, config: &Config) {
        if config.dv_install_dir.is_empty() {
            self.cars = Arc::new(Vec::new());
            return;
        }
        let mut cars = self
            .load_cars(cars_path(config), true)
            .unwrap_or_else(|err| {
                error!(
                    "Error loading cars from \"{:?}\": {:?}",
                    cars_path(config),
                    err
                );
                vec![]
            });
        cars.append(
            &mut self
                .load_cars(disabled_cars_path(), false)
                .unwrap_or_else(|err| {
                    error!(
                        "Error loading cars from \"{:?}\": {:?}",
                        disabled_cars_path(),
                        err
                    );
                    vec![]
                }),
        );
        cars.sort_by(|a, b| a.config.identifier.cmp(&b.config.identifier));
        self.cars = Arc::new(cars);
    }

    fn install(&mut self, config: &Config, path: &Path) {
        let temp_car = match Car::new(path, false) {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Failed to read car configuration in \"{}\": {}",
                    path.to_string_lossy().to_string(),
                    err
                );
                todo!("alert");
                // continue;
            }
        };
        let temp_car_ident = temp_car.config.identifier;
        if let Some(car) = self
            .cars
            .iter()
            .find(|x| x.config.identifier.eq(&temp_car_ident))
        {
            info!(
                "Deleting old car from \"{}\"",
                car.directory.to_string_lossy().to_string()
            );
            match car.delete() {
                Ok(_) => {}
                Err(err) => {
                    error!(
                        "Failed to delete old car {} at \"{}\": {}",
                        car.config.identifier,
                        car.directory.to_string_lossy().to_string(),
                        err.to_string()
                    );
                    todo!("alert");
                    // continue;
                }
            }
        }
        let cars_path = cars_path(config);
        match dir::copy(path, &cars_path, &CopyOptions::new()) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to copy car from \"{}\" to \"{}\": {}",
                    path.to_string_lossy().to_string(),
                    cars_path.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
                // continue;
            }
        };
        info!("Successfully installed {}", temp_car_ident);
    }
}

impl CustomCarLoader {
    fn load_cars(&self, dir: PathBuf, enabled: bool) -> Result<Vec<Car>, Box<dyn Error>> {
        let mut cars = Vec::new();

        if !dir.is_dir() {
            return Ok(cars);
        }

        let dirs = match fs::read_dir(&dir) {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Error while updating cars list from \"{}\": {}",
                    dir.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
            }
        };

        for dir in dirs {
            let path = match dir {
                Ok(path) => path,
                Err(err) => {
                    error!("Error while reading cars directory: {}", err.to_string());
                    todo!("alert");
                    // return Err(err);
                }
            }
            .path();
            match Self::dir_contains_file_of_interest(&path) {
                Ok(contains_car) => {
                    if contains_car {
                        let car = match Car::new(&path, enabled) {
                            Ok(res) => res,
                            Err(err) => {
                                error!("Failed to read car configuration: {}", err);
                                todo!("alert");
                                // return Err(err);
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
                    // return Err(err);
                }
            };
        }
        Ok(cars)
    }
}
