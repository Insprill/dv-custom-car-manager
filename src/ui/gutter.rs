use druid::WidgetExt;
use druid::{widget::Flex, Widget};

use crate::data::nav::Nav;
use crate::data::AppState;
use crate::ui::widget::painter;

use super::theme;
use super::widget::svg::svg;

pub fn gutter() -> impl Widget<AppState> {
    Flex::column()
        .with_child(ccl_icon())
        .with_spacer(theme::GUTTER_ICON_SPACING)
        .with_child(cargoswap_icon())
        .with_spacer(theme::GUTTER_ICON_SPACING)
        .with_child(skinmanager_icon())
        .with_spacer(theme::GUTTER_ICON_SPACING)
        .with_child(zsounds_icon())
        .with_flex_spacer(1.0)
        .with_child(settings_icon())
        .must_fill_main_axis(true)
        .expand_height()
        .background(painter::solid(theme::COLOR_RAISED_BACKGROUND))
        .border(theme::COLOR_RAISED_BACKGROUND, theme::GUTTER_PADDING)
}

fn cargoswap_icon() -> impl Widget<AppState> {
    icon(include_str!("assets/icons/cargoswap.svg"), Nav::CargoSwap)
}

fn ccl_icon() -> impl Widget<AppState> {
    icon(include_str!("assets/icons/ccl.svg"), Nav::CustomCarLoader)
}

fn skinmanager_icon() -> impl Widget<AppState> {
    icon(
        include_str!("assets/icons/skinmanager.svg"),
        Nav::SkinManager,
    )
}

fn zsounds_icon() -> impl Widget<AppState> {
    icon(include_str!("assets/icons/zsounds.svg"), Nav::ZSounds)
}

fn settings_icon() -> impl Widget<AppState> {
    icon(include_str!("assets/icons/settings.svg"), Nav::Settings)
}

fn icon(svg_data: &str, nav: Nav) -> impl Widget<AppState> {
    svg(svg_data)
        .padding(theme::GUTTER_ICON_PADDING)
        .background(painter::solid_reactive(
            theme::COLOR_BUTTON_BACKGROUND,
            theme::COLOR_BUTTON_BACKGROUND_HOVER,
            theme::COLOR_BUTTON_BACKGROUND_ACTIVE,
        ))
        .rounded(theme::BORDER_RADIUS)
        .on_click(move |_, state: &mut AppState, _| state.nav = nav)
        .disabled_if(|state, _env| state.config.dv_install_dir.is_empty())
}
