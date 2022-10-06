use druid::{widget::Controller, Env, Event, EventCtx, Widget};
use log::error;

use crate::{cmd, data::AppState};

pub struct CclController;

impl<W> Controller<AppState, W> for CclController
where
    W: Widget<AppState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        state: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(cmd::DELETE_CAR) => {
                let car = cmd.get_unchecked(cmd::DELETE_CAR);
                match car.delete() {
                    Err(err) => {
                        error!("Failed to delete car! {}", err.to_string())
                        //TODO: error dialog
                    }
                    Ok(_) => {
                        state.update_cars();
                    }
                }
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
