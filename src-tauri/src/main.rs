#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// TODO remove me
#![allow(
    unused,
    unused_imports,
    clippy::unwrap_used,
    clippy::missing_const_for_fn
)]

use application_state::ApplicationState;
use backend_message_handler::{start_message_handler, InternalMessage, WindowVisibility};
use heartbeat::heartbeat_process;
use parking_lot::Mutex;
use std::sync::{mpsc::Receiver, Arc};

#[cfg(debug_assertions)]
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

pub type TauriState<'a> = tauri::State<'a, Arc<Mutex<ApplicationState>>>;

const SYSTEM_TRAY_ID: &str = "obliqoro_system_tray";

// Change this to a const: &str
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
    let (sx, rx) = tokio::sync::broadcast::channel(128);
    let (sx1, sx2, sx3, sx4, sx5) = (sx.clone(), sx.clone(), sx.clone(), sx.clone(), sx.clone());

    let app_rename_me = Builder::default()
        .setup(|app| {
            // #[cfg(debug_assertions)]
            // {
            //     if let Some(main_window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
            //         main_window.open_devtools();
            //     }
            // }

            let app_data_dir = tauri::path::PathResolver::app_data_dir(app.path()).unwrap();
            let (temp_tx, tmp_rx) = std::sync::mpsc::channel();
            let system_tray_menu = system_tray::create_system_tray(app.app_handle());

            // // TODO - Tokio spawn here?
            tokio::spawn(async move {
            let Ok(state)  = ApplicationState::new(app_data_dir, sx, system_tray_menu).await else{
				std::process::exit(1);

            };
            temp_tx.send(state).ok();
		});
            let Ok(state) = tmp_rx.recv() else {
            std::process::exit(1);
            };

            let state = Arc::new(Mutex::new(state));

			let t1 = Arc::clone(&state);
			let t2 = Arc::clone(&state);

            //    loop {
            app.manage(state);
            // //    }
            println!("manage is called");
            // 	println!("about to call heartbeat");
            	heartbeat_process(t1);
            start_message_handler(app.app_handle(), t2, rx, sx4);
            Ok(())
        })
        // .manage(state)
        // .on_tray_icon_event(|app_handle, event| {
        //     system_tray::on_system_tray_event(event, &sx.clone())
        // })
        // .on_window_event(move |_window, event| match event {
        //     tauri::WindowEvent::CloseRequested { api, .. } => {
        //         api.prevent_close();
        //         sx1.send(InternalMessage::Window(WindowVisibility::Hide))
        //             .ok();
        //     }
        //     tauri::WindowEvent::Moved(val) => {
        //         if val.x <= -32000 && val.y <= -32000 {
        //             sx2
        //                 .send(InternalMessage::Window(WindowVisibility::Minimize))
        //                 .ok();
        //         }
        //     }
        //     _ => (),
        // })
        // .invoke_handler(tauri::generate_handler![
        //     request_handlers::init,
        //     request_handlers::minimize,
        //     request_handlers::open_database_location,
        //     request_handlers::pause_after_break,
        //     request_handlers::reset_settings,
        //     request_handlers::set_settings,
        //     request_handlers::toggle_pause,
        // ])
        // .plugin(tauri_plugin_single_instance::init(move |_app, _argv, _cwd| {
        //     sx3
        //         .send(InternalMessage::Window(WindowVisibility::Show))
        //         .ok();
        // }))
        // .event
        .build(tauri::generate_context!());

	// .run

    match app_rename_me {
        Ok(app) => {
            app.run(move |app, event| {
                // app.manage(app.state::<TauriState>().c);

                println!("in callback");
                // heartbeat_process(&app.state());
                // start_message_handler(app.app_handle(), app.state(), rx, sx4);
                if let tauri::RunEvent::ExitRequested { api, .. } = event {
                    sx5.send(InternalMessage::Window(WindowVisibility::Hide))
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
    Ok(())
}

// let ctx = tauri::generate_context!();

// // Fix me
// // let t = tauri::path::PathResolver::app_data_dir(ctx).unwrap();
// // match ApplicationState::new(t, &sx).await {
// //     Err(e) => {
// //         tracing::error!("{e:?}");
// //         std::process::exit(1);
// //     }
// //     Ok(app_state) => {
//         // TODO change this to just an Arc<ApplicationState>, and use a message bus everywhere?
//         // Application state could just be an Arc<Sx<InternalMessage>
//         // let state = Arc::new(Mutex::new(app_state));
//         // let (init_state, internal_state) = (Arc::clone(&state), Arc::clone(&state));
//         let (event_sx, close_sx, handler_sx, tray_sx, instance_sx) =
//             (sx.clone(), sx.clone(), sx.clone(), sx.clone(), sx.clone());

//         #[allow(unused_variables)]
//         let app_builder = tauri::Builder::default()
//             .plugin(tauri_plugin_shell::init())
//             .manage(state)
//             .setup(|app| {
//                 #[cfg(debug_assertions)]
//                 {
//                     if let Some(main_window) = app.get_webview_window(ObliqoroWindow::Main.as_str()) {
//                         main_window.open_devtools();
//                     }
//                 }
//                 Ok(())
//             })
// 			// todo fix me

//             // .system_tray(tray)
// 			// .tr
//             // .on_system_tray_event(move |_app, event| {
//             //     system_tray::on_system_tray_event(event, &tray_sx);
//             // })
//             .on_window_event(move |handler| match handler.event() {
//                 tauri::WindowEvent::CloseRequested { api, .. } => {
//                     api.prevent_close();
//                     event_sx
//                         .send(InternalMessage::Window(WindowVisibility::Hide))
//                         .ok();
//                 }
//                 tauri::WindowEvent::Moved(val) => {
//                     if val.x <= -32000 && val.y <= -32000 {
//                         event_sx
//                             .send(InternalMessage::Window(WindowVisibility::Minimize))
//                             .ok();
//                     }
//                 }
//                 _ => (),
//             })
//             .invoke_handler(tauri::generate_handler![
//                 request_handlers::init,
//                 request_handlers::minimize,
//                 request_handlers::open_database_location,
//                 request_handlers::pause_after_break,
//                 request_handlers::reset_settings,
//                 request_handlers::set_settings,
//                 request_handlers::toggle_pause,
//             ])
//             .plugin(tauri_plugin_single_instance::init(move |app, argv, cwd| {
//                 instance_sx
//                     .send(InternalMessage::Window(WindowVisibility::Show))
//                     .ok();
//             }))
//             .build(tauri::generate_context!());

//         match app_builder {
//             Ok(app) => {
//                 heartbeat_process(&init_state);
//                 start_message_handler(&app, internal_state, rx, handler_sx);
//                 app.run(move |_app, event| {
//                     if let tauri::RunEvent::ExitRequested { api, .. } = event {
//                         close_sx
//                             .send(InternalMessage::Window(WindowVisibility::Hide))
//                             .ok();
//                         api.prevent_exit();
//                     }
//                 });
//             }
//             Err(e) => {
//                 tracing::error!("{:?}", e);
//                 tracing::error!("Unable to build application");
//             }
//         // }
//     // }
// }
//     Ok(())
// }
