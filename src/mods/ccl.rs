use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use druid::{Data, Lens};
use fs_extra::dir::{self, CopyOptions};
use log::{info, warn};
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

    let extract_dir = env::temp_dir()
        .join("customcarmanager")
        .join(Alphanumeric.sample_string(&mut rand::thread_rng(), 12));

    info!(
        "Created temp dir at \"{}\"",
        extract_dir.to_string_lossy().to_string()
    );

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get archive file");
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
    install_from_folder(&extract_dir, state);
    fs::remove_dir_all(&extract_dir).expect("Failed to remove temp dir");
    info!(
        "Removed temp dir at \"{}\"",
        extract_dir.to_string_lossy().to_string()
    )
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
    if !root_path.is_dir() {
        panic!("TODO: not a directory")
    }
    for res in fs::read_dir(root_path).unwrap() {
        let file = res.unwrap();
        let file_path = &file.path();
        if file_path.is_dir() {
            if dir_contains_car(file_path) {
                let temp_car_ident = Car::new(file_path.clone()).config.identifier;
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
                dir::copy(file_path, cars_path, &CopyOptions::new()).expect("Failed to move dir");
                state.update_cars();
                info!("Successfully installed {}", temp_car_ident);
            } else {
                install_from_folder(file_path, state);
            }
        } else if file_path.is_file() && is_file_supported_archive(file_path) {
            install_from_archive(file_path, state)
        }
    }
}

fn create_parents(path: &PathBuf) {
    if let Some(p) = path.parent() {
        if !p.exists() {
            fs::create_dir_all(&p).expect("Failed to create directory");
        }
    }
}
