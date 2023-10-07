use tauri::{AppHandle, Manager};

use crate::ObliqoroWindow;

pub struct WindowAction;

impl WindowAction {
    /// Show the window
    /// Linux v Windows, need to handle fullscreen & resize on each platform differently
    #[cfg(target_os = "windows")]
    fn show(window: &tauri::Window, fullscreen: bool) {
        window.set_fullscreen(fullscreen).ok();
        window.set_resizable(false).ok();
        window.show().ok();
        window.center().ok();
    }

    /// Show the window
    /// see github issue #1
    #[cfg(not(target_os = "windows"))]
    fn show(window: &tauri::Window, fullscreen: bool) {
        if fullscreen {
            if window.is_visible().unwrap_or_default() {
                window.hide().ok();
            }
            window.set_resizable(true).ok();
            window.set_fullscreen(fullscreen).ok();
            // This is the linux fix - dirty, but it seems to work
            std::thread::sleep(std::time::Duration::from_millis(50));
        } else if window.is_resizable().unwrap_or(false) {
            window.set_resizable(false).ok();
        }
        window.show().ok();
        window.center().ok();
    }

    /// Hide window
    fn hide(window: &tauri::Window, fullscreen: bool) {
        if fullscreen {
            window.set_resizable(true).ok();
            window.set_fullscreen(false).ok();
        }
        window.hide().ok();
        window.center().ok();
    }

	/// show window
    pub fn show_window(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
            Self::show(&window, fullscreen);
        }
    }

    /// hide window
    pub fn hide_window(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
            Self::hide(&window, fullscreen);
        }
    }

    /// Toggle the visible of the main window based on current visibility
    pub fn toggle_visibility(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
            match window.is_visible() {
                Ok(true) => Self::hide(&window, fullscreen),
                Ok(false) => Self::show(&window, fullscreen),
                Err(_) => app.exit(1),
            }
        }
    }

    // unminimize the main window
    // pub fn unminimize(app: &AppHandle) {
    //     if let Some(window) = app.get_window(ObliqoroWindow::Main.as_str()) {
    //         window.unminimize().unwrap_or_default();
    //     }
    // }
}