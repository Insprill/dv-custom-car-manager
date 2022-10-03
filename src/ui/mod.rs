use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, Label, TextBox};

use crate::cmd;
use crate::data::AppState;

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(root())
        .title("Custom Car Manager")
        .window_size((600.0, 400.0))
}

fn root() -> impl Widget<AppState> {
    let install_dir_header = Label::new("Derail Valley Install Directory")
        .with_text_size(20.0);

    let install_dir_field = TextBox::new()
        .fix_width(400.0)
        .lens(AppState::dv_install_dir);
    let select_install_dir_button = Button::new("Select Install Dir")
        .on_click(|ctx, _, _| ctx.submit_command(cmd::SELECT_INSTALL_DIR));
    let install_dir_row = Flex::row()
        .with_child(install_dir_field)
        .with_spacer(5.0)
        .with_child(select_install_dir_button);

    Flex::column()
        .with_child(install_dir_header)
        .with_spacer(10.0)
        .with_child(install_dir_row)
        .align_vertical(UnitPoint::TOP)
}
