use druid::widget::Checkbox;
use druid::widget::{Flex, Label, List, Scroll};
use druid::{FileSpec, LensExt};
use druid::{Widget, WidgetExt};

use crate::cmd;
use crate::controller::ccl::CclController;
use crate::data::AppState;
use crate::mods::ccl::{Car, CarConfig};

use super::theme;
use super::widget::painter;
use super::widget::svg::svg;

pub fn root() -> impl Widget<AppState> {
    let header = Label::new("Custom Car Loader")
        .with_font(theme::HEADER_1_FONT)
        .with_text_alignment(druid::TextAlignment::Center);

    let install_from_folder_button = Flex::row()
        .with_child(svg(include_str!("assets/icons/folder.svg")))
        .with_default_spacer()
        .with_child(Label::new("Install Car(s) from Folder").with_font(theme::LIST_BOX_ITEM_FONT))
        .padding(4.0)
        .background(painter::solid_reactive(
            theme::COLOR_BUTTON_BACKGROUND,
            theme::COLOR_BUTTON_BACKGROUND_HOVER,
            theme::COLOR_BUTTON_BACKGROUND_ACTIVE,
        ))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, _: &mut AppState, _| {
            let options = druid::FileDialogOptions::new()
                .multi_selection()
                .select_directories()
                .accept_command(cmd::CCL_INSTALL_FOLDER);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });

    let install_from_archive_button = Flex::row()
        .with_child(svg(include_str!("assets/icons/archive.svg")))
        .with_default_spacer()
        .with_child(Label::new("Install Car(s) from Archive").with_font(theme::LIST_BOX_ITEM_FONT))
        .padding(4.0)
        .background(painter::solid_reactive(
            theme::COLOR_BUTTON_BACKGROUND,
            theme::COLOR_BUTTON_BACKGROUND_HOVER,
            theme::COLOR_BUTTON_BACKGROUND_ACTIVE,
        ))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, _, _| {
            let options = druid::FileDialogOptions::new()
                .multi_selection()
                .allowed_types(vec![FileSpec {
                    name: "Archive",
                    extensions: &["zip", "rar"],
                }])
                .accept_command(cmd::CCL_INSTALL_ARCHIVE);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });

    let install_car_row = Flex::row()
        .with_child(install_from_folder_button)
        .with_default_spacer()
        .with_child(install_from_archive_button);

    let cars_header = Label::new("Installed Cars").with_font(theme::HEADER_2_FONT);
    let cars_scroll = Scroll::new(List::new(car).with_spacing(theme::LIST_BOX_ITEM_SPACING))
        .vertical()
        .expand()
        .lens(AppState::cars)
        .padding(theme::PADDING);

    Flex::column()
        .with_child(header)
        .with_default_spacer()
        .with_child(install_car_row)
        .with_default_spacer()
        .with_child(cars_header)
        .with_flex_child(cars_scroll, 1.0)
        .expand()
        .controller(CclController)
}

fn car() -> impl Widget<Car> {
    let car_name = Label::raw()
        .with_font(theme::LIST_BOX_ITEM_FONT)
        .expand_width()
        .lens(Car::config.then(CarConfig::identifier));

    let toggle_button = Checkbox::new("").lens(Car::enabled);

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
