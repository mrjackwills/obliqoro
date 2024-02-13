use crate::{
    internal_message_handler::{Emitter, InternalMessage, SettingChange, WindowVisibility},
    TauriState,
};

mod messages;
use auto_launch::AutoLaunch;
pub use messages::*;
use tauri::utils::platform::current_exe;

fn auto_launch() -> Option<AutoLaunch> {
    current_exe().map_or(None, |app_exe| {
        let app_path = dunce::canonicalize(app_exe).unwrap_or_default();
        let app_name = app_path.file_stem().unwrap_or_default().to_os_string();
        Some(AutoLaunch::new(
            app_name.to_str().unwrap_or_default(),
            app_path.to_str().unwrap_or_default(),
            &[] as &[&str],
        ))
    })
}

/// Initialise the fontend store & settings
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn init(state: TauriState<'_>) {
    for message in [
        Emitter::Settings,
        Emitter::NextBreak,
        Emitter::SessionsBeforeLong,
        Emitter::PackageInfo,
    ] {
        state.lock().sx.send(InternalMessage::Emit(message)).ok();
    }
    get_autostart(state);
}

/// Request to reset settings to default
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn reset_settings(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(SettingChange::Reset))
        .ok();
}

/// Toggle the autostart option
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_autostart(state: TauriState<'_>, value: bool) {
    if let Some(i) = auto_launch() {
        if value {
            i.enable().ok();
        } else {
            i.disable().ok();
        }
    }
    get_autostart(state);
}

/// Toggle the pause option
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn toggle_pause(state: TauriState<'_>) {
    state.lock().sx.send(InternalMessage::Pause).ok();
}

/// Set the pause after break setting
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn pause_after_break(state: TauriState<'_>, pause: bool) {
    state.lock().pause_after_break = pause;
}

/// Get the current status of the autostart setting
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn get_autostart(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::Emit(Emitter::AutoStart(
            auto_launch().map_or(false, |i| i.is_enabled().unwrap_or_default()),
        )))
        .ok();
}

/// Request to set the full screen setting to the given boolean value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_fullscreen(state: TauriState<'_>, value: bool) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(SettingChange::FullScreen(
            value,
        )))
        .ok();
}

/// Request to set the session length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_session(state: TauriState<'_>, value: u16) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::SessionLength(value),
        ))
        .ok();
}

/// Request to set the long_break length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_longbreak(state: TauriState<'_>, value: u16) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::LongBreakLength(value),
        ))
        .ok();
}

/// Request to set the short_break length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_shortbreak(state: TauriState<'_>, value: u8) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::ShortBreakLength(value),
        ))
        .ok();
}

/// Request to set the number of sessions before long_break to the given u8 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_number_sessions(state: TauriState<'_>, value: u8) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::NumberSessions(value),
        ))
        .ok();
}

/// Request to minimize the application window
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn minimize(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::Window(WindowVisibility::Toggle))
        .ok();
}
