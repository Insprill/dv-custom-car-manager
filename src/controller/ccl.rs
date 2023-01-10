use druid::{widget::Controller, Env, Event, EventCtx, Widget};

use crate::{cmd, data::AppState, mods::Installable, ui::alert::Alert};

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
                    Alert::error(
                        ctx,
                        format!(
                            "Failed to delete car at {:?}!\nError: {:?}",
                            car.directory, err
                        ),
                    );
                });
                Alert::info(ctx, format!("Deleted {}", car.config.identifier));
                state.ccl.update(ctx, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_INSTALL_ARCHIVE) => {
                let file_info = cmd.get_unchecked(cmd::CCL_INSTALL_ARCHIVE);
                state
                    .ccl
                    .install_from_archive(ctx, &file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_INSTALL_FOLDER) => {
                let file_info = cmd.get_unchecked(cmd::CCL_INSTALL_FOLDER);
                state
                    .ccl
                    .install_from_folder(ctx, &file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_ENABLE_CAR) => {
                let car = cmd.get_unchecked(cmd::CCL_ENABLE_CAR);
                car.enable(ctx, state);
            }
            Event::Command(cmd) if cmd.is(cmd::CCL_DISABLE_CAR) => {
                let car = cmd.get_unchecked(cmd::CCL_DISABLE_CAR);
                car.disable(ctx, state);
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
