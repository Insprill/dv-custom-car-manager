use druid::widget::Checkbox;
use druid::widget::{Flex, Label, List, Scroll};
use druid::LensExt;
use druid::{Widget, WidgetExt};

use crate::cmd;
use crate::controller::ccl::CclController;
use crate::data::AppState;
use crate::mods::ccl::{Car, CarConfig, CustomCarLoader};
use crate::mods::skinmanager::Skin;

use super::theme;
use super::widget::svg::svg;
use super::widget::{mod_header::mod_header, painter};

pub fn root() -> impl Widget<AppState> {
    Flex::column()
        .with_child(mod_header(
            "Skin Manager",
            "Skin",
            cmd::SKINMANAGER_INSTALL_FOLDER,
            cmd::SKINMANAGER_INSTALL_ARCHIVE,
        ))
        .with_default_spacer()
        .with_child(Label::new("Installed Skins").with_font(theme::HEADER_2_FONT))
        .with_flex_child(
            Scroll::new(
                List::new(skin)
                    .with_spacing(theme::LIST_BOX_ITEM_SPACING)
                    .padding(theme::LIST_BOX_PADDING),
            )
            .vertical()
            .expand()
            .lens(AppState::skinmanager.then(SkinManager::skins))
            .padding(theme::PADDING),
            1.0,
        )
        .expand()
        .controller(CclController)
}

fn skin() -> impl Widget<Skin> {}
