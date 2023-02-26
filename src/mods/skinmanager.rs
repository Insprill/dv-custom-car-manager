use std::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use druid::{im::Vector, Data, EventCtx, Lens};

use crate::{data::config::Config, ui::alert::Alert};

use super::Installable;

pub const FILE_OF_INTEREST_NAME: &str = "d.png";
const SKINS_PATH: [&str; 3] = ["Mods", "SkinManagerMod", "Skins"];

#[derive(Clone, Data, Lens)]
pub struct SkinManager {
    pub skins: Vector<Skin>,
}

#[derive(Clone, Data, Lens)]
pub struct Skin {
    pub car_name: String,
    pub directory: Arc<PathBuf>,
}

impl Skin {
    fn new(path: &Path, car_name: String) -> Self {
        Skin {
            car_name,
            directory: Arc::new(path.to_path_buf())
        }
    }
}

impl Installable for SkinManager {
    fn is_file_of_interest(name: &str) -> bool {
        name.ends_with(FILE_OF_INTEREST_NAME)
    }

    fn update(&mut self, ctx: &mut EventCtx, config: &Config) {
        if !config.derail_valley.has_install_dir() {
            self.skins = Vector::new();
            return;
        }

        fn load(skin_manager: &SkinManager, ctx: &mut EventCtx, path: &Path) -> Vec<Skin> {
            skin_manager.load_skins(ctx, path).unwrap_or_else(|err| {
                Alert::error(
                    ctx,
                    format!("Failed to load cars from {:?}: {:?}", path, err),
                );
                vec![]
            })
        }

        let mut cars = self.load_skins(ctx, path).unwrap_or_else(|err| {
            Alert::error(
                ctx,
                format!("Failed to load cars from {:?}: {:?}", path, err),
            );
            vec![]
        })
        cars.sort_by(|a, b| a.config.identifier.cmp(&b.config.identifier));
        self.skins = Vector::from(cars);
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
        let mut already_installed = false;
        if let Some(car) = self
            .cars
            .iter()
            .find(|x| x.config.identifier.eq(&temp_car_ident))
        {
            already_installed = true;
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
        Alert::info(
            ctx,
            format!(
                "Successfully {} {}",
                if already_installed {
                    "updated"
                } else {
                    "installed"
                },
                temp_car_ident
            ),
        );
    }
}

impl SkinManager {
    fn load_skins(&self, ctx: &mut EventCtx, skins_path: &Path) -> Result<Vec<Skin>, Box<dyn Error>> {
        let mut skins = Vec::new();
 
        for dir in fs::read_dir(skins_path)? {
            let path = dir?.path();
            if Self::dir_contains_file_of_interest(&path)? {
                let skin = Skin::new(&path);
                skins.push(skin)
            }
        }  
        Ok(skins)
    }
} 
