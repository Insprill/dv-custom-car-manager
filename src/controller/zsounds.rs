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
                let audio_file_path = sound.path.to_path_buf();
                let audio_file_name = sound.filename.clone();
                let volume = state.config.volume;
                thread::spawn(move || {
                    match play_sound(&audio_file_path, volume) {
                        Ok(_) => {}
                        Err(err) => {
                            error!(
                                "Failed to play sound {} ({:?}): {}",
                                audio_file_name, audio_file_path, err
                            );
                            //todo: alert
                        }
                    }
                });
            }
            _ => child.event(ctx, event, state, env),
        }
    }
}

fn play_sound(path: &Path, volume: f64) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = stream_handle.play_once(File::open(path)?)?;
    sink.set_volume(volume as f32);
    sink.sleep_until_end();
    Ok(())
}
