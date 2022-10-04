use druid::{Data, Lens};
use crate::Config;

pub mod config;
pub mod ccl_car;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
        }
    }
}
