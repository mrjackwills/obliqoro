use crate::{
    internal_message_handler::{Emitter, InternalMessage, SettingChange, WindowVisibility},
    TauriState,
};

mod messages;
pub use messages::*;

/// Initialise the frontent store & settings
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn init(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::Emit(Emitter::SendSettings))
        .unwrap_or_default();
    state
        .lock()
        .sx
        .send(InternalMessage::Emit(Emitter::NextBreak))
        .unwrap_or_default();
    state
        .lock()
        .sx
        .send(InternalMessage::Emit(Emitter::SessionsBeforeLong))
        .unwrap_or_default();
    state
        .lock()
        .sx
        .send(InternalMessage::Emit(Emitter::PackageInfo))
        .unwrap_or_default();
}

/// Request to reset settings to default
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn reset_settings(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(SettingChange::Reset))
        .unwrap_or_default();
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
        .unwrap_or_default();
}

/// Request to set the session length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_session(state: TauriState<'_>, value: i64) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::SessionLength(value),
        ))
        .unwrap_or_default();
}

/// Request to set the long_break length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_longbreak(state: TauriState<'_>, value: i64) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::LongBreakLength(value),
        ))
        .unwrap_or_default();
}

/// Request to set the short_break length to the given i64 value
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn set_setting_shortbreak(state: TauriState<'_>, value: i64) {
    state
        .lock()
        .sx
        .send(InternalMessage::ChangeSetting(
            SettingChange::ShortBreakLength(value),
        ))
        .unwrap_or_default();
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
        .unwrap_or_default();
}

/// Request to minimize the application window
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn minimize(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::Window(WindowVisibility::Toggle))
        .unwrap_or_default();
}
