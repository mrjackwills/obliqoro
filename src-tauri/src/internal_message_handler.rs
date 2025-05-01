use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tracing::error;

use crate::{
    app_error::AppError,
    application_state::ApplicationState,
    db::ModelSettings,
    heartbeat::heartbeat_process,
    request_handlers::{FrontEnd, FrontEndState, ShowTimer},
    system_tray::{menu_enabled, set_icon, MenuItem},
    window_action::WindowAction,
    ObliqoroWindow,
};
use tokio::sync::broadcast::{Receiver, Sender};

/// Get information about self for the Footer component
/// BUILD_DATE is injected via the build.rs file
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct PackageInfo {
    pub homepage: String,
    pub version: String,
    pub build_date: String,
    pub github_version: Option<String>,
}
impl Default for PackageInfo {
    fn default() -> Self {
        let (homepage, _) = env!("CARGO_PKG_REPOSITORY")
            .split_once(env!("CARGO_PKG_NAME"))
            .unwrap_or_default();
        Self {
            homepage: homepage.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            build_date: env!("BUILD_DATE").to_owned(),
            github_version: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum BreakMessage {
    End,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum WindowVisibility {
    Close,
    Hide,
    Minimize,
    Show,
    Toggle,
}

#[derive(Debug, Clone)]
pub enum InternalMessage {
    Break(BreakMessage),
    Pause,
    ResetSettings,
    ResetTimer,
    SetSetting(FrontEndState),
    ToFrontEnd(FrontEnd),
    UpdateMenuTimer,
    Window(WindowVisibility),
}

// Update the taskbar to display how many sessions before next long break,
// and send internal message, to send message to front end to update settings in pinia
fn update_menu_session_number(
    app: &AppHandle,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) {
    if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
        window
            .app_handle()
            .tray_handle()
            .try_get_item(MenuItem::Session.get_id())
            .and_then(|item| {
                item.set_title(state.lock().get_sessions_before_long_title())
                    .ok()
            });
    }
    sx.send(InternalMessage::ToFrontEnd(FrontEnd::SessionsBeforeLong))
        .ok();
}

/// Update the systemtray next break in text, and emit to frontend to next break timer
fn update_menu_next_break(
    app: &AppHandle,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) {
    if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
        window
            .app_handle()
            .tray_handle()
            .try_get_item(MenuItem::Next.get_id())
            .and_then(|item| item.set_title(state.lock().get_next_break_title()).ok());
    }
    sx.send(InternalMessage::ToFrontEnd(FrontEnd::NextBreak))
        .ok();
}

/// Update the systemtray `Puased/Resume` item
fn update_menu_pause(app: &AppHandle, state: &Arc<Mutex<ApplicationState>>) {
    let paused = state.lock().get_paused();
    let title = if paused {
        "Resume"
    } else {
        MenuItem::Pause.as_str()
    };

    if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
        window
            .app_handle()
            .tray_handle()
            .try_get_item(MenuItem::Next.get_id())
            .and_then(|item| item.set_enabled(!paused).ok());
        window
            .app_handle()
            .tray_handle()
            .try_get_item(MenuItem::Session.get_id())
            .and_then(|item| item.set_enabled(!paused).ok());
        window
            .app_handle()
            .tray_handle()
            .try_get_item(MenuItem::Pause.get_id())
            .and_then(|item| item.set_title(title).ok());
    }
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
    state.lock().reset_settings(settings);
    reset_timer(state);
    sx.send(InternalMessage::ToFrontEnd(FrontEnd::GetSettings))
        .ok();
    sx.send(InternalMessage::ToFrontEnd(FrontEnd::Paused)).ok();
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
                // remove lock file!
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

/// Handle all internal messages about emitting messages to the frontend
#[allow(clippy::too_many_lines)]
fn handle_emitter(app: &AppHandle, front_end_msg: FrontEnd, state: &Arc<Mutex<ApplicationState>>) {
    let event_name = front_end_msg.as_str();
    match front_end_msg {
        FrontEnd::GoToSettings => {
            let on_break = state.lock().on_break();
            if !on_break {
                app.emit_to(ObliqoroWindow::Main.as_str(), event_name, ())
                    .ok();
                WindowAction::toggle_visibility(app, false);
            }
        }

        FrontEnd::Cpu(value) => {
            app.app_handle()
                .emit_to(ObliqoroWindow::Main.as_str(), event_name, value)
                .ok();
        }

        FrontEnd::NextBreak => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    event_name,
                    state.lock().get_next_break_title(),
                )
                .ok();
        }

        FrontEnd::OnBreak => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                state.lock().current_timer_left(),
            )
            .ok();
        }

        FrontEnd::Error => {
            app.emit_to(ObliqoroWindow::Main.as_str(), event_name, "Internal Error")
                .ok();
        }

        FrontEnd::GetSettings => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                state.lock().get_state(),
            )
            .ok();
        }
        FrontEnd::SessionsBeforeLong => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    event_name,
                    state.lock().get_sessions_before_long_title(),
                )
                .ok();
        }
        FrontEnd::GoToTimer => {
            let (break_time, strategy) = state.lock().get_break_settings();
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                ShowTimer::new(break_time, strategy),
            )
            .ok();
        }
        FrontEnd::PackageInfo(info) => {
            app.emit_to(ObliqoroWindow::Main.as_str(), event_name, info)
                .ok();
        }
        FrontEnd::Paused => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                event_name,
                state.lock().get_paused(),
            )
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
            sx.send(InternalMessage::ToFrontEnd(FrontEnd::GoToTimer))
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

/// On a tokio thread, handle all internal messages
pub fn start_message_handler(
    app: &tauri::App,
    state: Arc<Mutex<ApplicationState>>,
    mut rx: Receiver<InternalMessage>,
    sx: Sender<InternalMessage>,
) {
    let app_handle = app.handle();
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            match message {
                InternalMessage::ToFrontEnd(emitter) => {
                    handle_emitter(&app_handle, emitter, &state);
                }
                InternalMessage::SetSetting(frontend_state) => {
                    if let Err(e) = update_settings(frontend_state, &state).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(FrontEnd::Error)).ok();
                    }
                    update_menu(&app_handle, &state, &sx);
                }
                InternalMessage::ResetSettings => {
                    if let Err(e) = reset_settings(&state, &sx).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::ToFrontEnd(FrontEnd::Error)).ok();
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

                    update_menu_pause(&app_handle, &state);
                    set_icon(&app_handle, paused);
                    sx.send(InternalMessage::ToFrontEnd(FrontEnd::Paused)).ok();
                }
            }
        }
    });
}
