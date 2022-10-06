use druid::widget::{Button, Flex, Label, List, Scroll, TextBox};
use druid::LensExt;
use druid::{UnitPoint, Widget, WidgetExt, WindowDesc};
use native_dialog::FileDialog;

use crate::controller::ccl::CclController;
use crate::data::AppState;
use crate::mods::ccl::{self, Car, CarConfig};
use crate::{cmd, Config};

pub fn main_window() -> WindowDesc<AppState> {
    WindowDesc::new(root())
        .title("Custom Car Manager")
        .window_size((600.0, 400.0))
        .resizable(false)
}

fn root() -> impl Widget<AppState> {
    Flex::column()
        .with_child(dv_install_dir())
        .with_spacer(10.0)
        .with_child(cars())
        .align_vertical(UnitPoint::TOP)
    // .debug_paint_layout()
}

fn dv_install_dir() -> impl Widget<AppState> {
    let dv_install_dir_header = Label::new("Derail Valley Install Directory").with_text_size(22.0);

    let dv_install_dir_field = TextBox::new()
        .fix_width(375.0)
        .lens(AppState::config.then(Config::dv_install_dir));
    let dv_select_install_dir_button =
        Button::new("Select Install Directory").on_click(|_, state: &mut AppState, _| {
            let path = FileDialog::new().show_open_single_dir().unwrap();
            if let Some(path) = path {
                state.attempt_set_install_dir(path)
            }
        });
    let dv_install_dir_row = Flex::row()
        .with_child(dv_install_dir_field)
        .with_spacer(10.0)
        .with_child(dv_select_install_dir_button);

    Flex::column()
        .with_child(dv_install_dir_header)
        .with_spacer(10.0)
        .with_child(dv_install_dir_row)
}

fn cars() -> impl Widget<AppState> {
    let install_car_from_folder =
        Button::new("Install Car(s) from Folder").on_click(|_, state: &mut AppState, _| {
            let path = FileDialog::new().show_open_single_dir().unwrap();
            if let Some(path) = path {
                ccl::install_from_folder(&path, state);
            }
        });
    let install_car_from_archive =
        Button::new("Install Car(s) from Archive").on_click(|_, state: &mut AppState, _| {
            let path = FileDialog::new().show_open_single_file().unwrap();
            if let Some(path) = path {
                ccl::install_from_archive(&path, state);
            }
        });
    let install_car_row = Flex::row()
        .with_child(install_car_from_folder)
        .with_spacer(50.0)
        .with_child(install_car_from_archive);

    let cars_header = Label::new("Installed Cars").with_text_size(22.0);
    let cars_scroll = Scroll::new(List::new(|| car()))
        .fix_height(200.0)
        .lens(AppState::cars);

    Flex::column()
        .with_child(install_car_row)
        .with_spacer(15.0)
        .with_child(cars_header)
        .with_spacer(10.0)
        .with_child(cars_scroll)
        .controller(CclController)
}

fn car() -> impl Widget<Car> {
    let car_name = Label::raw()
        .with_text_size(16.0)
        .fix_width(215.0)
        .lens(Car::config.then(CarConfig::identifier));
    let open_dir_button = Button::new("Open Folder").align_right();
    let delete_button = Button::new("Delete")
        .on_click(|ctx, car: &mut Car, _| ctx.submit_command(cmd::DELETE_CAR.with(car.clone())))
        .align_right();

    Flex::row()
        .with_child(car_name)
        .with_child(open_dir_button)
        .with_child(delete_button)
        .fix_width(400.0)
}
