use std::fs::File;

use druid::AppLauncher;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};

use crate::data::config::Config;
use crate::data::AppState;

mod data;
mod ui;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            simplelog::Config::default(),
            File::create(
                Config::config_dir()
                    .expect("Failed to find config dir")
                    .join("latest.log"),
            )
            .unwrap(),
        ),
    ])
    .unwrap();

    let config = Config::load().unwrap_or_default();
    let state = AppState::from_config(config);

    let main_window = ui::main_window();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
