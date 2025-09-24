use std::sync::LazyLock;

use crate::{
    SYSTEM_TRAY_ID,
    message_handler::{MsgFE, MsgI, MsgWV},
};
use tauri::{
    AppHandle, Wry,
    image::Image,
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconEvent,
};
use tokio::sync::broadcast::Sender;

/// Load the Oblique Stratergies into a Lazylock vec
#[allow(clippy::unwrap_used)]
static ICON_PAUSE: LazyLock<Image> =
    LazyLock::new(|| Image::from_bytes(include_bytes!("../../icons/icon_paused.png")).unwrap());

/// Load the Oblique Stratergies into a Lazylock vec
#[allow(clippy::unwrap_used)]
static ICON_RUNNING: LazyLock<Image> =
    LazyLock::new(|| Image::from_bytes(include_bytes!("../../icons/icon.png")).unwrap());

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

/// Change the text of the Next item, and if on break, disable all items, else enable all
pub fn change_menu_entry_status(system_tray_menu: &Menu<Wry>, enable: bool) {
    if !enable {
        system_tray_menu
            .get(MenuEntry::Next.get_id())
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_text("on a break").ok()));
    }
    for i in [
        MenuEntry::Quit,
        MenuEntry::Settings,
        MenuEntry::Pause,
        MenuEntry::Next,
        MenuEntry::Session,
    ] {
        system_tray_menu
            .get(i.get_id())
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_enabled(enable).ok()));
    }
}

/// Change the system tray icon when paused & unpaused
pub fn set_icon(app: &tauri::AppHandle, paused: bool) {
    let icon = if paused {
        ICON_PAUSE.clone()
    } else {
        ICON_RUNNING.clone()
    };
    app.tray_by_id(SYSTEM_TRAY_ID)
        .and_then(|i| i.set_icon(Some(icon)).ok());
}

fn gen_menu_all_enabled(app_handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    let entry_quit = MenuItem::with_id(
        app_handle,
        MenuEntry::Quit.get_id(),
        MenuEntry::Quit.as_str(),
        true,
        None::<&str>,
    )?;
    let entry_settings = MenuItem::with_id(
        app_handle,
        MenuEntry::Settings.get_id(),
        MenuEntry::Settings.as_str(),
        true,
        None::<&str>,
    )?;
    let entry_pause = MenuItem::with_id(
        app_handle,
        MenuEntry::Pause.get_id(),
        MenuEntry::Pause.as_str(),
        true,
        None::<&str>,
    )?;
    let entry_next = MenuItem::with_id(
        app_handle,
        MenuEntry::Next.get_id(),
        MenuEntry::Next.as_str(),
        true,
        None::<&str>,
    )?;
    let entry_session = MenuItem::with_id(
        app_handle,
        MenuEntry::Session.get_id(),
        MenuEntry::Session.as_str(),
        true,
        None::<&str>,
    )?;

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
}

pub fn create_system_tray(
    app_handle: &tauri::AppHandle,
    sx: Sender<MsgI>,
) -> Result<Menu<Wry>, tauri::Error> {
    let s1 = sx.clone();
    let menu = gen_menu_all_enabled(app_handle)?;
    tauri::tray::TrayIconBuilder::with_id(SYSTEM_TRAY_ID)
        .icon(ICON_RUNNING.clone())
        .show_menu_on_left_click(false)
        .on_tray_icon_event(move |_, event| on_tray_event(event, sx.clone()))
        .menu(&menu)
        .on_menu_event(move |_, menu_event| on_menu_entry_event(&menu_event, &s1))
        .build(app_handle)?;
    Ok(menu)
}

#[allow(clippy::needless_pass_by_value)]
fn on_tray_event(event: TrayIconEvent, sx: Sender<MsgI>) {
    if let TrayIconEvent::DoubleClick { .. } = event {
        sx.send(MsgI::Window(MsgWV::Toggle)).ok();
    }
}

/// Handle interaction events on the systemtray icon/menu
fn on_menu_entry_event(event: &MenuEvent, sx: &Sender<MsgI>) {
    match event.id.as_ref() {
        val if val == MenuEntry::Settings.get_id() => {
            sx.send(MsgI::ToFrontEnd(MsgFE::GoToSettings)).ok();
        }
        val if val == MenuEntry::Quit.get_id() => {
            sx.send(MsgI::Window(MsgWV::Close)).ok();
        }
        val if val == MenuEntry::Pause.get_id() => {
            sx.send(MsgI::Pause).ok();
        }
        _ => (),
    }
}
