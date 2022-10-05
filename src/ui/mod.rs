use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};
use druid::LensExt;
use druid::widget::{Button, Flex, Label, Scroll, TextBox};
use native_dialog::FileDialog;

use crate::Config;
use crate::data::{AppState, ccl};
use crate::data::ccl::Car;

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(root())
        .title("Custom Car Manager")
        .window_size((600.0, 400.0))
}

fn root() -> impl Widget<AppState> {
    let dv_install_dir_header = Label::new("Derail Valley Install Directory")
        .with_text_size(20.0);

    let dv_install_dir_field = TextBox::new()
        .fix_width(375.0)
        .lens(AppState::config.then(Config::dv_install_dir));
    let dv_select_install_dir_button = Button::new("Select Install Directory")
        .on_click(|_, state: &mut AppState, _| {
            let path = FileDialog::new().show_open_single_dir().unwrap();
            if let Some(path) = path {
                state.attempt_set_install_dir(path)
            }
        });
    let dv_install_dir_row = Flex::row()
        .with_child(dv_install_dir_field)
        .with_spacer(10.0)
        .with_child(dv_select_install_dir_button);

    let install_car_from_folder = Button::new("Install Car(s) from Folder")
        .on_click(|_, _, _| {
            let path = FileDialog::new().show_open_single_dir().unwrap();
            if let Some(path) = path {
                ccl::install_from_folder(path);
            }
        });
    let install_car_from_archive = Button::new("Install Car(s) from Archive")
        .on_click(|_, state: &mut AppState, _| {
            let path = FileDialog::new().show_open_single_file().unwrap();
            if let Some(path) = path {
                ccl::install_from_archive(path, &*state);
            }
        });
    let install_car_row = Flex::row()
        .with_child(install_car_from_folder)
        .with_spacer(50.0)
        .with_child(install_car_from_archive);

    let cars_header = Label::new("Installed Cars")
        .with_text_size(20.0);
    let cars_scroll = Scroll::new(Flex::column())
        .vertical();

    Flex::column()
        .with_child(dv_install_dir_header)
        .with_spacer(10.0)
        .with_child(dv_install_dir_row)
        .with_spacer(20.0)
        .with_child(install_car_row)
        .with_spacer(10.0)
        .with_child(cars_header)
        .with_spacer(10.0)
        .with_child(cars_scroll)
        .align_vertical(UnitPoint::TOP)
    // .debug_paint_layout()
}

fn car(car: Car) -> impl Widget<AppState> {
    let car_name = Label::new(car.config.identifier);
    let delete_button = Button::new("Delete");
    Flex::row()
        .with_child(car_name)
        .with_child(delete_button)
}
