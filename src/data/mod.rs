use druid::{Data, Lens};
use crate::Config;
use crate::data::ccl_car::CclCar;

pub mod config;
pub mod ccl_car;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    #[data(ignore)]
    pub cars: Vec<CclCar>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            config,
            cars: Vec::new()
        }
    }
}
