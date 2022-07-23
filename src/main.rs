use fltk::{prelude::*, *};
use fltk_theme::{ColorTheme, color_themes};
mod home;

fn main() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    theme.apply();
    app.run().unwrap();
}
