use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tracing::error;

use crate::{
    app_error::AppError,
    application_state::ApplicationState,
    db::ModelSettings,
    request_handlers::{EmitMessages, ShowTimer},
    system_tray::{menu_enabled, MenuItem},
    tick::tick_process,
    window_action::WindowAction,
    ObliqoroWindow,
};
use tokio::sync::broadcast::{Receiver, Sender};

/// Get information about self for the Footer component
/// BUILD_DATE is injected via the build.rs file
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct PackageInfo {
    homepage: String,
    version: String,
    build_date: String,
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum SettingChange {
    FullScreen(bool),
    LongBreakLength(u16),
    NumberSessions(u8),
    ShortBreakLength(u8),
    SessionLength(u16),
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum BreakMessage {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum WindowVisibility {
    Close,
    Hide,
    Minimize,
    Toggle,
    Show,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum Emitter {
    AutoStart(bool),
    GoToSettings,
    NextBreak,
    OnBreak,
    PackageInfo,
    Paused,
    SendError,
    SessionsBeforeLong,
    Settings,
    Timer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub enum InternalMessage {
    Break(BreakMessage),
    ChangeSetting(SettingChange),
    Emit(Emitter),
    Pause,
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
    };
    sx.send(InternalMessage::Emit(Emitter::SessionsBeforeLong))
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
    sx.send(InternalMessage::Emit(Emitter::NextBreak)).ok();
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
    tick_process(state);
}

/// Update the database setting data, and self.setting, and if necessary reset timers etc
async fn handle_settings(
    setting_change: SettingChange,
    state: &Arc<Mutex<ApplicationState>>,
    sx: &Sender<InternalMessage>,
) -> Result<(), AppError> {
    let settings = state.lock().get_settings();
    match setting_change {
        SettingChange::FullScreen(value) => {
            let sqlite = state.lock().sqlite.clone();
            if value != settings.fullscreen {
                ModelSettings::update_fullscreen(&sqlite, value).await?;
                state.lock().set_fullscreen(value);
            }
        }
        SettingChange::LongBreakLength(value) => {
            if value != settings.long_break_as_sec {
                let sqlite = state.lock().sqlite.clone();
                ModelSettings::update_longbreak(&sqlite, value).await?;
                state.lock().set_long_break_as_sec(value);
            }
        }
        SettingChange::NumberSessions(value) => {
            if value != settings.number_session_before_break {
                let sqlite = state.lock().sqlite.clone();
                ModelSettings::update_number_session_before_break(&sqlite, value).await?;
                state.lock().set_number_session_before_break(value);
            }
        }
        SettingChange::Reset => {
            let sqlite = state.lock().sqlite.clone();
            let settings = ModelSettings::reset_settings(&sqlite).await?;
            state.lock().reset_settings(settings);
            reset_timer(state);
            sx.send(InternalMessage::Emit(Emitter::Settings)).ok();
            sx.send(InternalMessage::Emit(Emitter::Paused)).ok();
        }
        SettingChange::ShortBreakLength(value) => {
            if value != settings.short_break_as_sec {
                let sqlite = state.lock().sqlite.clone();
                ModelSettings::update_shortbreak(&sqlite, value).await?;
                state.lock().set_short_break_as_sec(value);
            }
        }
        SettingChange::SessionLength(value) => {
            if value != settings.session_as_sec {
                let sqlite = state.lock().sqlite.clone();
                ModelSettings::update_session(&sqlite, value).await?;
                state.lock().set_session_as_sec(value);
                reset_timer(state);
            }
        }
    }
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

/// Handle all internal messages about emitting messages to the frontend
fn handle_emitter(app: &AppHandle, emitter: Emitter, state: &Arc<Mutex<ApplicationState>>) {
    match emitter {
        Emitter::GoToSettings => {
            let on_break = state.lock().on_break();
            if !on_break {
                app.emit_to(
                    ObliqoroWindow::Main.as_str(),
                    EmitMessages::GoToSettings.as_str(),
                    (),
                )
                .ok();
                WindowAction::toggle_visibility(app, false);
            }
        }

        Emitter::NextBreak => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    EmitMessages::NextBreak.as_str(),
                    state.lock().get_next_break_title(),
                )
                .ok();
        }

        Emitter::OnBreak => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::OnBreak.as_str(),
                state.lock().current_timer_left(),
            )
            .ok();
        }

        Emitter::AutoStart(value) => {
            let on_break = state.lock().on_break();
            if !on_break {
                app.emit_to(
                    ObliqoroWindow::Main.as_str(),
                    EmitMessages::AutoStart.as_str(),
                    value,
                )
                .ok();
            }
        }

        Emitter::SendError => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::Error.as_str(),
                "Internal Error",
            )
            .ok();
        }

        Emitter::Settings => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::GetSettings.as_str(),
                state.lock().get_settings(),
            )
            .ok();
        }
        Emitter::SessionsBeforeLong => {
            app.app_handle()
                .emit_to(
                    ObliqoroWindow::Main.as_str(),
                    EmitMessages::SessionsBeforeLong.as_str(),
                    state.lock().get_sessions_before_long_title(),
                )
                .ok();
        }
        Emitter::Timer => {
            let (break_time, strategy) = state.lock().get_break_settings();
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::GoToTimer.as_str(),
                ShowTimer::new(break_time, strategy),
            )
            .ok();
        }
        Emitter::PackageInfo => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::PackageInfo.as_str(),
                PackageInfo::default(),
            )
            .ok();
        }
        Emitter::Paused => {
            app.emit_to(
                ObliqoroWindow::Main.as_str(),
                EmitMessages::Paused.as_str(),
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
            sx.send(InternalMessage::Emit(Emitter::Timer)).ok();
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
                InternalMessage::Emit(emitter) => {
                    handle_emitter(&app_handle, emitter, &state);
                }

                InternalMessage::ChangeSetting(setting_change) => {
                    if let Err(e) = handle_settings(setting_change, &state, &sx).await {
                        error!("{:#?}", e);
                        sx.send(InternalMessage::Emit(Emitter::SendError)).ok();
                    }
                    update_menu(&app_handle, &state, &sx);
                }

                InternalMessage::UpdateMenuTimer => update_menu(&app_handle, &state, &sx),

                InternalMessage::Break(break_message) => {
                    handle_break(break_message, &state, &app_handle, &sx);
                }

                InternalMessage::Window(window_visibility) => {
                    handle_visibility(&app_handle, window_visibility, &state);
                }

                InternalMessage::Pause => {
                    state.lock().toggle_pause();
                    update_menu_pause(&app_handle, &state);
                    sx.send(InternalMessage::Emit(Emitter::Paused)).ok();
                }
            }
        }
    });
}
