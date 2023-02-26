use std::time::Duration;

use druid::widget::{Flex, ViewSwitcher};
use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};

use crate::cmd;
use crate::controller::RootController;
use crate::data::nav::Nav;
use crate::data::AppState;

use self::widget::overlay::Overlay;
use self::widget::CustomWidgetExt;

pub mod alert;
pub mod ccl;
pub mod gutter;
pub mod settings;
pub mod skinmanager;
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
        .with_flex_child(Overlay::bottom(nav(), alert::alert_box()), 1.0)
        .controller(RootController)
        .run_after(
            |_, _| Duration::ZERO,
            |ctx, _, _| {
                ctx.submit_command(cmd::INIT);
            },
        )
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
