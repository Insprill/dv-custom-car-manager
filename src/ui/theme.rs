use druid::{
    theme::{self},
    Color, Env, Key,
};

use crate::data::AppState;

pub const WINDOW_BACKGROUND_COLOR: Key<Color> = Key::new("app.theme.window-background-color");
pub const BUTTON_LIGHT: Key<Color> = Key::new("app.theme.button-light");
pub const BUTTON_DARK: Key<Color> = Key::new("app.theme.button-dark");
pub const TEXT_COLOR: Key<Color> = Key::new("app.theme.text-color");

pub fn setup_theme(env: &mut Env, _state: &AppState) {
    setup_dark_theme(env);

    // Colors
    env.set(
        theme::WINDOW_BACKGROUND_COLOR,
        env.get(WINDOW_BACKGROUND_COLOR),
    );
    env.set(theme::BUTTON_LIGHT, env.get(BUTTON_LIGHT));
    env.set(theme::BUTTON_DARK, env.get(BUTTON_DARK));
    env.set(theme::TEXT_COLOR, env.get(TEXT_COLOR));

    // Style
    env.set(theme::BUTTON_BORDER_RADIUS, 3.0);
}

fn setup_dark_theme(env: &mut Env) {
    env.set(WINDOW_BACKGROUND_COLOR, hex("#212121"));
    env.set(BUTTON_LIGHT, hex("#323232"));
    env.set(BUTTON_DARK, hex("#323232"));
    env.set(TEXT_COLOR, hex("#e1e1e1"));
}

fn hex(hex: &str) -> Color {
    Color::from_hex_str(hex).unwrap()
}
