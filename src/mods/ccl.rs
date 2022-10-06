use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{fs, io};

use druid::{Data, Lens};
use log::{info, warn};
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

    pub fn delete(&self) {
        trash::delete(&self.directory).expect("Failed to move dir to trash");
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

pub fn install_from_folder(path: &PathBuf, state: &mut AppState) {
    if !path.is_dir() {
        panic!("TODO: not a directory")
    }
    for res in fs::read_dir(path).unwrap() {
        let file = res.unwrap();
        let file_path = &file.path();
        let file_type = file.file_type().unwrap();
        if file_type.is_dir() {
            if dir_contains_car(path) {
                install_from_folder(file_path, state);
            } else {
                create_parents(&file_path);
            }
        } else if file_type.is_file() {
            install_from_archive(file_path, state)
        }
    }
}

pub fn install_from_archive(path: &PathBuf, state: &mut AppState) {
    if !path.is_file() {
        panic!("TODO: not a file")
    }
    if !is_file_supported_archive(path) {
        panic!("TODO: not supported")
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
        };

        if !file.is_file() {
            continue;
        }

        create_parents(&file_path);

        let mut outfile = File::create(&file_path).expect("Failed to open file");
        io::copy(&mut file, &mut outfile).expect("Failed to write file");
        info!(
            "Extracted \"{}\" to \"{}\" ({} bytes)",
            file.name(),
            file_path.display(),
            file.size()
        );
    }
    state.update_cars();
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

fn create_parents(path: &PathBuf) {
    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(&p).expect("Failed to create directory");
            info!(
                "Created directory \"{}\"",
                p.file_name().unwrap().to_string_lossy().to_string()
            );
        }
    }
}
