use std::sync::{Arc, Mutex};

use crate::{
    application_state::ApplicationState,
    backend_message_handler::{InternalMessage, WindowVisibility},
    request_handlers::MsgToFrontend,
    ObliqoroWindow, TauriState, SYSTEM_TRAY_ID,
};
use tauri::{
    image::Image, menu::{Menu, MenuEvent, MenuItem}, tray::{TrayIcon, TrayIconEvent}, AppHandle, EventLoopMessage, Manager, Wry
};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuEntry {
    Session,
    Next,
    Pause,
    Quit,
    Settings,
}

impl MenuEntry {
    pub const fn get_id(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Next => "next",
            Self::Pause => "pause",
            Self::Quit => "quit",
            Self::Settings => "settings",
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Next => "loading...",
            Self::Pause => "Pause",
            Self::Quit => "Quit",
            Self::Settings => "Settings",
        }
    }
}

pub fn menu_enabled(app: &tauri::AppHandle, enable: bool) {

	let menu = gen_menu_all_enabled(app.app_handle());
    if !enable {
		// send internal message try "on a break"
    //     app.get_webview_window(ObliqoroWindow::Main.as_str())
    //         .and_then(|window| {
    //             window
    //                 .app_handle()
    //                 .tray_by_id(SYSTEM_TRAY_ID)
    //                 .and_then(|i| {
	// 					// i.set_menu(menu)

    //                     // MenuItem::with_id(i, MenuEntry::Next.get_id(), text, enabled, accelerator)
    //                     // MenuItem::
    //                     // i.app_handle().manage(state)
    //                 });
    //             // .
    //             // .try_get_item(MenuItem::Next.get_id())
    //             // .and_then(|item| item.set_title("on a break").ok())
    //             Some(())
    //         });
    // }
	}
    for i in [
        MenuEntry::Quit,
        MenuEntry::Settings,
        MenuEntry::Pause,
        MenuEntry::Next,
        MenuEntry::Session,
    ] {
			// send internal message try "set_enabled"
        // app.app_handle()
        // .tray_by_id(i.get_id())
        // .and_then(|item| item.on_menu_event(|i|i.i) tem.set_enabled(enable).ok());
    }
}

/// Change the system tray icon when paused & unpaused
pub fn set_icon(app: &tauri::AppHandle, paused: bool) {
    let icon = if paused {
        include_bytes!("../icons/icon_paused.png").to_vec()
    } else {
        include_bytes!("../icons/icon.png").to_vec()
    };
    app.tray_by_id(SYSTEM_TRAY_ID)
        .and_then(|i| i.set_icon(Image::from_bytes(&icon).ok()).ok());
}

fn gen_menu_all_enabled(app_handle: &AppHandle) -> Menu<Wry> {
	    // TODO macro here?
    let entry_quit = MenuItem::with_id(
        app_handle,
        MenuEntry::Quit.get_id(),
        MenuEntry::Quit.as_str(),
        true,
        None::<&str>,
    )
    .unwrap();
    let entry_settings = MenuItem::with_id(
        app_handle,
        MenuEntry::Settings.get_id(),
        MenuEntry::Settings.as_str(),
        true,
        None::<&str>,
    )
    .unwrap();
    let entry_pause = MenuItem::with_id(
        app_handle,
        MenuEntry::Pause.get_id(),
        MenuEntry::Pause.as_str(),
        true,
        None::<&str>,
    )
    .unwrap();
    let entry_next = MenuItem::with_id(
        app_handle,
        MenuEntry::Next.get_id(),
        MenuEntry::Next.as_str(),
        true,
        None::<&str>,
    )
    .unwrap();
    let entry_session = MenuItem::with_id(
        app_handle,
        MenuEntry::Session.get_id(),
        MenuEntry::Session.as_str(),
        true,
        None::<&str>,
    )
    .unwrap();

    Menu::with_items(
        app_handle,
        &[
            &entry_quit,
            &entry_settings,
            &entry_pause,
            &entry_next,
            &entry_session,
        ],
    )
    .unwrap()
}

pub fn create_system_tray(app_handle: &AppHandle) -> Menu<Wry>{
	let menu = gen_menu_all_enabled(app_handle);
    tauri::tray::TrayIconBuilder::with_id(SYSTEM_TRAY_ID)
        .menu(&menu)
        .on_menu_event(|a, b| on_menu_entry_event(a, b))
        .on_tray_icon_event(on_tray_event)
        .build(app_handle)
        .unwrap();
	menu
}

fn on_tray_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::DoubleClick { .. } => {
            tray.app_handle()
                .state::<TauriState>()
                .lock()
                .sx
                .send(InternalMessage::Window(WindowVisibility::Toggle))
                .ok();
        }
        _ => (),
    }
}

/// Handle interaction events on the systemtray icon/menu
fn on_menu_entry_event(app_handle: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        val if val == MenuEntry::Settings.get_id() => {
            app_handle
                .state::<TauriState>()
                .lock()
                .sx
                .send(InternalMessage::ToFrontEnd(MsgToFrontend::GoToSettings))
                .ok();
        }
        val if val == MenuEntry::Quit.get_id() => {
            app_handle
                .state::<TauriState>()
                .lock()
                .sx
                .send(InternalMessage::Window(WindowVisibility::Close))
                .ok();
        }
        val if val == MenuEntry::Pause.get_id() => {
            app_handle
                .state::<TauriState>()
                .lock()
                .sx
                .send(InternalMessage::Pause)
                .ok();
        }
        _ => (),
    }
    // MenuEvent::DoubleClick { .. } => {
    //     sx.send(InternalMessage::Window(WindowVisibility::Toggle))
    //         .ok();
    // }
    // MenuEvent::MenuItemClick { id, .. } => match id.as_str() {
    //     x if x == MenuEntry::Settings.get_id() => {
    //         sx.send(InternalMessage::ToFrontEnd(MsgToFrontend::GoToSettings))
    //             .ok();
    //     }
    //     x if x == MenuEntry::Quit.get_id() => {
    //         sx.send(InternalMessage::Window(WindowVisibility::Close))
    //             .ok();
    //     }
    //     x if x == MenuEntry::Pause.get_id() => {
    //         sx.send(InternalMessage::Pause).ok();
    //     }
    //     _ => (),
    // },
    // _ => (),
    // }
}
