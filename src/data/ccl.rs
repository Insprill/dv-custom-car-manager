use std::fs::{create_dir_all, File};
use std::{fs, io};
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use zip::ZipArchive;
use crate::{AppState, Config};

pub const CONFIG_NAME: &str = "car.json";
const CARS_PATH: &str = "Mods/DVCustomCarLoader/Cars";

#[derive(Clone)]
pub struct Car {
    pub config: CarConfig,
    pub directory: PathBuf,
}

impl Car {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            config: CarConfig::new(directory.as_path()),
            directory,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CarConfig {
    pub identifier: String,
}

impl CarConfig {
    fn new(directory: &Path) -> Self {
        let file = File::open(directory.join(CONFIG_NAME)).expect("Failed to find car configuration");
        serde_json::from_reader(file).expect("Failed to read car configuration")
    }
}

pub fn cars_path(config: &Config) -> PathBuf {
    PathBuf::from(config.dv_install_dir.as_str()).join(CARS_PATH)
}

pub fn dir_contains_car(path: &PathBuf) -> bool {
    let mut dir = fs::read_dir(path).expect("Failed to read dir");
    dir.any(|f| f.unwrap().file_name().to_string_lossy().to_string().eq(CONFIG_NAME))
}

pub fn install_from_folder(path: PathBuf) {
    if !path.is_dir() {
        panic!("TODO: not a directory")
    }
}

pub fn install_from_archive(path: PathBuf, state: &AppState) {
    if !path.is_file() {
        panic!("TODO: not a file")
    }
    let archive_file = File::open(path).unwrap();
    let reader = BufReader::new(archive_file);

    let mut archive = ZipArchive::new(reader).unwrap();

    if !archive.file_names().any(|name| name.ends_with(CONFIG_NAME)) {
        panic!("TODO: failed to find car")
    }

    let cars_path = cars_path(&state.config);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let file_path = match file.enclosed_name() {
            Some(path) => cars_path.join(path),
            None => {
                println!("Suspicious path {}", file.name());
                continue;
            }
        };

        if file.name().ends_with('/') {
            create_dir_all(&file_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                file_path.display(),
                file.size()
            );
            if let Some(p) = file_path.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&file_path).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
