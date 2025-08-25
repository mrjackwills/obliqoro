use crate::{
    application_state::{system_tray::MenuEntry, ApplicationState},
    message_handler::{MsgFE, MsgI},
};

pub struct MenuManipulation;

impl MenuManipulation {
    // Update the taskbar to display how many sessions before next long break,
    // and send internal message, to send message to front end to update settings in pinia
    fn update_session_number(state: &ApplicationState) {
        let title = state.get_sessions_before_long_title();
        state
            .get_menu_entry(MenuEntry::Session)
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
        state.send(MsgI::ToFrontEnd(MsgFE::SessionsBeforeLong));
    }

    /// Update the systemtray next break in text, and emit to frontend to next break timer
    fn update_next_break(state: &ApplicationState) {
        let title = state.get_next_break_title();
        state
            .get_menu_entry(MenuEntry::Next)
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
        state.send(MsgI::ToFrontEnd(MsgFE::NextBreak));
    }

    /// Update the systemtray `Puased/Resume` item
    pub fn update_pause(state: &ApplicationState, paused: bool) {
        let title = if paused {
            "Resume"
        } else {
            MenuEntry::Pause.as_str()
        };

        state
            .get_menu_entry(MenuEntry::Next)
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_enabled(!paused).ok()));
        state
            .get_menu_entry(MenuEntry::Session)
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_enabled(!paused).ok()));

        state
            .get_menu_entry(MenuEntry::Pause)
            .and_then(|i| i.as_menuitem().and_then(|i| i.set_text(title).ok()));
    }

    /// Update all menu items
    pub fn update_all(state: &ApplicationState) {
        Self::update_next_break(state);
        Self::update_session_number(state);
    }
}
