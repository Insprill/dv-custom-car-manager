use std::time::Duration;

use druid::{widget::ControllerHost, Data, Env, EventCtx, Selector, Widget, WidgetExt};

use crate::controller::{on_command::OnCommand, run_after::RunAfter};

pub mod mod_header;
pub mod overlay;
pub mod painter;
pub mod svg;

impl<T: Data, W: Widget<T> + 'static> CustomWidgetExt<T> for W {}

pub trait CustomWidgetExt<T: Data>: Widget<T> + Sized + 'static {
    fn on_command<U, F>(
        self,
        selector: Selector<U>,
        func: F,
    ) -> ControllerHost<Self, OnCommand<U, F>>
    where
        U: 'static,
        F: Fn(&mut EventCtx, &U, &mut T),
    {
        self.controller(OnCommand::new(selector, func))
    }

    fn run_after(
        self,
        duration_func: impl Fn(&T, &Env) -> Duration + 'static,
        func: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, RunAfter<T>> {
        self.controller(RunAfter::new(duration_func, func))
    }
}
