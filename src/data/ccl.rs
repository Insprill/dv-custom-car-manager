use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{fs, io};

use log::{info, warn};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::{AppState, Config};

pub const CONFIG_NAME: &str = "car.json";
const CARS_PATH: [&'static str; 3] = ["Mods", "DVCustomCarLoader", "Cars"];

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
        let file =
            File::open(directory.join(CONFIG_NAME)).expect("Failed to find car configuration");
        serde_json::from_reader(file).expect("Failed to read car configuration")
    }
}

pub fn cars_path(config: &Config) -> PathBuf {
    PathBuf::from(config.dv_install_dir.as_str()).join(CARS_PATH.iter().collect::<PathBuf>())
}

pub fn dir_contains_car(path: &PathBuf) -> bool {
    let mut dir = fs::read_dir(path).expect("Failed to read dir");
    dir.any(|f| {
        f.unwrap()
            .file_name()
            .to_string_lossy()
            .to_string()
            .eq(CONFIG_NAME)
    })
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
    let archive_file = File::open(path).expect("Failed to open archive file");
    let reader = BufReader::new(archive_file);

    let mut archive = ZipArchive::new(reader).expect("Failed to open archive");

    if !archive.file_names().any(|name| name.ends_with(CONFIG_NAME)) {
        panic!("TODO: failed to find car")
    }

    let cars_path = cars_path(&state.config);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get archive file");
        let file_path = match file.enclosed_name() {
            Some(path) => cars_path.join(path),
            None => {
                warn!(
                    "Not extracting file with suspicious path \"{}\"",
                    file.name()
                );
                continue;
            }
        }
        .canonicalize()
        .expect("Failed to canonicalize path");

        if !file.is_file() {
            continue;
        }

        if let Some(p) = file_path.parent() {
            if !p.exists() {
                create_dir_all(&p).expect("Failed to create directory");
                info!(
                    "Created directory \"{}\"",
                    file_path.file_name().unwrap().to_string_lossy().to_string()
                );
            }
        }

        let mut outfile = File::create(&file_path).expect("Failed to open file");
        io::copy(&mut file, &mut outfile).expect("Failed to write file");
        info!(
            "Extracted \"{}\" to \"{}\" ({} bytes)",
            file.name(),
            file_path.display(),
            file.size()
        );
    }
}
