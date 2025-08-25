use tauri::WebviewWindow;
use tauri::{AppHandle, Manager};

use crate::MAIN_WINDOW;

pub struct WindowAction;

impl WindowAction {
    /// Show the window
    /// Linux v Windows, need to handle fullscreen & resize on each platform differently
    #[cfg(target_os = "windows")]
    fn show(window: &WebviewWindow, fullscreen: bool) {
        if fullscreen {
            window.set_fullscreen(true).ok();
        } else {
            window.center().ok();
        }
        window.show().ok();
    }

    /// Show the window
    /// see github issue #1
    #[cfg(not(target_os = "windows"))]
    fn show(window: &WebviewWindow, _fullscreen: bool) {
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

    /// Change from full screen to the standard window size
    fn _remove_fullscreen(window: &WebviewWindow) {
        window.set_fullscreen(false).ok();
    }

    /// Hide window
    fn hide(window: &WebviewWindow, fullscreen: bool) {
        // if fullscreen {
        //     Self::_remove_fullscreen(window);
        // }
        window.hide().ok();
        window.center().ok();
        if fullscreen {
            Self::_remove_fullscreen(window);
        }
    }

    /// show window
    pub fn show_window(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
            // window.sh
            Self::show(&window, fullscreen);
        }
    }

    /// hide window
    pub fn hide_window(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
            Self::hide(&window, fullscreen);
        }
    }

    /// Toggle the visible of the main window based on current visibility
    pub fn toggle_visibility(app: &AppHandle, fullscreen: bool) {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
            match window.is_visible() {
                Ok(true) => Self::hide(&window, fullscreen),
                Ok(false) => Self::show(&window, fullscreen),
                Err(_) => app.exit(1),
            }
        }
    }

    /// Change from full screen to the standard window size
    pub fn remove_fullscreen(app: &AppHandle) {
        if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
            match window.is_visible() {
                Ok(_) => {
                    Self::_remove_fullscreen(&window);
                }
                Err(_) => app.exit(1),
            }
        }
    }

    // unminimize the main window
    // pub fn unminimize(app: &AppHandle) {
    //     if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
    //         window.unminimize().unwrap_or_default();
    //     }
    // }
}
