use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::widget::{Flex, Label, TextBox};

use crate::data::AppState;

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(root())
        .title("Custom Car Manager")
}

fn root() -> impl Widget<AppState> {
    let install_dir_header = Label::new("Derail Valley Install Directory");
    let install_dir_field = TextBox::new()
        .fix_width(500.0)
        .lens(AppState::dv_install_dir);
    Flex::column()
        .with_child(install_dir_header)
        .with_spacer(10.0)
        .with_child(install_dir_field)
        .align_vertical(UnitPoint::CENTER)
}
