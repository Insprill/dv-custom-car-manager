use druid::widget::{Flex, ViewSwitcher};
use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};

use crate::data::nav::Nav;
use crate::data::AppState;

pub mod ccl;
pub mod gutter;
pub mod settings;
pub mod theme;
pub mod widget;
pub mod zsounds;

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(root())
        .title("DV Content Manager")
        .window_size((700.0, 500.0))
        .with_min_size((600.0, 300.0))
}

fn root() -> impl Widget<AppState> {
    Flex::row()
        .with_child(gutter::gutter())
        .with_flex_child(nav(), 1.0)
}

fn nav() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |state: &AppState, _| state.nav,
        |selector, _state, _| match selector {
            Nav::Settings => Box::new(settings::root()),
            Nav::CustomCarLoader => Box::new(ccl::root()),
            Nav::SkinManager => todo!(),
            Nav::CargoSwap => todo!(),
            Nav::ZSounds => Box::new(zsounds::root()),
        },
    )
    .align_vertical(UnitPoint::TOP)
}
