#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use application_state::ApplicationState;
use backend_message_handler::{start_message_handler, InternalMessage, WindowVisibility};
use heartbeat::heartbeat_process;
use parking_lot::Mutex;
use std::sync::Arc;

#[cfg(debug_assertions)]
use tauri::Manager;

mod app_error;
mod application_state;
mod backend_message_handler;
mod check_version;
mod db;
mod heartbeat;
mod request_handlers;
mod system_tray;
mod window_action;

pub type TauriState<'a> = tauri::State<'a, Arc<Mutex<ApplicationState>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ObliqoroWindow {
    Main,
}

impl ObliqoroWindow {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Main => "main",
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let tray = system_tray::create_system_tray();

    let (sx, rx) = tokio::sync::broadcast::channel(128);

    let ctx = tauri::generate_context!();

    match ApplicationState::new(tauri::api::path::app_data_dir(ctx.config()), &sx).await {
        Err(e) => {
            tracing::error!("{e:?}");
            std::process::exit(1);
        }
        Ok(app_state) => {
            // TODO change this to just an Arc<ApplicationState>, and use a message bus everywhere?
            // Application state could just be an Arc<Sx<InternalMessage>
            let state = Arc::new(Mutex::new(app_state));
            let (init_state, internal_state) = (Arc::clone(&state), Arc::clone(&state));
            let (event_sx, close_sx, handler_sx, tray_sx, instance_sx) =
                (sx.clone(), sx.clone(), sx.clone(), sx.clone(), sx.clone());

            #[allow(unused_variables)]
            let app_builder = tauri::Builder::default()
                .plugin(tauri_plugin_shell::init())
                .manage(state)
                .setup(|app| {
                    #[cfg(debug_assertions)]
                    {
                        if let Some(main_window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
                            main_window.open_devtools();
                        }
                    }
                    Ok(())
                })
				
                // .system_tray(tray)
				// .tr
                .on_system_tray_event(move |_app, event| {
                    system_tray::on_system_tray_event(event, &tray_sx);
                })
                .on_window_event(move |handler| match handler.event() {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        event_sx
                            .send(InternalMessage::Window(WindowVisibility::Hide))
                            .ok();
                    }
                    tauri::WindowEvent::Moved(val) => {
                        if val.x <= -32000 && val.y <= -32000 {
                            event_sx
                                .send(InternalMessage::Window(WindowVisibility::Minimize))
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
                .plugin(tauri_plugin_single_instance::init(move |app, argv, cwd| {
                    instance_sx
                        .send(InternalMessage::Window(WindowVisibility::Show))
                        .ok();
                }))
                .build(tauri::generate_context!());

            match app_builder {
                Ok(app) => {
                    heartbeat_process(&init_state);
                    start_message_handler(&app, internal_state, rx, handler_sx);
                    app.run(move |_app, event| {
                        if let tauri::RunEvent::ExitRequested { api, .. } = event {
                            close_sx
                                .send(InternalMessage::Window(WindowVisibility::Hide))
                                .ok();
                            api.prevent_exit();
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("{:?}", e);
                    tracing::error!("Unable to build application");
                }
            }
        }
    }
    Ok(())
}
