use druid::widget::{Flex, Label, SizedBox, Slider};
use druid::{FileDialogOptions, LensExt};
use druid::{Widget, WidgetExt};

use crate::controller::dv::DvController;
use crate::data::dv::DerailValley;
use crate::data::AppState;
use crate::ui::theme;
use crate::{cmd, Config};

use super::widget::painter;

pub fn root() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Settings").with_font(theme::HEADER_1_FONT))
        .with_default_spacer()
        .with_child(derail_valley())
        .with_default_spacer()
        .with_child(zsounds())
}

fn derail_valley() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Derail Valley").with_font(theme::HEADER_2_FONT))
        .with_default_spacer()
        .with_child(dv_install_dir())
        .expand()
        .controller(DvController)
}

fn dv_install_dir() -> impl Widget<AppState> {
    let dv_install_dir_field = Label::raw()
        .with_font(theme::LIST_BOX_ITEM_FONT)
        .scroll()
        .expand_width()
        .padding(theme::TEXT_PADDING)
        .background(painter::solid(theme::COLOR_BUTTON_NORMAL))
        .rounded(theme::BORDER_RADIUS)
        .lens(
            AppState::config
                .then(Config::derail_valley)
                .then(DerailValley::install_dir),
        );
    let dv_select_install_dir_button = Flex::row()
        .with_child(Label::new("Select Install Directory").with_font(theme::LIST_BOX_ITEM_FONT))
        .padding(theme::TEXT_PADDING)
        .background(painter::solid_reactive(theme::COLOR_BUTTON))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, _, _| {
            let options = FileDialogOptions::new()
                .select_directories()
                .accept_command(cmd::DV_SET_INSTALL_DIR);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });
    let dv_install_dir_row = Flex::row()
        .with_default_spacer()
        .with_flex_child(dv_install_dir_field, 1.0)
        .with_default_spacer()
        .with_child(dv_select_install_dir_button)
        .with_default_spacer();

    Flex::column()
        .with_child(Label::new("Installation Directory").with_font(theme::HEADER_3_FONT))
        .with_default_spacer()
        .with_child(dv_install_dir_row)
        .expand()
        .controller(DvController)
}

fn zsounds() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("ZSounds").with_font(theme::HEADER_2_FONT))
        .with_default_spacer()
        .with_child(zs_volume())
        .expand()
        .controller(DvController)
}

fn zs_volume() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("Playback Volume").with_font(theme::HEADER_3_FONT))
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    SizedBox::new(Label::dynamic(|state: &AppState, _| {
                        format!("{:.0}%", state.config.volume * 100.0)
                    }))
                    .width(35.0),
                )
                .with_default_spacer()
                .with_child(Slider::new().lens(AppState::config.then(Config::volume))),
        )
}
