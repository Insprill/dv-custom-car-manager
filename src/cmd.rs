use druid::{FileInfo, Selector};

use crate::{
    mods::{
        ccl::Car,
        zsounds::{Sound, SoundGroup},
    },
    ui::alert::Alert,
};

// DV
pub const DV_SET_INSTALL_DIR: Selector<FileInfo> = Selector::new("app.dv.set-install-dir");
pub const DV_VALIDATE_INSTALL_DIR: Selector = Selector::new("app.dv.validate-install-dir");

// CCL
pub const CCL_DELETE_CAR: Selector<Car> = Selector::new("app.ccl.delete-car");
pub const CCL_INSTALL_FOLDER: Selector<Vec<FileInfo>> = Selector::new("app.ccl.install.folder");
pub const CCL_INSTALL_ARCHIVE: Selector<Vec<FileInfo>> = Selector::new("app.ccl.install.archive");
pub const CCL_ENABLE_CAR: Selector<Car> = Selector::new("app.ccl.enable-car");
pub const CCL_DISABLE_CAR: Selector<Car> = Selector::new("app.ccl.disable-car");

// ZSounds
pub const ZSOUNDS_DELETE_SOUNDGROUP: Selector<SoundGroup> =
    Selector::new("app.zsounds.delete-sound");
pub const ZSOUNDS_INSTALL_FOLDER: Selector<Vec<FileInfo>> =
    Selector::new("app.zsounds.install.folder");
pub const ZSOUNDS_INSTALL_ARCHIVE: Selector<Vec<FileInfo>> =
    Selector::new("app.zsounds.install.archive");
pub const ZSOUNDS_PLAY_SOUND: Selector<Sound> = Selector::new("app.zsounds.play-sound");

// Alerts
pub const ALERT: Selector<Alert> = Selector::new("app.alert");
pub const DISMISS_ALERT: Selector<u32> = Selector::new("app.alert.dismiss");

// Root
pub const CONFIG_SAVE: Selector = Selector::new("app.config.save");
pub const NAV_TOGGLE: Selector<bool> = Selector::new("app.nav.toggle");
