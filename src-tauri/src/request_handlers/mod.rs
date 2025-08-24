use crate::{
    backend_message_handler::{PackageInfo, MsgFE, MsgI, MsgWV},
    check_version, TauriState,
};

mod messages;
pub use messages::*;

/// Initialise the fontend store & settings
#[tauri::command]
// Change state to just use sx
#[allow(clippy::needless_pass_by_value)]
pub fn init(sx: TauriState<'_>) {
    for message in [
        MsgFE::GetSettings,
        MsgFE::NextBreak,
        MsgFE::SessionsBeforeLong,
        MsgFE::PackageInfo(PackageInfo::default()),
    ] {
        sx.send(MsgI::ToFrontEnd(message)).ok();
    }
    check_version::fetch_updates(sx.inner().clone());
}

/// Request to reset settings to default
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn reset_settings(sx: TauriState<'_>) {
    sx.send(MsgI::ResetSettings).ok();
}

/// Toggle the pause option
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn toggle_pause(sx: TauriState<'_>) {
    sx.send(MsgI::Pause).ok();
}

/// Set the pause after break setting
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn pause_after_break(sx: TauriState<'_>, pause: bool) {
    sx.send(MsgI::UpdatePause(pause)).ok();
}

/// Request to set the full screen setting to the given boolean value
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn open_database_location(sx: TauriState<'_>) {
    sx.send(MsgI::OpenLocation).ok();
    sx.send(MsgI::Window(MsgWV::Hide)).ok();
}

/// Set all settings
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn set_settings(sx: TauriState<'_>, value: FrontEndState) {
    sx.send(MsgI::SetSetting(value)).ok();
}

/// Request to minimize the application window
#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub fn minimize(sx: TauriState<'_>) {
    sx.send(MsgI::Window(MsgWV::Toggle)).ok();
}
