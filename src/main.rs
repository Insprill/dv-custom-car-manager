use std::fs::File;
use druid::AppLauncher;
use log::{LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, TerminalMode, TermLogger, WriteLogger};

use crate::data::AppState;
use crate::data::config::Config;

mod ui;
mod data;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, simplelog::Config::default(), File::create(Config::config_dir().expect("Failed to find config dir").join("latest.log")).unwrap())
        ]
    ).unwrap();

    let config = Config::load().unwrap_or_default();
    let state = AppState::from_config(config);

    let main_window = ui::main_window();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
