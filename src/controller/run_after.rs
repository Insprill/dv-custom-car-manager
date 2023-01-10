use std::time::Duration;

use druid::{
    widget::Controller, Data, Env, Event, EventCtx, LifeCycle, LifeCycleCtx, TimerToken, Widget,
};

type DelayFunc<T> = Box<dyn Fn(&mut EventCtx, &mut T, &Env)>;
type DurationFunc<T> = Box<dyn Fn(&T, &Env) -> Duration>;

pub struct RunAfter<T> {
    dur_func: DurationFunc<T>,
    timer: TimerToken,
    func: DelayFunc<T>,
}

impl<T> RunAfter<T> {
    pub fn new(
        duration_func: impl Fn(&T, &Env) -> Duration + 'static,
        func: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> Self {
        Self {
            dur_func: Box::new(duration_func),
            timer: TimerToken::INVALID,
            func: Box::new(func),
        }
    }
}

impl<T, W> Controller<T, W> for RunAfter<T>
where
    T: Data,
    W: Widget<T>,
{
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Timer(token) if token == &self.timer => {
                (self.func)(ctx, data, env);
                self.timer = TimerToken::INVALID;
            }
            _ => child.event(ctx, event, data, env),
        }
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.timer = ctx.request_timer((self.dur_func)(data, env));
        }
        child.lifecycle(ctx, event, data, env)
    }
}
