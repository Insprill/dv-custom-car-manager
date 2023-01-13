use druid::{widget::Controller, Env, Event, EventCtx, Widget};

use crate::{cmd, data::AppState, ui::alert::Alert};

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
                    .config
                    .derail_valley
                    .attempt_set_install_dir(ctx, &file_info.path)
                    .unwrap_or_else(|err| {
                        Alert::error(
                            ctx,
                            format!(
                                "Failed to set DV installation directory to {:?}: {:?}",
                                file_info.path, err
                            ),
                        );
                        false
                    });
                if !set {
                    Alert::error(
                        ctx,
                        format!("Invalid DV installation directory {:?}", file_info.path),
                    );
                    return;
                }
                state.update_all(ctx)
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
