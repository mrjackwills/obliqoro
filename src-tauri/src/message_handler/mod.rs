use std::path::PathBuf;
use tauri::{menu::Menu, AppHandle, Wry};
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::Level;
use tracing_subscriber::{fmt as t_fmt, prelude::__tracing_subscriber_SubscriberExt};

use crate::{
    app_error::AppError,
    application_state::ApplicationState,
    db::{self, ModelSettings},
};

mod messages;

pub use messages::*;

/// Setup tracing - warning this can write huge amounts to disk
#[cfg(debug_assertions)]
fn setup_tracing(app_dir: &PathBuf) -> Result<(), AppError> {
    let logfile = tracing_appender::rolling::never(app_dir, "obliqoro.log");

    let log_fmt = t_fmt::Layer::default()
        .json()
        .flatten_event(true)
        .with_writer(logfile);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(true)
            .with_line_number(true)
            .with_max_level(Level::DEBUG)
            .finish()
            .with(log_fmt),
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
        }
    }
}

/// Setup tracing
#[cfg(not(debug_assertions))]
fn setup_tracing(_app_dir: &PathBuf) -> Result<(), AppError> {
    let level = Level::INFO;
    let log_fmt = t_fmt::Layer::default().json().flatten_event(true);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(false)
            .with_line_number(true)
            .with_max_level(level)
            .finish()
            .with(log_fmt),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
        }
    }
}

pub struct MessageHandler;

impl MessageHandler {
    /// Handle HeartBeat messages
    fn handle_heartbeat(msg: MsgHB, state: &mut ApplicationState) {
        match msg {
            MsgHB::Abort => state.heartbeat_abort(),
            MsgHB::Update(handle) => {
                state.heartbeat_update(handle);
            }
            MsgHB::OnHeartbeat(cpu_usage) => {
                state.on_heartbeat(cpu_usage);
            }
            MsgHB::UpdateTimer => {
                state.update_timer_check();
            }
        }
    }
    /// Start the message handling loop in it's own tokio theread
    pub async fn start_message_loop(mut state: ApplicationState, mut rx: Receiver<MsgI>) {
        while let Ok(msg) = rx.recv().await {
            match msg {
                MsgI::Break(break_message) => {
                    state.handle_break(break_message);
                }

                MsgI::HeartBeat(msg_hb) => Self::handle_heartbeat(msg_hb, &mut state),

                MsgI::OpenLocation => {
                    open::that(state.get_data_location()).ok();
                }

                MsgI::Pause => {
                    let paused = state.toggle_pause();
                    state.update_menu_pause(paused);
                    state.update_icon(paused);
                    // set_icon(state.get_app_handle(), paused);
                    state.send(MsgI::ToFrontEnd(MsgFE::Paused(paused)));
                }

                MsgI::ResetSettings => {
                    if let Err(e) = state.reset_settings().await {
                        tracing::error!("{:#?}", e);
                        state.send(MsgI::ToFrontEnd(MsgFE::Error));
                    }
                    state.update_menu_all();
                }

                MsgI::ResetTimer => {
                    state.reset_timer();
                }

                MsgI::SetSetting(frontend_state) => {
                    if let Err(e) = state.update_settings(frontend_state).await {
                        tracing::error!("{:#?}", e);
                        state.send(MsgI::ToFrontEnd(MsgFE::Error));
                    }
                    state.update_menu_all();
                }

                MsgI::ToFrontEnd(to_front_end) => {
                    state.emit_to_frontend(to_front_end);
                }
                MsgI::UpdateMenuTimer => state.update_menu_all(),

                MsgI::UpdatePause(pause) => {
                    state.update_pause_after_break(pause);
                }
                MsgI::Window(window_visibility) => {
                    state.handle_visibility(window_visibility);
                }
            }
        }
    }

    /// Create a MessageHandler instance, which will container application state
    /// Will spawn the message handling in its own tokio thread
    pub fn init(
        rx: Receiver<MsgI>,
        sx: Sender<MsgI>,
        tmp_rx: tokio::sync::oneshot::Receiver<(AppHandle, PathBuf, Menu<Wry>)>,
    ) {
        tokio::spawn(async move {
            let Ok((app_handle, data_location, system_tray_menu)) = tmp_rx.await else {
                // todo print err
                std::process::exit(1)
            };
            if !std::fs::exists(&data_location).unwrap_or_default()
                && std::fs::create_dir(&data_location).is_err()
            {
                // todo print err
                std::process::exit(1)
            }
            if setup_tracing(&data_location).is_err() {
                // todo print err
                std::process::exit(1)
            }

            let Ok(sqlite) = db::init_db(&data_location).await else {
                // todo print err
                std::process::exit(1)
            };
            let Ok(settings) = ModelSettings::init(&sqlite).await else {
                // todo print err
                std::process::exit(1)
            };

            let state = ApplicationState::new(
                app_handle,
                data_location,
                sx,
                settings,
                sqlite,
                system_tray_menu,
            );
            Self::start_message_loop(state, rx).await;
        });
    }
}
