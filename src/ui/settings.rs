use druid::widget::{Button, Flex, Label, TextBox};
use druid::LensExt;
use druid::{Widget, WidgetExt};

use crate::controller::dv::DvController;
use crate::data::AppState;
use crate::ui::theme;
use crate::{cmd, Config};

pub fn root() -> impl Widget<AppState> {
    dv_install_dir()
}

fn dv_install_dir() -> impl Widget<AppState> {
    let settings_header = Label::new("Settings")
        .with_font(theme::HEADER_1_FONT)
        .with_text_alignment(druid::TextAlignment::Center);

    let dv_install_dir_header = Label::new("Derail Valley Install Directory")
        .with_font(theme::HEADER_2_FONT)
        .with_text_alignment(druid::TextAlignment::Center);

    let dv_install_dir_field = TextBox::new()
        .scroll()
        .fix_width(375.0)
        .lens(AppState::config.then(Config::dv_install_dir));
    let dv_select_install_dir_button =
        Button::new("Select Install Directory").on_click(|ctx, _, _| {
            let options = druid::FileDialogOptions::new()
                .select_directories()
                .accept_command(cmd::DV_SET_INSTALL_DIR);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });
    let dv_install_dir_row = Flex::row()
        .with_child(dv_install_dir_field)
        .with_spacer(10.0)
        .with_child(dv_select_install_dir_button);

    Flex::column()
        .with_child(settings_header)
        .with_spacer(10.0)
        .with_child(dv_install_dir_header)
        .with_spacer(10.0)
        .with_child(dv_install_dir_row)
        .expand()
        .controller(DvController)
}
