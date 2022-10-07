use druid::{widget::Controller, Env, Event, EventCtx, Widget};

use crate::{cmd, data::AppState};

pub struct DvController;

impl<W> Controller<AppState, W> for DvController
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
            Event::Command(cmd) if cmd.is(cmd::DV_SET_INSTALL_DIR) => {
                let file_info = cmd.get_unchecked(cmd::DV_SET_INSTALL_DIR);
                let set = state
                    .attempt_set_install_dir(&file_info.path)
                    .unwrap_or_else(|err| {
                        panic!("TODO: failed to set install dir: {}", err.to_string())
                    });
                if !set {
                    panic!("TODO: invalid dir")
                }
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
