use std::fs::File;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_NAME: &str = "car.json";

pub struct Car {
    config: CarConfig,
    directory: PathBuf,
}

impl Car {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            config: Self::read_config(&directory),
            directory,
        }
    }

    fn read_config(directory: &Path) -> CarConfig {
        let file = File::open(directory.join(CONFIG_NAME)).expect("Failed to find car configuration");
        serde_json::from_reader(file).expect("Failed to read car configuration")
    }
}

#[derive(Serialize, Deserialize)]
pub struct CarConfig {
    pub identifier: String,
}
