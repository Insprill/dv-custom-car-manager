use druid::widget::Checkbox;
use druid::widget::{Flex, Label, List, Scroll};
use druid::LensExt;
use druid::{Widget, WidgetExt};

use crate::cmd;
use crate::controller::ccl::CclController;
use crate::data::AppState;
use crate::mods::ccl::{Car, CarConfig, CustomCarLoader};

use super::theme;
use super::widget::svg::svg;
use super::widget::{mod_header::mod_header, painter};

pub fn root() -> impl Widget<AppState> {
    let cars_scroll = Scroll::new(List::new(car).with_spacing(theme::LIST_BOX_ITEM_SPACING))
        .vertical()
        .expand()
        .lens(AppState::ccl.then(CustomCarLoader::cars))
        .padding(theme::PADDING);

    Flex::column()
        .with_child(mod_header(
            "Custom Car Loader",
            "Car",
            cmd::CCL_INSTALL_FOLDER,
            cmd::CCL_INSTALL_ARCHIVE,
        ))
        .with_default_spacer()
        .with_child(Label::new("Installed Cars").with_font(theme::HEADER_2_FONT))
        .with_flex_child(cars_scroll, 1.0)
        .expand()
        .controller(CclController)
}

fn car() -> impl Widget<Car> {
    let car_name = Label::raw()
        .with_font(theme::LIST_BOX_ITEM_FONT)
        .expand_width()
        .lens(Car::config.then(CarConfig::identifier));

    let toggle_button = Checkbox::new("")
        .lens(Car::enabled)
        .on_click(|ctx, car, _| {
            let clone = car.clone();
            let command = if car.enabled {
                cmd::CCL_DISABLE_CAR.with(clone)
            } else {
                cmd::CCL_ENABLE_CAR.with(clone)
            };
            ctx.submit_command(command)
        });

    let delete_button = Flex::row()
        .with_child(svg(include_str!("assets/icons/delete.svg")))
        .with_default_spacer()
        .with_child(
            Label::new("Delete")
                .with_font(theme::LIST_BOX_ITEM_DELETE_FONT)
                .with_text_color(theme::COLOR_TEXT_INVERTED),
        )
        .padding(4.0)
        .background(painter::solid_reactive(
            theme::COLOR_RED,
            theme::COLOR_RED_HOVER,
            theme::COLOR_RED_ACTIVE,
        ))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, car: &mut Car, _| {
            ctx.submit_command(cmd::CCL_DELETE_CAR.with(car.clone()))
        });

    Flex::row()
        .with_default_spacer()
        .with_flex_child(car_name, 1.0)
        .with_child(toggle_button)
        .with_child(delete_button)
        .expand_width()
        .background(painter::solid(theme::COLOR_RAISED_BACKGROUND))
        .rounded(theme::BORDER_RADIUS)
}
