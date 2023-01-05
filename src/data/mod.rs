use std::sync::Arc;

use druid::{Data, Lens};

use crate::{
    mods::ccl::{self, Car},
    Config,
};

use self::nav::Nav;

pub mod config;
pub mod nav;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub nav: Nav,
    pub cars: Arc<Vec<Car>>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        let mut state = Self {
            cars: Arc::new(Vec::new()),
            nav: Nav::default(),
            config,
        };
        ccl::update_cars(&mut state);
        state
    }
}
