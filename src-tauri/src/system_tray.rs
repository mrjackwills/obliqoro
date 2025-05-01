use crate::{
    internal_message_handler::{InternalMessage, WindowVisibility},
    request_handlers::FrontEnd,
    ObliqoroWindow,
};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MenuItem {
    Session,
    Next,
    Pause,
    Quit,
    Settings,
}

impl MenuItem {
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
    if !enable {
        app.get_window(ObliqoroWindow::Main.as_str())
            .and_then(|window| {
                window
                    .app_handle()
                    .tray_handle()
                    .try_get_item(MenuItem::Next.get_id())
                    .and_then(|item| item.set_title("on a break").ok())
            });
    }

    for i in [
        MenuItem::Quit,
        MenuItem::Settings,
        MenuItem::Pause,
        MenuItem::Next,
        MenuItem::Session,
    ] {
        app.app_handle()
            .tray_handle()
            .try_get_item(i.get_id())
            .and_then(|item| item.set_enabled(enable).ok());
    }
}

/// Change the system tray icon when paused & unpaused
pub fn set_icon(app: &tauri::AppHandle, paused: bool) {
    let icon = if paused {
        include_bytes!("../icons/icon_paused.png").to_vec()
    } else {
        include_bytes!("../icons/icon.png").to_vec()
    };
    app.tray_handle().set_icon(tauri::Icon::Raw(icon)).ok();
}

pub fn create_system_tray() -> SystemTray {
    let mut tray_menu = SystemTrayMenu::new();
    for i in [
        MenuItem::Quit,
        MenuItem::Settings,
        MenuItem::Pause,
        MenuItem::Next,
        MenuItem::Session,
    ] {
        tray_menu = tray_menu.add_item(CustomMenuItem::new(i.get_id(), i.as_str()));
        if i == MenuItem::Quit || i == MenuItem::Settings {
            tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
        }
    }
    SystemTray::new().with_menu(tray_menu)
}

pub fn on_system_tray_event(event: SystemTrayEvent, sx: &Sender<InternalMessage>) {
    match event {
        SystemTrayEvent::DoubleClick { .. } => {
            sx.send(InternalMessage::Window(WindowVisibility::Toggle))
                .ok();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            x if x == MenuItem::Settings.get_id() => {
                sx.send(InternalMessage::ToFrontEnd(FrontEnd::GoToSettings))
                    .ok();
            }
            x if x == MenuItem::Quit.get_id() => {
                sx.send(InternalMessage::Window(WindowVisibility::Close))
                    .ok();
            }
            x if x == MenuItem::Pause.get_id() => {
                sx.send(InternalMessage::Pause).ok();
            }
            _ => (),
        },
        _ => (),
    }
}
