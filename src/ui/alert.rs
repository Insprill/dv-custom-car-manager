use druid::widget::{Flex, Label, LineBreaking, List};
use druid::{Widget, WidgetExt};
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
    pub fn new(message: impl Display, style: AlertStyle) -> Self {
        Alert {
            id: next_id(),
            message: message.to_string().into(),
            style,
        }
    }
}

#[derive(Clone, Data, Eq, PartialEq)]
pub enum AlertStyle {
    Error,
    Info,
}

impl AlertStyle {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Error => theme::ALERT_ERROR_DURATION,
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
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .lens(Alert::message)
                    .padding(theme::ALERT_TEXT_PADDING),
                1.0,
            )
            .background(painter::dyn_solid_reactive(
                |alert: &Alert, _| match alert.style {
                    AlertStyle::Error => theme::COLOR_RED,
                    AlertStyle::Info => theme::COLOR_BUTTON,
                },
            ))
            .rounded(theme::BORDER_RADIUS)
            .env_scope(|env, alert: &Alert| {
                if let AlertStyle::Error = alert.style {
                    env.set(
                        druid::theme::TEXT_COLOR,
                        env.get(theme::COLOR_TEXT_INVERTED),
                    );
                }
            })
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
    .on_command(cmd::DISMISS_ALERT, |_, &id, state| {
        state.dismiss_alert(id);
    })
}
