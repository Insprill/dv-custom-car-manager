use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

use druid::{Data, Lens};
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use log::error;
use log::info;
use serde::{self, Deserialize};
use serde_json_lenient::Value;

use crate::data::config::Config;

use super::Installable;

pub const CONFIG_NAME: &str = "zsounds-config.json";
const SOUNDS_PATH: [&str; 1] = ["Mods"];

#[derive(Debug, Clone, Data, Lens)]
pub struct SoundGroup {
    pub name: String,
    pub sounds: Arc<Vec<Sound>>,
    #[data(ignore)]
    pub directory: PathBuf,
}

impl SoundGroup {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let this_caused_much_pain: Value = serde_json_lenient::from_reader(file)?;
        let mut sound_config: SoundConfig = serde_json_lenient::from_value(this_caused_much_pain)?;
        let sound_folder = path.parent().ok_or("unknown")?;
        let path_name = sound_folder
            .file_name()
            .ok_or("unknown")?
            .to_string_lossy()
            .to_string();
        sound_config.sounds.iter_mut().for_each(|(name, sound)| {
            sound.path = sound_folder.join(&sound.filename);
            sound.name = name.to_string();
        });
        Ok(SoundGroup {
            name: path_name,
            sounds: Arc::new(sound_config.sounds.into_values().collect()),
            directory: sound_folder.to_path_buf(),
        })
    }

    pub fn delete(&self) -> Result<(), trash::Error> {
        trash::delete(&self.directory)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SoundConfig {
    pub sounds: HashMap<String, Sound>,
}

#[derive(Debug, Clone, Data, Lens, Deserialize)]
pub struct Sound {
    #[serde(skip)]
    pub name: String,
    #[serde(skip)]
    #[data(ignore)]
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub sound_type: String,
    pub filename: String,
}

#[derive(Clone, Data, Lens)]
pub struct ZSounds {
    pub sound_groups: Arc<Vec<SoundGroup>>,
}

impl Installable for ZSounds {
    fn is_file_of_interest(name: &str) -> bool {
        name == CONFIG_NAME
    }

    fn update(&mut self, config: &Config) {
        if config.dv_install_dir.is_empty() {
            self.sound_groups = Arc::new(Vec::new());
            return;
        }
        let (mut groups, errors) = Self::load_sounds(config).unwrap_or_else(|err| {
            error!("Error loading sounds: {:?}", err);
            //todo: alert
            (vec![], vec![])
        });
        for err in errors {
            error!("Error loading sound: {:?}", err);
            //todo: alert
        }
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        self.sound_groups = Arc::new(groups);
    }

    fn install(&mut self, config: &Config, path: &Path) {
        let tmp = match SoundGroup::new(path) {
            Ok(res) => res,
            Err(err) => {
                error!(
                    "Failed to read sound configuration in \"{}\": {}",
                    path.to_string_lossy().to_string(),
                    err
                );
                todo!("alert");
                // continue;
            }
        };

        if let Some(sound_group) = self.find_sound_group(&tmp) {
            info!(
                "Deleting old sound group from \"{}\"",
                sound_group.directory.to_string_lossy().to_string()
            );
            match sound_group.delete() {
                Ok(_) => {}
                Err(err) => {
                    error!(
                        "Failed to delete old sound group {} at \"{}\": {}",
                        sound_group.name,
                        sound_group.directory.to_string_lossy().to_string(),
                        err.to_string()
                    );
                    todo!("alert");
                    // continue;
                }
            }
        }
        let sounds_path = config.relative_to_install(SOUNDS_PATH);
        match dir::copy(path, &sounds_path, &CopyOptions::new()) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to copy car from \"{}\" to \"{}\": {}",
                    path.to_string_lossy().to_string(),
                    sounds_path.to_string_lossy().to_string(),
                    err.to_string()
                );
                todo!("alert");
                // continue;
            }
        };
        info!("Successfully installed {}", tmp.name);
    }
}

impl ZSounds {
    // TODO: Fix this mess
    #[allow(clippy::type_complexity)]
    fn load_sounds(
        config: &Config,
    ) -> Result<(Vec<SoundGroup>, Vec<Box<dyn Error>>), Box<dyn Error>> {
        let (groups, errors): (Vec<_>, Vec<_>) =
            dir::get_dir_content(config.relative_to_install(SOUNDS_PATH))?
                .files
                .into_iter()
                .map(PathBuf::from)
                .filter(|path| {
                    Self::is_file_of_interest(
                        &PathBuf::from(path)
                            .file_name()
                            .unwrap_or_else(|| OsStr::new("Failed to find file name"))
                            .to_string_lossy(),
                    )
                })
                .map(|path| SoundGroup::new(&path))
                .partition(Result::is_ok);
        Ok((
            groups.into_iter().map(|g| g.unwrap()).collect(),
            errors.into_iter().map(|e| e.unwrap_err()).collect(),
        ))
    }

    fn find_sound_group(&self, other_group: &SoundGroup) -> Option<&SoundGroup> {
        // Check if any nested sounds have the same name
        for group in self.sound_groups.iter() {
            if group.name == other_group.name {
                return Some(group);
            }
            for sound in group.sounds.iter() {
                if sound.name == other_group.name {
                    return Some(group);
                }
            }
        }
        None
    }
}
