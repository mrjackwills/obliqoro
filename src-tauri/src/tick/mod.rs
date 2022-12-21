use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

use crate::{
    application_state::{ApplicationState, SessionStatus},
    internal_message_handler::{BreakMessage, Emitter, InternalMessage},
};

// const ONE_SECOND_AS_MS: u64 = 1000;
// const ONE_MINUTE_AS_MS: u64 = ONE_SECOND_AS_MS * 60;

/// Spawn off a tokio thread, that loops continually, well with a 250ms pause between each loop
/// The timer checking is then spawned off into another thread
/// The outer tread is saved into ApplicationState, so that it can be cancelled at any time
pub fn tick_process(state: &Arc<Mutex<ApplicationState>>, sx: Sender<InternalMessage>) {
    if let Some(x) = &state.lock().tick_process {
        x.abort();
    }

    let menu_updated = Arc::new(Mutex::new(std::time::Instant::now()));
    let spawn_state = Arc::clone(state);

    state
        .lock()
        .sx
        .send(InternalMessage::UpdateMenuTimer)
        .unwrap_or_default();
    state.lock().tick_process = Some(tokio::task::spawn(async move {
        loop {
            let paused = spawn_state.lock().get_paused();
            if !paused {
                let spawn_state = Arc::clone(&spawn_state);
                let sx = sx.clone();
                let menu_updated = Arc::clone(&menu_updated);

                // TODO do all of this in the state itself!
                tokio::spawn(async move {
                    let to_run = spawn_state.lock().session_status;
                    let session_left_in_sec = spawn_state.lock().current_timer_left();
                    match to_run {
                        SessionStatus::Break(_) => {
                            sx.send(InternalMessage::Emit(Emitter::OnBreak))
                                .unwrap_or_default();
                            if session_left_in_sec < 1 {
                                sx.send(InternalMessage::Break(BreakMessage::End))
                                    .unwrap_or_default();
                            }
                        }
                        SessionStatus::Work => {
                                sx.send(InternalMessage::UpdateMenuTimer)
                                    .unwrap_or_default();
                                *menu_updated.lock() = std::time::Instant::now();
                            if session_left_in_sec < 1 {
                                sx.send(InternalMessage::Break(BreakMessage::Start))
                                    .unwrap_or_default();
                            }
                        }
                    }
                });
            }
			// change to 500ms?
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    }));
}
