pub mod ccl;
pub mod dv;
pub mod on_command;
pub mod run_after;
pub mod zsounds;

use druid::{widget::Controller, Env, Event, EventCtx, Widget};
use sysinfo::{ProcessRefreshKind, System, SystemExt};

use crate::{
    cmd,
    data::{nav::Nav, AppState},
    ui::alert::Alert,
};

pub struct RootController;

impl<W> Controller<AppState, W> for RootController
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
            Event::Command(cmd) if cmd.is(cmd::INIT) => {
                ctx.submit_command(cmd::DV_VALIDATE_INSTALL_DIR);
                ctx.submit_command(cmd::DV_CHECK_RUNNING);
            }
            Event::Command(cmd) if cmd.is(cmd::DV_CHECK_RUNNING) => {
                let mut sys = System::new();
                sys.refresh_processes_specifics(ProcessRefreshKind::default());
                if sys.processes_by_name("DerailValley").count() > 0 {
                    Alert::warn(ctx, "Derail Valley is running! Making any modifications to mods while the game is running may lead to undefined behaviour. Proceed at your own risk!")
                }
            }
            Event::Command(cmd) if cmd.is(cmd::NAV_TOGGLE) => {
                state.can_navigate = *cmd.get_unchecked(cmd::NAV_TOGGLE);
            }
            Event::Command(cmd) if cmd.is(cmd::CONFIG_SAVE) => {
                state.config.save();
            }
            Event::Command(cmd) if cmd.is(cmd::DV_VALIDATE_INSTALL_DIR) => {
                let mut is_set_and_valid = true;
                if !state.config.derail_valley.has_install_dir() {
                    Alert::error(ctx, "Please set the installation directory of Derail Valley!\nThis is where the executable is located, e.g. C:/Program Files (x86)/Steam/steamapps/common/Derail Valley/");
                    state.nav = Nav::Settings;
                    is_set_and_valid = false;
                } else if !state.config.derail_valley.is_install_dir_valid(ctx) {
                    Alert::error(
                        ctx,
                        "Invalid Derail Valley installation directory! Please set it again.",
                    );
                    state.config.derail_valley.install_dir = String::new();
                    state.config.save();
                    is_set_and_valid = false;
                }
                state.can_navigate = is_set_and_valid;
                if is_set_and_valid {
                    state.update_all(ctx);
                } else {
                    state.nav = Nav::Settings;
                }
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}
