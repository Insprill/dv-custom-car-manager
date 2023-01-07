use std::sync::Arc;

use druid::{Data, Lens};

use crate::{
    mods::{ccl::CustomCarLoader, zsounds::ZSounds, Installable},
    Config,
};

use self::nav::Nav;

pub mod config;
pub mod nav;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub nav: Nav,
    pub ccl: CustomCarLoader,
    pub zsounds: ZSounds,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        let mut state = Self {
            nav: Nav::default(),
            config,
            ccl: CustomCarLoader {
                cars: Arc::new(vec![]),
            },
            zsounds: ZSounds {
                sound_groups: Arc::new(vec![]),
            },
        };
        state.ccl.update(&state.config);
        state.zsounds.update(&state.config);
        state
    }
}
