use druid::AppLauncher;

use crate::data::AppState;
use crate::data::config::Config;

mod ui;
mod data;

fn main() {
    let config = Config::load().unwrap_or_default();
    let state = AppState::from_config(config);

    let main_window = ui::main_window();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
