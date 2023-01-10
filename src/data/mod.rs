use std::sync::Arc;

use druid::{im::Vector, Data, EventCtx, Lens};
use log::{error, info, warn};

use crate::{
    mods::{ccl::CustomCarLoader, zsounds::ZSounds, Installable},
    ui::alert::{Alert, AlertStyle},
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
    pub alerts: Vector<Alert>,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            nav: if config.dv_install_dir.is_empty() {
                Nav::Settings
            } else {
                Nav::default()
            },
            config,
            ccl: CustomCarLoader {
                cars: Arc::new(vec![]),
            },
            zsounds: ZSounds {
                sound_groups: Arc::new(vec![]),
            },
            alerts: Vector::new(),
        }
    }

    pub fn update_all(&mut self, ctx: &mut EventCtx) {
        self.ccl.update(ctx, &self.config);
        self.zsounds.update(ctx, &self.config);
    }

    pub fn alert(&mut self, alert: &Alert) {
        let single_line = alert.message.to_string().replace('\n', ". ");
        match alert.style {
            AlertStyle::Error => error!("{}", single_line),
            AlertStyle::Warn => warn!("{}", single_line),
            AlertStyle::Info => info!("{}", single_line),
        }
        self.alerts.push_back(alert.clone())
    }

    pub fn dismiss_alert(&mut self, id: u32) {
        self.alerts.retain(|a| a.id != id)
    }
}
