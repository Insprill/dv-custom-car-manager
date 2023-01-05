use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{env, fs, io};

use druid::{Data, Lens};
use fs_extra::dir::{self, CopyOptions};
use log::{error, info, warn};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::data::AppState;
use crate::Config;

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
    pub fn new(directory: PathBuf, enabled: bool) -> Result<Self, String> {
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
            directory,
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
        update_cars(state);
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct CarConfig {
    pub identifier: String,
}

impl CarConfig {
    fn new(file: &File) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(file)
    }
}

pub fn cars_path(cnfg: &Config) -> PathBuf {
    PathBuf::from(cnfg.dv_install_dir.as_str()).join(CARS_PATH.iter().collect::<PathBuf>())
}

pub fn disabled_cars_path() -> PathBuf {
    Config::data_dir().join(DISABLED_CARS_PATH.iter().collect::<PathBuf>())
}

pub fn update_cars(state: &mut AppState) {
    if state.config.dv_install_dir.is_empty() {
        state.cars = Arc::new(Vec::new());
    } else {
        let mut cars = load_cars(cars_path(&state.config), true).unwrap_or_else(|_| vec![]);
        cars.append(&mut load_cars(disabled_cars_path(), false).unwrap_or_else(|_| vec![]));
        cars.sort_by(|a, b| a.config.identifier.cmp(&b.config.identifier));
        state.cars = Arc::new(cars);
    }
}

fn load_cars(dir: PathBuf, enabled: bool) -> Result<Vec<Car>, Box<dyn Error>> {
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
        match dir_contains_car(&path) {
            Ok(contains_car) => {
                if contains_car {
                    let car = match Car::new(path, enabled) {
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

pub fn dir_contains_car(path: &PathBuf) -> Result<bool, io::Error> {
    let mut paths = match fs::read_dir(path) {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    if paths.any(|path| match path {
        Ok(path) => path
            .file_name()
            .to_string_lossy()
            .to_string()
            .eq(CONFIG_NAME),
        Err(_) => false,
    }) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn install_from_archive(path: &PathBuf, state: &mut AppState) {
    let is_supported = match is_supported_archive(path) {
        Ok(res) => res,
        Err(err) => {
            error!(
                "Failed to read archive at \"{}\": {}",
                path.to_string_lossy().to_string(),
                err.to_string()
            );
            todo!("alert");
            // return;
        }
    };
    if !is_supported {
        error!(
            "{} is not a supported archive type!",
            path.to_string_lossy().to_string()
        );
        todo!("alert");
        // return;
    }

    let archive_file = match File::open(path) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to open archive: {}", err.to_string());
            todo!("alert");
            // return;
        }
    };

    let mut archive = match ZipArchive::new(BufReader::new(archive_file)) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to open archive: {}", err.to_string());
            todo!("alert");
            // return;
        }
    };

    if !archive.file_names().any(|name| name.ends_with(CONFIG_NAME)) {
        error!(
            "Failed to find car in archive \"{}\"",
            path.to_string_lossy().to_string()
        );
        todo!("alert");
        // return;
    }

    let extract_dir = env::temp_dir()
        .join("customcarmanager")
        .join(Alphanumeric.sample_string(&mut rand::thread_rng(), 12));

    info!(
        "Created temp dir at \"{}\"",
        extract_dir.to_string_lossy().to_string()
    );

    match extract_archive(&mut archive, &extract_dir) {
        Ok(_) => {
            install_from_folder(&extract_dir, state);
        }
        Err(_) => {
            todo!("alert");
        }
    }

    match fs::remove_dir_all(&extract_dir) {
        Ok(_) => {
            info!(
                "Removed temp dir at \"{}\"",
                extract_dir.to_string_lossy().to_string()
            )
        }
        Err(_) => {
            error!(
                "Failed to remove temp dir at \"{}\"",
                extract_dir.to_string_lossy().to_string()
            );
            todo!("alert");
        }
    }
}

fn extract_archive(
    archive: &mut ZipArchive<BufReader<File>>,
    extract_dir: &Path,
) -> Result<(), io::Error> {
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Failed to extract file {} from archive: {}",
                    i,
                    err.to_string()
                );
                todo!("alert");
                // return Err(err);
            }
        };
        if !file.is_file() {
            continue;
        }

        let file_path = match file.enclosed_name() {
            Some(path) => extract_dir.join(path),
            None => {
                warn!(
                    "Not extracting file with suspicious path \"{}\"",
                    file.name()
                );
                continue;
            }
        };

        if let Some(p) = file_path.parent() {
            if !p.exists() {
                match fs::create_dir_all(p) {
                    Ok(_) => {}
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        }

        let mut outfile = match File::create(&file_path) {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Failed to create file \"{}\"",
                    file_path.to_string_lossy().to_string()
                );
                return Err(err);
            }
        };
        match io::copy(&mut file, &mut outfile) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to extract file \"{}\" to \"{}\"",
                    file.name().to_string(),
                    file_path.to_string_lossy().to_string()
                );
                return Err(err);
            }
        };
        info!(
            "Extracted \"{}\" to \"{}\" ({} bytes)",
            file.name(),
            file_path.display(),
            file.size()
        );
    }
    Ok(())
}

fn is_supported_archive(path: &PathBuf) -> Result<bool, io::Error> {
    let result = match infer::get_from_path(path) {
        Ok(res) => res,
        Err(err) => {
            error!(
                "Failed to read file \"{}\": {}",
                path.to_string_lossy().to_string(),
                err.to_string()
            );
            return Err(err);
        }
    };
    return match result {
        Some(file_type) => match file_type.mime_type() {
            "application/zip" => Ok(true),
            "application/vnd.rar" => Ok(true),
            _ => Ok(false),
        },
        None => Ok(false),
    };
}

pub fn install_from_folder(root_path: &PathBuf, state: &mut AppState) {
    let paths = match fs::read_dir(root_path) {
        Ok(res) => res,
        Err(err) => {
            error!(
                "Failed to read directory \"{}\": {}",
                root_path.to_string_lossy().to_string(),
                err.to_string()
            );
            todo!("alert");
            // return;
        }
    };
    for res in paths {
        let file = match res {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Error occured while reading directory in \"{}\": {}",
                    root_path.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
                // return;
            }
        };
        let path = &file.path();
        let is_supported = is_supported_archive(path).unwrap_or(false);
        if path.is_file() && is_supported {
            install_from_archive(path, state)
        } else if !path.is_dir() {
            continue;
        }
        let contains_car = match dir_contains_car(path) {
            Ok(res) => res,
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
        if !contains_car {
            install_from_folder(path, state);
            continue;
        }
        let temp_car = match Car::new(path.clone(), false) {
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
        if let Some(car) = state
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
        let cars_path = cars_path(&state.config);
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
        update_cars(state);
        info!("Successfully installed {}", temp_car_ident);
    }
}
