use crate::{
    internal_message_handler::{Emitter, InternalMessage, WindowVisibility},
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
        if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
            window
                .app_handle()
                .tray_handle()
                .get_item(MenuItem::Next.get_id())
                .set_title("on a break")
                .unwrap_or(());
        }
    }

    for i in [
        MenuItem::Quit,
        MenuItem::Settings,
        MenuItem::Pause,
        MenuItem::Next,
        MenuItem::Session,
    ] {
        app.tray_handle()
            .get_item(i.get_id())
            .set_enabled(enable)
            .unwrap_or_default();
    }
}

// refactor into own mod
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
                .unwrap_or_default();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            x if x == MenuItem::Settings.get_id() => {
                sx.send(InternalMessage::Emit(Emitter::GoToSettings))
                    .unwrap_or_default();
            }
            x if x == MenuItem::Quit.get_id() => {
                sx.send(InternalMessage::Window(WindowVisibility::Close))
                    .unwrap_or_default();
            }
            x if x == MenuItem::Pause.get_id() => {
                sx.send(InternalMessage::Pause).unwrap_or_default();
            }
            _ => (),
        },
        _ => (),
    }
}
