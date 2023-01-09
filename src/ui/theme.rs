use druid::{
    theme, Color, Env, FontDescriptor, FontFamily, FontWeight, Insets, Key, RoundedRectRadii,
};

use crate::data::AppState;

pub const COLOR_BACKGROUND: Key<Color> = Key::new("app.theme.color.background");
pub const COLOR_RAISED_BACKGROUND: Key<Color> = Key::new("app.theme.color.raised-background");
pub const COLOR_RAISED_BACKGROUND_HOVER: Key<Color> =
    Key::new("app.theme.color.raised-background.hover");
pub const COLOR_RED: Key<Color> = Key::new("app.theme.color.red");
pub const COLOR_RED_HOVER: Key<Color> = Key::new("app.theme.color.red.hover");
pub const COLOR_RED_ACTIVE: Key<Color> = Key::new("app.theme.color.red.active");
pub const COLOR_GREEN: Key<Color> = Key::new("app.theme.color.green");
pub const COLOR_GREEN_HOVER: Key<Color> = Key::new("app.theme.color.green.color");
pub const COLOR_GREEN_ACTIVE: Key<Color> = Key::new("app.theme.color.green.active");
pub const COLOR_TEXT_INVERTED: Key<Color> = Key::new("app.theme.color.text.dark");
pub const COLOR_BUTTON_BACKGROUND: Key<Color> = Key::new("app.theme.color.button.background");
pub const COLOR_BUTTON_BACKGROUND_HOVER: Key<Color> =
    Key::new("app.theme.color.button.background.hover");
pub const COLOR_BUTTON_BACKGROUND_ACTIVE: Key<Color> =
    Key::new("app.theme.color.button.background.active");

pub const GUTTER_PADDING: Key<f64> = Key::new("app.theme.gutter.background.vertical-padding");
pub const GUTTER_WIDTH: Key<f64> = Key::new("app.theme.gutter.width");
pub const GUTTER_ICON_SPACING: Key<f64> = Key::new("app.theme.gutter.icon.spacing");
pub const GUTTER_ICON_PADDING: Key<Insets> = Key::new("app.theme.gutter.icon.padding");

pub const HEADER_1_FONT: Key<FontDescriptor> = Key::new("app.theme.header.1");
pub const HEADER_2_FONT: Key<FontDescriptor> = Key::new("app.theme.header.2");
pub const HEADER_3_FONT: Key<FontDescriptor> = Key::new("app.theme.header.3");

pub const BORDER_RADIUS: Key<RoundedRectRadii> = Key::new("app.theme.border-radius");
pub const SPACER: Key<f64> = Key::new("app.theme.spacer");
pub const PADDING: Key<Insets> = Key::new("app.theme.padding");

pub const LIST_BOX_PADDING: Key<Insets> = Key::new("app.theme.list-box.padding");
pub const LIST_BOX_ITEM_SPACING: Key<f64> = Key::new("app.theme.list-box.item.spacing");
pub const LIST_BOX_ITEM_INDENT: Key<f64> = Key::new("app.theme.list-box.item.indent");
pub const LIST_BOX_ITEM_FONT: Key<FontDescriptor> = Key::new("app.theme.list-box.item.font");
pub const LIST_BOX_ITEM_DELETE_FONT: Key<FontDescriptor> =
    Key::new("app.theme.list-box.delete.item.font");

pub fn apply_theme(env: &mut Env, _state: &AppState) {
    apply_dark_theme(env);
    apply_style(env);
}

fn apply_dark_theme(env: &mut Env) {
    env.set(COLOR_BACKGROUND, hex("#16181c"));
    env.set(theme::WINDOW_BACKGROUND_COLOR, env.get(COLOR_BACKGROUND));
    env.set(COLOR_RAISED_BACKGROUND, hex("#26292f"));
    env.set(COLOR_RAISED_BACKGROUND_HOVER, hex("#464646"));

    env.set(COLOR_BUTTON_BACKGROUND, hex("#434956"));
    env.set(COLOR_BUTTON_BACKGROUND_HOVER, hex("#494f58"));
    env.set(COLOR_BUTTON_BACKGROUND_ACTIVE, hex("#616570"));

    env.set(theme::TEXT_COLOR, hex("#b0bac5"));
    env.set(COLOR_TEXT_INVERTED, env.get(COLOR_BACKGROUND));

    env.set(COLOR_RED, hex("#ff496e"));
    env.set(COLOR_RED_HOVER, hex("#d93e5d"));
    env.set(COLOR_RED_ACTIVE, hex("#ce3b59"));

    env.set(COLOR_GREEN, hex("#1bd96a"));
    env.set(COLOR_GREEN_HOVER, hex("#17b85a"));
    env.set(COLOR_GREEN_ACTIVE, hex("#16ae55"));

    fn hex(hex: &str) -> Color {
        Color::from_hex_str(hex).unwrap()
    }
}

fn apply_style(env: &mut Env) {
    env.set(BORDER_RADIUS, 6.0);
    env.set(theme::BUTTON_BORDER_RADIUS, env.get(BORDER_RADIUS));
    env.set(SPACER, 12.0);
    env.set(PADDING, Insets::uniform(10.0));

    env.set(GUTTER_PADDING, 6.0);
    env.set(GUTTER_WIDTH, 48.0);
    env.set(GUTTER_ICON_PADDING, Insets::uniform(2.0));
    env.set(GUTTER_ICON_SPACING, 10.0);

    env.set(
        HEADER_1_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF)
            .with_weight(FontWeight::BOLD)
            .with_size(42.0),
    );
    env.set(
        HEADER_2_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF)
            .with_weight(FontWeight::SEMI_BOLD)
            .with_size(24.0),
    );
    env.set(
        HEADER_3_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF)
            .with_weight(FontWeight::MEDIUM)
            .with_size(18.0),
    );

    env.set(
        LIST_BOX_ITEM_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF).with_size(18.0),
    );
    env.set(
        LIST_BOX_ITEM_DELETE_FONT,
        FontDescriptor::new(FontFamily::SANS_SERIF)
            .with_weight(FontWeight::SEMI_BOLD)
            .with_size(16.0),
    );
    env.set(LIST_BOX_ITEM_SPACING, 4.0);
    env.set(LIST_BOX_ITEM_INDENT, 18.0);
    env.set(LIST_BOX_PADDING, Insets::new(0.0, 0.0, 14.0, 0.0));

    env.set(theme::SCROLLBAR_PAD, 0.1);
}
