use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
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
const CARS_PATH: [&'static str; 3] = ["Mods", "DVCustomCarLoader", "Cars"];

#[derive(Clone, Data, Lens)]
pub struct Car {
    pub config: CarConfig,
    #[data(ignore)]
    pub directory: PathBuf,
}

impl Car {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            config: CarConfig::new(directory.as_path()),
            directory,
        }
    }

    pub fn delete(&self) -> Result<(), trash::Error> {
        trash::delete(&self.directory)
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct CarConfig {
    pub identifier: String,
}

impl CarConfig {
    fn new(directory: &Path) -> Self {
        let file =
            File::open(directory.join(CONFIG_NAME)).expect("Failed to find car configuration");
        serde_json::from_reader(file).expect("Failed to read car configuration")
    }
}

pub fn cars_path(config: &Config) -> PathBuf {
    PathBuf::from(config.dv_install_dir.as_str()).join(CARS_PATH.iter().collect::<PathBuf>())
}

pub fn dir_contains_car(path: &PathBuf) -> Result<bool, io::Error> {
    let mut paths = match fs::read_dir(&path) {
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
    if !is_file_supported_archive(path) {
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
    extract_dir: &PathBuf,
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
                match fs::create_dir_all(&p) {
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

fn is_file_supported_archive(path: &PathBuf) -> bool {
    let result = infer::get_from_path(path).expect("Failed to read file");
    return match result {
        Some(file_type) => match file_type.mime_type() {
            "application/zip" => true,
            "application/vnd.rar" => true,
            _ => false,
        },
        None => false,
    };
}

pub fn install_from_folder(root_path: &PathBuf, state: &mut AppState) {
    let paths = match fs::read_dir(&root_path) {
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
        if path.is_file() && is_file_supported_archive(path) {
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
        let temp_car_ident = Car::new(path.clone()).config.identifier;
        if let Some(car) = state
            .cars
            .iter()
            .find(|x| x.config.identifier.eq(&temp_car_ident))
        {
            info!(
                "Deleting old car from \"{}\"",
                car.directory.to_string_lossy().to_string()
            );
            car.delete().expect("Failed to delete old car")
        }
        let cars_path = cars_path(&state.config);
        dir::copy(path, cars_path, &CopyOptions::new()).expect("Failed to move dir");
        state.update_cars();
        info!("Successfully installed {}", temp_car_ident);
    }
}