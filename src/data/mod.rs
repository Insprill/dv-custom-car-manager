use druid::{Data, Lens};
use crate::Config;

pub mod config;
pub mod ccl_car;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    dv_install_dir: String,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            dv_install_dir: config.dv_install_dir
        }
    }
}
