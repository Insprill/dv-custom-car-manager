use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use druid::EventCtx;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::cmd;
use crate::ui::alert::Alert;

const DV_EXE: &str = "DerailValley.exe";

#[derive(Clone, Debug, Data, Lens, Serialize, Deserialize)]
pub struct DerailValley {
    pub install_dir: String,
}

impl DerailValley {
    pub fn new() -> Self {
        DerailValley {
            install_dir: String::new(),
        }
    }

    pub fn attempt_set_install_dir(
        &mut self,
        ctx: &mut EventCtx,
        path: &Path,
    ) -> Result<bool, Box<dyn Error>> {
        if Self::is_valid_install_dir(path)? {
            // is_valid_install_dir ensures the string is valid UTF-8
            self.install_dir = path.to_str().expect("Not valid UTF-8").to_string();
            ctx.submit_command(cmd::CONFIG_SAVE);
            ctx.submit_command(cmd::NAV_TOGGLE.with(true));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn relative_to_install<T: AsRef<[&'static str]>>(&self, arr: T) -> PathBuf {
        PathBuf::from(self.install_dir.as_str()).join(arr.as_ref().iter().collect::<PathBuf>())
    }

    pub fn has_install_dir(&self) -> bool {
        !self.install_dir.is_empty()
    }

    pub fn is_install_dir_valid(&self, ctx: &mut EventCtx) -> bool {
        #[allow(clippy::blocks_in_if_conditions)]
        if self.install_dir.is_empty() {
            return false;
        } else if Self::is_valid_install_dir(Path::new(&self.install_dir)).unwrap_or_else(|err| {
            Alert::error(
                ctx,
                format!(
                    "Failed to read install dir at {:?}: {:?}",
                    self.install_dir, err
                ),
            );
            false
        }) {
            return true;
        }
        false
    }

    // todo: check this on startup
    pub fn is_valid_install_dir(path: &Path) -> Result<bool, Box<dyn Error>> {
        for entry_res in path.read_dir()? {
            let entry = entry_res?;
            let metadata = entry.metadata()?;
            if !metadata.is_file() {
                continue;
            }
            if entry.file_name().to_str().ok_or("not valid utf8")? == DV_EXE {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
