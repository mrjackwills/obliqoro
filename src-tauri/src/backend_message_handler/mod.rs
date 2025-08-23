use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tracing::error;
mod messages;

pub use messages::*;

use crate::{
    app_error::AppError,
    application_state::ApplicationState,
    db::ModelSettings,
    heartbeat::heartbeat_process,
    request_handlers::{FrontEndState, MsgToFrontend, ShowTimer},
    system_tray::{menu_enabled, set_icon, MenuEntry},
    window_action::WindowAction,
    ObliqoroWindow,
};
use tokio::sync::broadcast::{Receiver, Sender};

// Update the taskbar to display how many sessions before next long break,
// and send internal message, to send message to front end to update settings in pinia
fn update_menu_session_number(
    app: &AppHandle,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) {
	// TODO fix me
    // if let Some(window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
    //     window
    //         .app_handle()
	// 		.tray_by_id(id)
			
    //         .tray_handle()
    //         .try_get_item(MenuItem::Session.get_id())
    //         .and_then(|item| {
    //             item.set_title(state.lock().get_sessions_before_long_title())
    //                 .ok()
    //         });
    // }
    // sx.send(InternalMessage::ToFrontEnd(
    //     MsgToFrontend::SessionsBeforeLong,
    // ))
    // .ok();
}

/// Update the systemtray next break in text, and emit to frontend to next break timer
fn update_menu_next_break(
    app: &AppHandle,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) {
	// todo fix me
    // if let Some(window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
    //     window
    //         .app_handle()
    //         .tray_handle()
    //         .try_get_item(MenuItem::Next.get_id())
    //         .and_then(|item| item.set_title(state.lock().get_next_break_title()).ok());
    // }
    // sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::NextBreak))
    //     .ok();
}

/// Update the systemtray `Puased/Resume` item
fn update_menu_pause(app: &AppHandle, paused: bool) {
    // let paused = state.lock().get_paused();
    let title = if paused {
        "Resume"
    } else {
        MenuEntry::Pause.as_str()
    };

	// todo fix me
    // if let Some(window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
    //     window
    //         .app_handle()
    //         .tray_handle()
    //         .try_get_item(MenuItem::Next.get_id())
    //         .and_then(|item| item.set_enabled(!paused).ok());
    //     window
    //         .app_handle()
    //         .tray_handle()
    //         .try_get_item(MenuItem::Session.get_id())
    //         .and_then(|item| item.set_enabled(!paused).ok());
    //     window
    //         .app_handle()
    //         .tray_handle()
    //         .try_get_item(MenuItem::Pause.get_id())
    //         .and_then(|item| item.set_title(title).ok());
    // }
}

/// Update all menu items
fn update_menu(
    app: &AppHandle,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) {
    update_menu_next_break(app, state, sx);
    update_menu_session_number(app, state, sx);
}

/// Stop the tick process, and start a new one
fn reset_timer(state: &Arc<Mutex<ApplicationState>>) {
    state.lock().reset_timer();
    heartbeat_process(state);
}

async fn reset_settings(
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) -> Result<(), AppError> {
    let sqlite = state.lock().sqlite.clone();
    let settings = ModelSettings::reset_settings(&sqlite).await?;
    state.lock().set_settings(settings);
    reset_timer(state);
    sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::GetSettings))
        .ok();
    sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::Paused(
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
    msg_to_frontend: MsgToFrontend,
    state: &Arc<Mutex<ApplicationState>>,
) {
    let event_name = msg_to_frontend.as_str();
    match msg_to_frontend {
        MsgToFrontend::GoToSettings => {
            let on_break = state.lock().on_break();
            if !on_break {
				app.emit_str(ObliqoroWindow::Main.as_str(), event_name.to_owned())
                    .ok();
                WindowAction::toggle_visibility(app, false);
            }
        }

        MsgToFrontend::Cpu(value) => {
            app.app_handle()
                .emit_to(ObliqoroWindow::Main.as_str(), event_name, value)
                .ok();
        }

        MsgToFrontend::NextBreak => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    event_name,
                    state.lock().get_next_break_title(),
                )
                .ok();
        }

        MsgToFrontend::OnBreak => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                state.lock().current_timer_left(),
            )
            .ok();
        }

        MsgToFrontend::Error => {
            app.emit_to(ObliqoroWindow::Main.as_str(), event_name, "Internal Error")
                .ok();
        }

        MsgToFrontend::GetSettings => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                state.lock().get_state(),
            )
            .ok();
        }
        MsgToFrontend::SessionsBeforeLong => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    event_name,
                    state.lock().get_sessions_before_long_title(),
                )
                .ok();
        }
        MsgToFrontend::GoToTimer => {
            let (break_time, strategy) = state.lock().get_break_settings();
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                ShowTimer::new(break_time, strategy),
            )
            .ok();
        }
        MsgToFrontend::BuildInfo(info) => {
            app.emit_to(ObliqoroWindow::Main.as_str(), event_name, info)
                .ok();
        }
        MsgToFrontend::Paused(paused) => {
            app.emit_to(ObliqoroWindow::Main.as_str(), event_name, paused)
                .ok();
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
            menu_enabled(app, false);
            sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::GoToTimer))
                .ok();
            WindowAction::show_window(app, fullscreen);
        }
        BreakMessage::End => {
            state.lock().start_work_session();
            menu_enabled(app, true);
            if state.lock().pause_after_break {
                sx.send(InternalMessage::Pause).ok();
                // if the app is in fullscreen mode, need to remove the fullscreen, normally this is handled by the hide_window function, but it's not being called here
                WindowAction::remove_fullscreen(app);
            } else {
                WindowAction::hide_window(app, fullscreen);
                update_menu(app, state, sx);
            }

            state.lock().pause_after_break = false;
        }
    }
}

/// Spawn into a tokio thread, handle all internal messages
pub fn start_message_handler(
    app: &tauri::App,
    state: Arc<Mutex<ApplicationState>>,
    mut rx: Receiver<InternalMessage>,
    sx: Sender<InternalMessage>,
) {
    let app_handle = app.app_handle().to_owned();
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            match message {
                InternalMessage::ToFrontEnd(emitter) => {
                    emit_to_frontend(&app_handle, emitter, &state);
                }
                InternalMessage::SetSetting(frontend_state) => {
                    if let Err(e) = update_settings(frontend_state, &state).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::Error))
                            .ok();
                    }
                    update_menu(&app_handle, &state, &sx);
                }
                InternalMessage::ResetSettings => {
                    if let Err(e) = reset_settings(&state, &sx).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::Error))
                            .ok();
                    }
                    update_menu(&app_handle, &state, &sx);
                }
                InternalMessage::ResetTimer => {
                    reset_timer(&state);
                }

                InternalMessage::UpdateMenuTimer => update_menu(&app_handle, &state, &sx),

                InternalMessage::Break(break_message) => {
                    handle_break(break_message, &state, &app_handle, &sx);
                }

                InternalMessage::Window(window_visibility) => {
                    handle_visibility(&app_handle, window_visibility, &state);
                }

                InternalMessage::Pause => {
                    let paused = state.lock().toggle_pause();
                    update_menu_pause(&app_handle, paused);
                    set_icon(&app_handle, paused);
                    sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::Paused(paused)))
                        .ok();
                }
            }
        }
    });
}
