use fltk::{*, prelude::*};
use fltk_theme::{color_themes, ColorTheme};
use crate::ui::home;

pub fn init() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    let mut ui = home::UserInterface::make_window();
    ui.window.center_screen();
    ui.select_install_dir_button.set_callback(move |_| {
        println!("Works!");
    });
    theme.apply();
    app.run().unwrap();
}
