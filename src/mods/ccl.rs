use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use druid::{Data, EventCtx, Lens};
use fs_extra::dir::{self, CopyOptions};
use log::info;
use serde::Deserialize;

use crate::data::AppState;
use crate::ui::alert::Alert;
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
    pub fn new(directory: &Path, enabled: bool) -> Result<Self, Box<dyn Error>> {
        let config_path = directory.join(CONFIG_NAME);
        let file = match File::open(config_path) {
            Ok(res) => res,
            Err(err) => return Err(Box::new(err)),
        };
        let cnfg = match CarConfig::new(&file) {
            Ok(res) => res,
            Err(err) => return Err(Box::new(err)),
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

    pub fn enable(&self, ctx: &mut EventCtx, state: &mut AppState) {
        let path = cars_path(&state.config);
        if Self::move_car(self, ctx, state, &path).is_ok() {
            Alert::info(ctx, format!("Enabled {}", self.config.identifier));
        }
    }

    pub fn disable(&self, ctx: &mut EventCtx, state: &mut AppState) {
        let disabled_dir = Config::data_dir().join("ccl").join("disabled");
        if Self::move_car(self, ctx, state, &disabled_dir).is_ok() {
            Alert::info(ctx, format!("Disabled {}", self.config.identifier));
        }
    }

    fn move_car(
        &self,
        ctx: &mut EventCtx,
        state: &mut AppState,
        new_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        let self_dir = &self.directory;
        match fs::create_dir_all(new_dir) {
            Ok(_) => {}
            Err(err) => {
                Alert::error(
                    ctx,
                    format!("Failed to create directory {:?}: {:?}", new_dir, err),
                );
                return Err(Box::new(err));
            }
        };
        match dir::move_dir(self_dir, new_dir, &CopyOptions::new()) {
            Ok(_) => {}
            Err(err) => {
                Alert::error(
                    ctx,
                    format!(
                        "Failed to copy car from {:?} to {:?}: {:?}",
                        self_dir, new_dir, err
                    ),
                );
                return Err(Box::new(err));
            }
        };
        state.ccl.update(ctx, &state.config);
        Ok(())
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

    fn update(&mut self, ctx: &mut EventCtx, config: &Config) {
        if config.dv_install_dir.is_empty() {
            self.cars = Arc::new(Vec::new());
            return;
        }

        fn load(ccl: &CustomCarLoader, ctx: &mut EventCtx, path: &Path, enabled: bool) -> Vec<Car> {
            ccl.load_cars(ctx, path, enabled).unwrap_or_else(|err| {
                Alert::error(
                    ctx,
                    format!("Failed to load cars from {:?}: {:?}", path, err),
                );
                vec![]
            })
        }

        let mut cars = load(self, ctx, &cars_path(config), true);
        cars.append(&mut load(self, ctx, &disabled_cars_path(), false));
        cars.sort_by(|a, b| a.config.identifier.cmp(&b.config.identifier));
        self.cars = Arc::new(cars);
    }

    fn install(&mut self, ctx: &mut EventCtx, config: &Config, path: &Path) {
        let temp_car = match Car::new(path, false) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(
                    ctx,
                    format!("Failed to read car configuration in {:?}: {:?}", path, err),
                );
                return;
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
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to delete old car {} at {:?}: {:?}",
                            car.config.identifier, car.directory, err
                        ),
                    );
                    return;
                }
            }
        }
        let cars_path = cars_path(config);
        match dir::copy(path, &cars_path, &CopyOptions::new()) {
            Ok(_) => {}
            Err(err) => {
                Alert::error(
                    ctx,
                    format!(
                        "Failed to copy car {} from {:?} to {:?}: {:?}",
                        temp_car_ident, path, cars_path, err
                    ),
                );
                return;
            }
        };
        Alert::info(ctx, format!("Successfully installed {}", temp_car_ident));
    }
}

impl CustomCarLoader {
    fn load_cars(
        &self,
        ctx: &mut EventCtx,
        dir: &Path,
        enabled: bool,
    ) -> Result<Vec<Car>, Box<dyn Error>> {
        let mut cars = Vec::new();

        if !dir.is_dir() {
            return Ok(cars);
        }

        let dirs = match fs::read_dir(&dir) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(ctx, format!("Failed to read cars directory: {:?}", err));
                return Err(Box::new(err));
            }
        };

        for dir in dirs {
            let path = match dir {
                Ok(path) => path,
                Err(err) => {
                    Alert::error(ctx, format!("Failed to read car directory: {:?}", err));
                    return Err(Box::new(err));
                }
            }
            .path();
            match Self::dir_contains_file_of_interest(&path) {
                Ok(contains_car) => {
                    if contains_car {
                        let car = match Car::new(&path, enabled) {
                            Ok(res) => res,
                            Err(err) => {
                                Alert::error(
                                    ctx,
                                    format!(
                                        "Failed to read car configuration at {:?}: {:?}",
                                        path, err
                                    ),
                                );
                                return Err(err);
                            }
                        };
                        cars.push(car)
                    }
                }
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to check if directory {:?} contains car: {:?}",
                            path, err
                        ),
                    );
                    return Err(Box::new(err));
                }
            };
        }
        Ok(cars)
    }
}
