#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use application_state::ApplicationState;
use backend_message_handler::{start_message_handler, InternalMessage, WindowVisibility};
use heartbeat::heartbeat_process;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::generate_context;

use tauri::{Builder, Manager};

mod app_error;
mod application_state;
mod backend_message_handler;
mod check_version;
mod db;
mod heartbeat;
mod request_handlers;
mod system_tray;
mod window_action;

// TODO change to an sx
pub type TauriState<'a> = tauri::State<'a, tokio::sync::broadcast::Sender<InternalMessage>>;

const SYSTEM_TRAY_ID: &str = "obliqoro_system_tray";
const MAIN_WINDOW: &str = "main";

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (sx, rx) = tokio::sync::broadcast::channel(128);
    let (sx1, sx2, sx3, sx4) = (sx.clone(), sx.clone(), sx.clone(), sx.clone());

	// Start the message_handler here, use a sx/rx to send the tray mnu & data location to the state?


    Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                if let Some(main_window) = app.get_webview_window(MAIN_WINDOW) {
                    main_window.open_devtools();
                }
            }

            let Ok(app_data_dir) = tauri::path::PathResolver::app_data_dir(app.path()) else {
                std::process::exit(1)
            };
            let (temp_tx, tmp_rx) = std::sync::mpsc::channel();
            let system_tray_menu = system_tray::create_system_tray(app.app_handle(), &sx)?;

			let t = system_tray_menu.clone();

			// use  asingel one shot here? to send to the
			// TODO all of this in the message handler thead
			// have  msg called SetDBLocation, sent after manae is called
            tokio::spawn(async move {
                let Ok(state) = ApplicationState::new(app_data_dir, sx, system_tray_menu).await
                else {
                    std::process::exit(1);
                };
                temp_tx.send(state).ok();
            });
            let Ok(state) = tmp_rx.recv() else {
                std::process::exit(1);
            };


            heartbeat_process(&sx2);
            start_message_handler(app.app_handle(), state, rx, sx2);
            app.manage(sx1);
            Ok(())
        })
        .on_window_event(move |_window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                sx3.send(InternalMessage::Window(WindowVisibility::Hide))
                    .ok();
            }
            tauri::WindowEvent::Moved(val) => {
                if val.x <= -32000 && val.y <= -32000 {
                    sx3.send(InternalMessage::Window(WindowVisibility::Minimize))
                        .ok();
                }
            }
            _ => (),
        })
        .invoke_handler(tauri::generate_handler![
            request_handlers::init,
            request_handlers::minimize,
            request_handlers::open_database_location,
            request_handlers::pause_after_break,
            request_handlers::reset_settings,
            request_handlers::set_settings,
            request_handlers::toggle_pause,
        ])
        .plugin(tauri_plugin_single_instance::init(
            move |_app, _argv, _cwd| {
                sx4.send(InternalMessage::Window(WindowVisibility::Show))
                    .ok();
            },
        ))
        .run(generate_context!())
        .ok();
    Ok(())
}
