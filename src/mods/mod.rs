use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use druid::EventCtx;
use log::info;
use rand::distributions::{Alphanumeric, DistString};
use zip::ZipArchive;

use crate::data::config::Config;
use crate::ui::alert::Alert;

pub mod ccl;
pub mod zsounds;

pub trait Installable {
    fn install_from_archive(&mut self, ctx: &mut EventCtx, path: &PathBuf, config: &Config) {
        let is_supported = match Self::is_supported_archive(ctx, path) {
            Ok(res) => res,
            Err(_) => {
                return;
            }
        };
        if !is_supported {
            Alert::error(ctx, format!("{:?} is not a supported archive type!", path));
            return;
        }

        let archive_file = match File::open(path) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(
                    ctx,
                    format!("Failed to open archive file {:?}: {:?}", path, err),
                );
                return;
            }
        };

        let mut archive = match ZipArchive::new(BufReader::new(archive_file)) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(ctx, format!("Failed to read archive {:?}: {:?}", path, err));
                return;
            }
        };

        if !archive
            .file_names()
            .any(|name| Self::is_file_of_interest(name))
        {
            Alert::error(ctx, format!("Failed to find car in archive {:?}", path));
            return;
        }

        let extract_dir = env::temp_dir()
            .join(env!("CARGO_CRATE_NAME"))
            .join(Alphanumeric.sample_string(&mut rand::thread_rng(), 12));

        info!("Created temp dir at {:?}", extract_dir);

        match Self::extract_archive(ctx, &mut archive, path, &extract_dir) {
            Ok(_) => {
                Self::install_from_folder(self, ctx, &extract_dir, config);
            }
            Err(_) => {
                Alert::error(
                    ctx,
                    format!("Failed to extract archive {:?} to {:?}", path, extract_dir),
                );
                return;
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
                Alert::warn(
                    ctx,
                    format!("Failed to remove temp dir at {:?}", extract_dir),
                );
            }
        }
    }

    fn extract_archive(
        ctx: &mut EventCtx,
        archive: &mut ZipArchive<BufReader<File>>,
        archive_dir: &Path,
        extract_dir: &Path,
    ) -> Result<(), Box<dyn Error>> {
        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(res) => res,
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to the {:?}th from archive {:?} to {:?}",
                            i, archive_dir, extract_dir
                        ),
                    );
                    return Err(Box::new(err));
                }
            };
            if !file.is_file() {
                continue;
            }

            let file_path = match file.enclosed_name() {
                Some(path) => extract_dir.join(path),
                None => {
                    Alert::warn(
                        ctx,
                        format!(
                            "Not extracting file with suspicious path \"{}\"",
                            file.name()
                        ),
                    );
                    continue;
                }
            };

            if let Some(p) = file_path.parent() {
                if !p.exists() {
                    match fs::create_dir_all(p) {
                        Ok(_) => {}
                        Err(err) => {
                            Alert::error(
                                ctx,
                                format!("Failed to create directory {:?}: {:?}", p, err),
                            );
                            return Err(Box::new(err));
                        }
                    }
                }
            }

            let mut outfile = match File::create(&file_path) {
                Ok(res) => res,
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!("Failed to create file {:?}: {:?}", file_path, err),
                    );
                    return Err(Box::new(err));
                }
            };
            match io::copy(&mut file, &mut outfile) {
                Ok(_) => {}
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to extract file \"{}\" to {:?}: {:?}",
                            file.name(),
                            file_path,
                            err
                        ),
                    );
                    return Err(Box::new(err));
                }
            };
            info!(
                "Extracted \"{}\" to {:?} ({} bytes)",
                file.name(),
                file_path,
                file.size()
            );
        }
        Ok(())
    }

    fn is_supported_archive(ctx: &mut EventCtx, path: &PathBuf) -> Result<bool, io::Error> {
        let result = match infer::get_from_path(path) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(
                    ctx,
                    format!("Failed to read archive at {:?}: {:?}", path, err),
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

    fn install_from_folder(&mut self, ctx: &mut EventCtx, root_path: &PathBuf, config: &Config) {
        let paths = match fs::read_dir(root_path) {
            Ok(res) => res,
            Err(err) => {
                Alert::error(
                    ctx,
                    format!("Failed to read directory {:?}: {:?}", root_path, err),
                );
                return;
            }
        };
        for res in paths {
            let file = match res {
                Ok(res) => res,
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!("Failed to read directory {:?}: {:?}", root_path, err),
                    );
                    return;
                }
            };
            let path = &file.path();
            if path.is_file() && Self::is_supported_archive(ctx, path).unwrap_or(false) {
                Self::install_from_archive(self, ctx, path, config)
            } else if !path.is_dir() {
                continue;
            }

            let contains_car = match Self::dir_contains_file_of_interest(path) {
                Ok(res) => res,
                Err(err) => {
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to check if directory {:?} contains car: {:?}",
                            path, err
                        ),
                    );
                    return;
                }
            };
            if !contains_car {
                Self::install_from_folder(self, ctx, path, config);
                continue;
            }
            Self::install(self, ctx, config, path);
            Self::update(self, ctx, config);
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

    fn update(&mut self, ctx: &mut EventCtx, config: &Config);

    fn install(&mut self, ctx: &mut EventCtx, config: &Config, path: &Path);
}
