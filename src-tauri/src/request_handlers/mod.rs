use crate::{
    check_version,
    backend_message_handler::{BuildInfo, InternalMessage, WindowVisibility},
    TauriState,
};

mod messages;
pub use messages::*;

/// Initialise the fontend store & settings
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn init(state: TauriState<'_>) {
    for message in [
        MsgToFrontend::GetSettings,
        MsgToFrontend::NextBreak,
        MsgToFrontend::SessionsBeforeLong,
        MsgToFrontend::BuildInfo(BuildInfo::default()),
    ] {
        state
            .lock()
            .sx
            .send(InternalMessage::ToFrontEnd(message))
            .ok();
    }
    check_version::parse_github(state.lock().sx.clone());
}

/// Request to reset settings to default
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn reset_settings(state: TauriState<'_>) {
    state.lock().sx.send(InternalMessage::ResetSettings).ok();
}

/// Toggle the pause option
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn toggle_pause(state: TauriState<'_>) {
    state.lock().sx.send(InternalMessage::Pause).ok();
}

/// Set the pause after break setting
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn pause_after_break(state: TauriState<'_>, pause: bool) {
    state.lock().pause_after_break = pause;
}

/// Request to set the full screen setting to the given boolean value
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn open_database_location(state: TauriState<'_>) {
    open::that(state.lock().get_data_location()).ok();
    state
        .lock()
        .sx
        .send(InternalMessage::Window(WindowVisibility::Hide))
        .ok();
}

/// Set all settings
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn set_settings(state: TauriState<'_>, value: FrontEndState) {
    state
        .lock()
        .sx
        .send(InternalMessage::SetSetting(value))
        .ok();
}

/// Request to minimize the application window
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn minimize(state: TauriState<'_>) {
    state
        .lock()
        .sx
        .send(InternalMessage::Window(WindowVisibility::Toggle))
        .ok();
}
