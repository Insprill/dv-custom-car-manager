use fltk::{*, prelude::*};
use fltk_theme::{color_themes, ColorTheme, ThemeType, WidgetTheme};

use crate::ui::home;
use crate::util::utils;

pub fn init() {
    let app = app::App::default();
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    let widget_theme = WidgetTheme::new(ThemeType::Dark);
    let mut ui = home::UserInterface::make_window();
    ui.window.center_screen();
    ui.select_install_dir_button.set_callback(move |_| {
        let (selected, path) = utils::open_folder_chooser();
        println!("{selected}");
        println!("{path}");
    });
    theme.apply();
    widget_theme.apply();
    app.run().unwrap();
}
