use druid::{widget::Controller, Env, Event, EventCtx, Widget};

use crate::{cmd, data::AppState, mods::Installable, ui::alert::AlertStyle};

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
            Event::Command(cmd) if cmd.is(cmd::CCL_DELETE_CAR) => {
                let car = cmd.get_unchecked(cmd::CCL_DELETE_CAR);
                car.delete().unwrap_or_else(|err| {
                    let car = cmd.get_unchecked(cmd::CCL_DELETE_CAR);
                    state.alert(
                        format!(
                            "Failed to delete car at {:?}!\nError: {:?}",
                            car.directory, err
                        ),
                        AlertStyle::Error,
                    );
                });
                state.ccl.update(&state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_INSTALL_ARCHIVE) => {
                let file_info = cmd.get_unchecked(cmd::CCL_INSTALL_ARCHIVE);
                state
                    .ccl
                    .install_from_archive(&file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_INSTALL_FOLDER) => {
                let file_info = cmd.get_unchecked(cmd::CCL_INSTALL_FOLDER);
                state
                    .ccl
                    .install_from_folder(&file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_ENABLE_CAR) => {
                let car = cmd.get_unchecked(cmd::CCL_ENABLE_CAR);
                car.enable(state);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_DISABLE_CAR) => {
                let car = cmd.get_unchecked(cmd::CCL_DISABLE_CAR);
                car.disable(state);
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
