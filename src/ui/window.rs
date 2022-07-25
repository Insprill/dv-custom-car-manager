use fltk::{*, prelude::*};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};

use crate::ui::home;

pub fn init() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    let ui = home::UserInterface::make_window();
    ui.window.center_screen();
    theme.apply();
    widget_theme.apply();
    app.run().unwrap();
}
