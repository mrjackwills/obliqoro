use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tracing::error;
mod menu;
mod messages;
mod st;
mod application_state;

pub use st::MessageHandler;
pub use messages::*;

use crate::{
    app_error::AppError,
    application_state::ApplicationState,
    db::ModelSettings,
    heartbeat::heartbeat_process,
    request_handlers::{FrontEndState, ShowTimer, ToFrontEnd},
    system_tray::{change_menu_entry_status, set_icon, MenuEntry},
    window_action::WindowAction,
    MAIN_WINDOW,
};
use tokio::sync::broadcast::{Receiver, Sender};

// // Update the taskbar to display how many sessions before next long break,
// // and send internal message, to send message to front end to update settings in pinia
// fn update_menu_session_number(state: &Arc<Mutex<ApplicationState>>, sx: &Sender<InternalMessage>) {
//     let title = state.lock().get_sessions_before_long_title();
//     state
//         .lock()
//         .system_tray_menu
//         .get(MenuEntry::Session.get_id())
//         .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
//     sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::SessionsBeforeLong))
//         .ok();
// }

// /// Update the systemtray next break in text, and emit to frontend to next break timer
// fn update_menu_next_break(state: &Arc<Mutex<ApplicationState>>, sx: &Sender<InternalMessage>) {
//     let title = state.lock().get_next_break_title();
//     state
//         .lock()
//         .system_tray_menu
//         .get(MenuEntry::Next.get_id())
//         .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
//     sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::NextBreak))
//         .ok();
// }

// /// Update the systemtray `Puased/Resume` item
// fn update_menu_pause(state: &Arc<Mutex<ApplicationState>>, paused: bool) {
//     let title = if paused {
//         "Resume"
//     } else {
//         MenuEntry::Pause.as_str()
//     };

//     // the error is here
//     // let state = app.state::<Arc<Mutex<ApplicationState>>>();

//     state
//         .lock()
//         .system_tray_menu
//         .get(MenuEntry::Next.get_id())
//         .and_then(|i| i.as_menuitem().and_then(|i| i.set_enabled(!paused).ok()));
//     state
//         .lock()
//         .system_tray_menu
//         .get(MenuEntry::Session.get_id())
//         .and_then(|i| i.as_menuitem().and_then(|i| i.set_enabled(!paused).ok()));

//     state
//         .lock()
//         .system_tray_menu
//         .get(MenuEntry::Pause.get_id())
//         .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
// }

// /// Update all menu items
// fn update_menu(state: &Arc<Mutex<ApplicationState>>, sx: &Sender<InternalMessage>) {
//     update_menu_next_break(state, sx);
//     update_menu_session_number(state, sx);
// }

// fix thie
/// Stop the tick process, and start a new one
fn reset_timer(state: &Arc<Mutex<ApplicationState>>) {
    state.lock().reset_timer();
    heartbeat_process(&state.lock().sx);
}

async fn reset_settings(
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) -> Result<(), AppError> {
    let sqlite = state.lock().sqlite.clone();
    let settings = ModelSettings::reset_settings(&sqlite).await?;
    state.lock().set_settings(settings);
    reset_timer(state);
    sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::GetSettings))
        .ok();
    sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Paused(
        state.lock().get_paused(),
    )))
    .ok();
    Ok(())
}
async fn update_settings(
    frontend_state: FrontEndState,
    state: &Arc<Mutex<ApplicationState>>,
) -> Result<(), AppError> {
    let sqlite = state.lock().sqlite.clone();
    let new_settings = ModelSettings::from(&frontend_state);
    ModelSettings::update(&sqlite, &new_settings).await?;
    state.lock().update_all_settings(&frontend_state);
    Ok(())
}

/// Handle all internal messages about window visibility
fn handle_visibility(
    app: &AppHandle,
    window_visibility: WindowVisibility,
    state: &Arc<Mutex<ApplicationState>>,
) {
    let on_break = state.lock().on_break();
    match window_visibility {
        WindowVisibility::Close => {
            if !on_break {
                app.exit(0);
            }
        }
        WindowVisibility::Hide => {
            if !on_break {
                WindowAction::hide_window(app, false);
            }
        }
        WindowVisibility::Minimize => {
            WindowAction::hide_window(app, false);
        }
        WindowVisibility::Show => {
            WindowAction::show_window(app, false);
        }
        WindowVisibility::Toggle => {
            if !on_break {
                WindowAction::toggle_visibility(app, false);
            }
        }
    }
}

/// Handle all internal messages about emitting messages to the frontend, and send to the frontend
#[allow(clippy::too_many_lines)]
fn emit_to_frontend(
    app: &AppHandle,
    msg_to_frontend: ToFrontEnd,
    state: &Arc<Mutex<ApplicationState>>,
) {
    let event_name = msg_to_frontend.as_str();
    match msg_to_frontend {
        ToFrontEnd::GoToSettings => {
            // WindowAction::show_window(app, false);
            let on_break = state.lock().on_break();
            // state.lock().sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Fullscreen(false))).ok();
            if !on_break {
                app.emit_str(MAIN_WINDOW, event_name.to_owned()).ok();
                // todo fix this?
                WindowAction::toggle_visibility(app, false);
            }
        }

        //  ToFrontEnd::Fullscreen(value) => {
        // 	  app.emit_to(MAIN_WINDOW, event_name, value).ok();
        // }
        ToFrontEnd::Cpu(value) => {
            app.app_handle()
                .emit_to(MAIN_WINDOW, event_name, value)
                .ok();
        }

        ToFrontEnd::NextBreak => {
            app.app_handle()
                .emit_to(MAIN_WINDOW, event_name, state.lock().get_next_break_title())
                .ok();
        }

        ToFrontEnd::OnBreak => {
            app.emit_to(MAIN_WINDOW, event_name, state.lock().current_timer_left())
                .ok();
        }

        ToFrontEnd::Error => {
            app.emit_to(MAIN_WINDOW, event_name, "Internal Error").ok();
        }

        ToFrontEnd::GetSettings => {
            app.emit_to(MAIN_WINDOW, event_name, state.lock().get_state())
                .ok();
        }
        ToFrontEnd::SessionsBeforeLong => {
            app.app_handle()
                .emit_to(
                    MAIN_WINDOW,
                    event_name,
                    state.lock().get_sessions_before_long_title(),
                )
                .ok();
        }
        ToFrontEnd::GoToTimer => {
            let (break_time, strategy) = state.lock().get_break_settings();
            app.emit_to(MAIN_WINDOW, "fullscreen", state.lock().get_fullscreen())
                .ok();
            app.emit_to(
                MAIN_WINDOW,
                event_name,
                ShowTimer::new(break_time, strategy),
            )
            .ok();
        }
        ToFrontEnd::BuildInfo(info) => {
            app.emit_to(MAIN_WINDOW, event_name, info).ok();
        }
        ToFrontEnd::Paused(paused) => {
            app.emit_to(MAIN_WINDOW, event_name, paused).ok();
        }
    }
}

/// Handle all internal messages about the Break/Session stats
fn handle_break(
    break_message: BreakMessage,
    state: &Arc<Mutex<ApplicationState>>,
    app: &AppHandle,
    sx: &Sender<InternalMessage>,
) {
    let fullscreen = state.lock().get_fullscreen();
    match break_message {
        BreakMessage::Start => {
            state.lock().start_break_session();
            change_menu_entry_status(state, false);
            sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::GoToTimer))
                .ok();
            WindowAction::show_window(app, fullscreen);
        }
        BreakMessage::End => {
            state.lock().start_work_session();
            change_menu_entry_status(state, true);
            if state.lock().pause_after_break {
                sx.send(InternalMessage::Pause).ok();
                // if the app is in fullscreen mode, need to remove the fullscreen, normally this is handled by the hide_window function, but it's not being called here
                WindowAction::remove_fullscreen(app);
            } else {
                WindowAction::hide_window(app, fullscreen);
                update_menu(state, sx);
            }

            state.lock().pause_after_break = false;
            //    sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Fullscreen(false)))
            // .ok();
        }
    }
}


// create state here
// TODO combine this with the application state, have state as on .self
/// Spawn into a tokio thread, handle all internal messages
pub fn start_message_handler(
    app_handle: &AppHandle,
    state: Arc<Mutex<ApplicationState>>,
    mut rx: Receiver<InternalMessage>,
    sx: Sender<InternalMessage>,
) {
    let app_handle = app_handle.to_owned();
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            match message {
                InternalMessage::ToFrontEnd(emitter) => {
                    emit_to_frontend(&app_handle, emitter, &state);
                }
                InternalMessage::OpenLocation => {
                    open::that(state.lock().get_data_location()).ok();
                }
                InternalMessage::UpdatePause(pause) => {
                    state.lock().pause_after_break = pause;
                }
                InternalMessage::SetSetting(frontend_state) => {
                    if let Err(e) = update_settings(frontend_state, &state).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Error)).ok();
                    }
                    update_menu(&state, &sx);
                }
                InternalMessage::ResetSettings => {
                    if let Err(e) = reset_settings(&state, &sx).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Error)).ok();
                    }
                    update_menu(&state, &sx);
                }
                InternalMessage::Hearbteat(heartbeat) => match heartbeat {
                    Hearbteat::Abort => {
                        if let Some(handle) = state.lock().heartbeat_process.as_ref() {
                            handle.abort();
                        }
                    }
                    Hearbteat::Update(handle) => {
                        state.lock().heartbeat_process = Some(handle);
                    }
                    Hearbteat::OnHeartbeat(cpu_usage) => {
                        state.lock().on_heartbeat(cpu_usage);
                    }
                    Hearbteat::UpdateTimer => {
                        state.lock().update_timer_check();
                    }
                },
                InternalMessage::ResetTimer => {
                    reset_timer(&state);
                }

                InternalMessage::UpdateMenuTimer => update_menu(&state, &sx),

                InternalMessage::Break(break_message) => {
                    handle_break(break_message, &state, &app_handle, &sx);
                }

                InternalMessage::Window(window_visibility) => {
                    handle_visibility(&app_handle, window_visibility, &state);
                }

                InternalMessage::Pause => {
                    let paused = state.lock().toggle_pause();
                    update_menu_pause(&state, paused);
                    set_icon(&app_handle, paused);
                    sx.send(InternalMessage::ToFrontEnd(ToFrontEnd::Paused(paused)))
                        .ok();
                }
            }
        }
    });
}
