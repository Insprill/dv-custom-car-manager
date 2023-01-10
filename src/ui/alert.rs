use druid::widget::{Flex, Label, LineBreaking, List};
use druid::{EventCtx, Widget, WidgetExt};
use std::time::Duration;
use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use crate::cmd;
use crate::data::AppState;

use druid::{Data, Lens};

use super::theme;
use super::widget::{painter, CustomWidgetExt};

static ALERT_ID: AtomicU32 = AtomicU32::new(0);

fn next_id() -> u32 {
    ALERT_ID.fetch_add(1, Ordering::SeqCst)
}

#[derive(Clone, Data, Lens)]
pub struct Alert {
    pub id: u32,
    pub message: Arc<str>,
    pub style: AlertStyle,
}

impl Alert {
    fn new(message: impl Display, style: AlertStyle) -> Self {
        Alert {
            id: next_id(),
            message: message.to_string().into(),
            style,
        }
    }

    pub fn error(ctx: &mut EventCtx, message: impl Display) {
        Self::new(message, AlertStyle::Error).send(ctx)
    }

    pub fn warn(ctx: &mut EventCtx, message: impl Display) {
        Self::new(message, AlertStyle::Warn).send(ctx)
    }

    pub fn info(ctx: &mut EventCtx, message: impl Display) {
        Self::new(message, AlertStyle::Info).send(ctx)
    }

    pub fn send(self, ctx: &mut EventCtx) {
        ctx.submit_command(cmd::ALERT.with(self))
    }
}

#[derive(Clone, Data, Eq, PartialEq)]
pub enum AlertStyle {
    Error,
    Warn,
    Info,
}

impl AlertStyle {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Error => theme::ALERT_ERROR_DURATION,
            Self::Warn => theme::ALERT_WARN_DURATION,
            Self::Info => theme::ALERT_INFO_DURATION,
        }
    }
}

pub fn alert_box() -> impl Widget<AppState> {
    List::new(|| {
        Flex::row()
            .with_flex_child(
                Label::raw()
                    .with_font(theme::LIST_BOX_ITEM_FONT)
                    .with_text_color(theme::COLOR_TEXT_INVERTED)
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .lens(Alert::message)
                    .padding(theme::ALERT_TEXT_PADDING),
                1.0,
            )
            .background(painter::dyn_solid_reactive(
                |alert: &Alert, _| match alert.style {
                    AlertStyle::Error => theme::COLOR_RED,
                    AlertStyle::Warn => theme::COLOR_ORANGE,
                    AlertStyle::Info => theme::COLOR_GREEN,
                },
            ))
            .rounded(theme::BORDER_RADIUS)
            .on_click(|ctx, alert: &mut Alert, _| {
                ctx.submit_command(cmd::DISMISS_ALERT.with(alert.id));
            })
            .run_after(
                |alert: &Alert, _| alert.style.duration(),
                |ctx, alert: &mut Alert, _| {
                    ctx.submit_command(cmd::DISMISS_ALERT.with(alert.id));
                },
            )
    })
    .with_spacing(theme::ALERT_SPACING)
    .padding(theme::PADDING)
    .lens(AppState::alerts)
    .on_command(cmd::ALERT, |_, alert, state| state.alert(alert))
    .on_command(cmd::DISMISS_ALERT, |_, &id, state| {
        state.dismiss_alert(id);
    })
}
