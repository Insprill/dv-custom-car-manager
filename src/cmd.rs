use druid::{FileInfo, Selector};

use crate::mods::ccl::Car;

pub const DV_SET_INSTALL_DIR: Selector<FileInfo> = Selector::new("app.dv.install-dir");
pub const CCL_DELETE_CAR: Selector<Car> = Selector::new("app.ccl.delete-car");
pub const CCL_INSTALL_FOLDER: Selector<FileInfo> = Selector::new("app.ccl.install.folder");
pub const CCL_INSTALL_ARCHIVE: Selector<FileInfo> = Selector::new("app.ccl.install.archive");
