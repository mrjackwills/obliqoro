use crate::{
    TauriState, check_version,
    message_handler::{MsgFE, MsgI, MsgWV, PackageInfo},
};

mod messages;
pub use messages::*;

/// Initialise the fontend store & settings
#[tauri::command]
// Change state to just use sx
#[allow(clippy::needless_pass_by_value)]
pub async fn init(sx: TauriState<'_>) -> Result<(),()> {
    for message in [
        MsgFE::GetSettings,
        MsgFE::NextBreak,
        MsgFE::SessionsBeforeLong,
        MsgFE::PackageInfo(PackageInfo::default()),
    ] {
        sx.send(MsgI::ToFrontEnd(message)).await.ok();
    }
    check_version::fetch_updates(sx.inner().clone());
	Ok(())
}

/// Request to reset settings to default
#[tauri::command]
pub async fn reset_settings(sx: TauriState<'_>)  -> Result<(),()> {
    sx.send(MsgI::ResetSettings).await.ok();
	Ok(())
}

/// Toggle the pause option
#[tauri::command]
pub async fn toggle_pause(sx: TauriState<'_>)-> Result<(),()>  {
    sx.send(MsgI::Pause).await.ok();
	Ok(())
}

/// Set the pause after break setting
#[tauri::command]
pub async fn pause_after_break(sx: TauriState<'_>, pause: bool) -> Result<(),()> {
    sx.send(MsgI::UpdatePause(pause)).await.ok();
	Ok(())
}

/// Request to set the full screen setting to the given boolean value
#[tauri::command]
pub async fn open_location(sx: TauriState<'_>, location: Option<String>) -> Result<(),()> {
    sx.send(MsgI::OpenLocation(location)).await.ok();
    sx.send(MsgI::Window(MsgWV::Hide)).await.ok();
	Ok(())
}

/// Set all settings
#[tauri::command]
pub async fn set_settings(sx: TauriState<'_>, value: FrontEndState) -> Result<(),()> {
    sx.send(MsgI::SetSetting(value)).await.ok();
	Ok(())
}

/// Request to minimize the application window
#[tauri::command]
pub async fn minimize(sx: TauriState<'_>) -> Result<(),()> {
    sx.send(MsgI::Window(MsgWV::Toggle)).await.ok();
	Ok(())
}
