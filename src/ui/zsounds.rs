use druid::widget::{Flex, Label, List, Scroll};
use druid::{LensExt, Widget, WidgetExt};

use crate::cmd;
use crate::controller::zsounds::ZSoundsController;
use crate::data::AppState;
use crate::mods::zsounds::{Sound, SoundGroup, ZSounds};

use super::theme::{self, COLOR_RED};
use super::widget::svg::svg;
use super::widget::{mod_header::mod_header, painter};

pub fn root() -> impl Widget<AppState> {
    let sounds_scroll = Scroll::new(
        List::new(sound_group)
            .with_spacing(theme::LIST_BOX_ITEM_SPACING)
            .padding(theme::LIST_BOX_PADDING),
    )
    .vertical()
    .expand()
    .lens(AppState::zsounds.then(ZSounds::sound_groups))
    .padding(theme::PADDING);

    Flex::column()
        .with_child(mod_header(
            "Zeibach's Sounds",
            "Sound",
            cmd::ZSOUNDS_INSTALL_FOLDER,
            cmd::ZSOUNDS_INSTALL_ARCHIVE,
        ))
        .with_default_spacer()
        .with_child(Label::new("Installed Sounds").with_font(theme::HEADER_2_FONT))
        .with_flex_child(sounds_scroll, 1.0)
        .expand()
        .controller(ZSoundsController)
}

fn sound_group() -> impl Widget<SoundGroup> {
    let name = Label::raw()
        .with_font(theme::LIST_BOX_ITEM_FONT)
        .expand_width()
        .lens(SoundGroup::name);

    let delete_button = Flex::row()
        .with_child(svg(include_str!("assets/icons/delete.svg")))
        .with_default_spacer()
        .with_child(
            Label::new("Delete")
                .with_font(theme::LIST_BOX_ITEM_DELETE_FONT)
                .with_text_color(theme::COLOR_TEXT_INVERTED)
                .env_scope(|env, _| {
                    env.set(
                        druid::theme::DISABLED_TEXT_COLOR,
                        env.get(theme::COLOR_TEXT_INVERTED),
                    )
                }),
        )
        .padding(4.0)
        .background(painter::solid_reactive(COLOR_RED))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, group: &mut SoundGroup, _| {
            ctx.submit_command(cmd::ZSOUNDS_DELETE_SOUNDGROUP.with(group.clone()))
        })
        .disabled_if(|group, _| group.is_root);

    let group_row = Flex::row()
        .with_default_spacer()
        .with_flex_child(name, 1.0)
        .with_child(delete_button)
        .expand_width()
        .background(painter::solid(theme::COLOR_RAISED_BACKGROUND))
        .rounded(theme::BORDER_RADIUS);

    Flex::column()
        .with_child(group_row)
        .with_spacer(theme::LIST_BOX_ITEM_SPACING)
        .with_child(
            Flex::row().with_spacer(theme::SPACER).with_flex_child(
                List::new(sound)
                    .with_spacing(theme::LIST_BOX_ITEM_SPACING)
                    .lens(SoundGroup::sounds),
                1.0,
            ),
        )
}

pub fn sound() -> impl Widget<Sound> {
    let name = Label::raw()
        .with_font(theme::LIST_BOX_ITEM_FONT)
        .expand_width()
        .lens(Sound::name);

    let play_button = Flex::row()
        .with_child(svg(include_str!("assets/icons/play_sound.svg")).fix_size(20.0, 20.0))
        .with_default_spacer()
        .with_child(
            Label::new("Play")
                .with_font(theme::LIST_BOX_ITEM_DELETE_FONT)
                .with_text_color(theme::COLOR_TEXT_INVERTED),
        )
        .padding(4.0)
        .background(painter::solid_reactive(theme::COLOR_GREEN))
        .rounded(theme::BORDER_RADIUS)
        .on_click(|ctx, sound: &mut Sound, _| {
            ctx.submit_command(cmd::ZSOUNDS_PLAY_SOUND.with(sound.clone()))
        });

    Flex::row()
        .with_default_spacer()
        .with_flex_child(name, 1.0)
        .with_child(play_button)
        .expand_width()
        .background(painter::solid(theme::COLOR_RAISED_BACKGROUND))
        .rounded(theme::BORDER_RADIUS)
}
