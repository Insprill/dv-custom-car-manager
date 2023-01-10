use druid::widget::{Flex, Label};
use druid::{FileInfo, FileSpec, Widget};
use druid::{Selector, WidgetExt};

use crate::data::AppState;
use crate::ui::theme;

use super::painter;
use super::svg::svg;

pub fn mod_header(
    mod_name: &str,
    install_name: &str,
    folder_install_cmd: Selector<FileInfo>,
    archive_install_cmd: Selector<FileInfo>,
) -> impl Widget<AppState> {
    let header = Label::new(mod_name)
        .with_font(theme::HEADER_1_FONT)
        .with_text_alignment(druid::TextAlignment::Center);

    let install_from_folder_button = Flex::row()
        .with_child(svg(include_str!("../assets/icons/folder.svg")))
        .with_default_spacer()
        .with_child(
            Label::new(format!("Install {}(s) from Folder", install_name))
                .with_font(theme::LIST_BOX_ITEM_FONT),
        )
        .padding(4.0)
        .background(painter::solid_reactive(theme::COLOR_BUTTON))
        .rounded(theme::BORDER_RADIUS)
        .on_click(move |ctx, _, _| {
            let options = druid::FileDialogOptions::new()
                .multi_selection()
                .select_directories()
                .accept_command(folder_install_cmd);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });

    let install_from_archive_button = Flex::row()
        .with_child(svg(include_str!("../assets/icons/archive.svg")))
        .with_default_spacer()
        .with_child(Label::new("Install Car(s) from Archive").with_font(theme::LIST_BOX_ITEM_FONT))
        .padding(4.0)
        .background(painter::solid_reactive(theme::COLOR_BUTTON))
        .rounded(theme::BORDER_RADIUS)
        .on_click(move |ctx, _, _| {
            let options = druid::FileDialogOptions::new()
                .multi_selection()
                .allowed_types(vec![FileSpec {
                    name: "Archive",
                    extensions: &["zip", "rar"],
                }])
                .accept_command(archive_install_cmd);
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options))
        });

    Flex::column()
        .with_child(header)
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(install_from_folder_button)
                .with_default_spacer()
                .with_child(install_from_archive_button),
        )
}
