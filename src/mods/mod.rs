use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use log::{error, info, warn};
use rand::distributions::{Alphanumeric, DistString};
use zip::ZipArchive;

use crate::data::config::Config;

pub mod ccl;
pub mod zsounds;

pub trait Installable {
    fn install_from_archive(&mut self, path: &PathBuf, config: &Config) {
        let is_supported = match Self::is_supported_archive(path) {
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

        if !archive
            .file_names()
            .any(|name| Self::is_file_of_interest(name))
        {
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

        match Self::extract_archive(&mut archive, &extract_dir) {
            Ok(_) => {
                Self::install_from_folder(self, &extract_dir, config);
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

    fn install_from_folder(&mut self, root_path: &PathBuf, config: &Config) {
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
            let is_supported = Self::is_supported_archive(path).unwrap_or(false);
            if path.is_file() && is_supported {
                Self::install_from_archive(self, path, config)
            } else if !path.is_dir() {
                continue;
            }

            let contains_car = match Self::dir_contains_file_of_interest(path) {
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
                Self::install_from_folder(self, path, config);
                continue;
            }
            Self::install(self, config, path);
            Self::update(self, config);
        }
    }

    fn is_file_of_interest(name: &str) -> bool;

    fn dir_contains_file_of_interest(path: &PathBuf) -> Result<bool, io::Error> {
        let mut paths = match fs::read_dir(path) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        Ok(paths.any(|path| match path {
            Ok(path) => Self::is_file_of_interest(&path.file_name().to_string_lossy()),
            Err(_) => false,
        }))
    }

    fn update(&mut self, config: &Config);

    fn install(&mut self, config: &Config, path: &Path);
}
