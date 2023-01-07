use druid::{widget::Controller, Env, Event, EventCtx, Widget};
use log::error;
use rodio::OutputStream;
use std::path::Path;
use std::thread;
use std::{error::Error, fs::File};

use crate::{cmd, data::AppState, mods::Installable};

pub struct ZSoundsController;

impl<W> Controller<AppState, W> for ZSoundsController
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
            Event::Command(cmd) if cmd.is(cmd::ZSOUNDS_DELETE_SOUNDGROUP) => {
                let group = cmd.get_unchecked(cmd::ZSOUNDS_DELETE_SOUNDGROUP);
                group.delete().unwrap_or_else(|err| {
                    error!("Failed to delete sound! {}", err.to_string());
                    todo!("alert")
                });
                state.zsounds.update(&state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::ZSOUNDS_INSTALL_ARCHIVE) => {
                let file_info = cmd.get_unchecked(cmd::ZSOUNDS_INSTALL_ARCHIVE);
                state
                    .zsounds
                    .install_from_archive(&file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::ZSOUNDS_INSTALL_FOLDER) => {
                let file_info = cmd.get_unchecked(cmd::ZSOUNDS_INSTALL_FOLDER);
                state
                    .zsounds
                    .install_from_folder(&file_info.path, &state.config);
            }
            Event::Command(cmd) if cmd.is(cmd::ZSOUNDS_PLAY_SOUND) => {
                let sound = cmd.get_unchecked(cmd::ZSOUNDS_PLAY_SOUND);
                match play_sound(&sound.path) {
                    Ok(_) => {}
                    Err(err) => {
                        error!(
                            "Failed to play sound {} ({:?}): {}",
                            sound.filename, sound.path, err
                        );
                        //todo: alert
                    }
                }
                todo!("Play sounds properly, update button when playing to allow the sound to be stopped");
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}

fn play_sound(path: &Path) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = stream_handle.play_once(File::open(path)?)?;
    sink.set_volume(0.5);
    thread::spawn(move || {
        sink.sleep_until_end();
    });
    Ok(())
}
