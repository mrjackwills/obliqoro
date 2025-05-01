#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app_error::AppError;
use application_state::ApplicationState;
use heartbeat::heartbeat_process;
use internal_message_handler::{start_message_handler, InternalMessage, WindowVisibility};
use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};
use tracing::{error, Level};
use tracing_subscriber::{fmt as t_fmt, prelude::__tracing_subscriber_SubscriberExt};

#[cfg(debug_assertions)]
use tauri::Manager;

mod app_error;
mod application_state;
mod db;
mod heartbeat;
mod internal_message_handler;
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

/// Setup tracing - warning this can write huge amounts to disk
#[cfg(debug_assertions)]
fn setup_tracing(app_dir: &PathBuf) -> Result<(), AppError> {
    let level = Level::DEBUG;
    let logfile = tracing_appender::rolling::never(app_dir, "obliqoro.log");

    let log_fmt = t_fmt::Layer::default()
        .json()
        .flatten_event(true)
        .with_writer(logfile);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(true)
            .with_line_number(true)
            .with_max_level(level)
            .finish()
            .with(log_fmt),
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
        }
    }
}

/// Setup tracing - warning this can write huge amounts to disk
#[cfg(not(debug_assertions))]
fn setup_tracing(_app_dir: &PathBuf) -> Result<(), AppError> {
    let level = Level::INFO;
    let log_fmt = t_fmt::Layer::default().json().flatten_event(true);

    match tracing::subscriber::set_global_default(
        t_fmt::Subscriber::builder()
            .with_file(false)
            .with_line_number(true)
            .with_max_level(level)
            .finish()
            .with(log_fmt),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Internal("Unable to start tracing".to_owned()))
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
            error!("{e:?}");
            std::process::exit(1);
        }
        Ok(app_state) => {
            // TODO change this to just an Arc<ApplicationState>, and use a message bus everywhere?
            let state = Arc::new(Mutex::new(app_state));
            let (init_state, internal_state) = (Arc::clone(&state), Arc::clone(&state));
            let (event_sx, close_sx, handler_sx, tray_sx, instance_sx) =
                (sx.clone(), sx.clone(), sx.clone(), sx.clone(), sx.clone());

            #[allow(unused_variables)]
            let app_builder = tauri::Builder::default()
                .manage(state)
                .setup(|app| {
                    #[cfg(debug_assertions)]
                    {
                        if let Some(main_window) = app.get_window(ObliqoroWindow::Main.as_str()) {
                            main_window.open_devtools();
                        }
                    }
                    Ok(())
                })
                .system_tray(tray)
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
                // put all this in the handlers mod, then just import one thing?
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
                    error!("{:?}", e);
                    error!("Unable to build application");
                }
            }
        }
    }
    Ok(())
}
