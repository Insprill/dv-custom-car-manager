use druid::{Data, Lens};
use crate::Config;
use crate::data::ccl::Car;

pub mod config;
pub mod ccl;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    #[data(ignore)]
    pub cars: Vec<Car>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            cars: Vec::new()
        }
    }
}
