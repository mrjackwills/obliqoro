use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

use crate::{
    application_state::{ApplicationState, SessionStatus},
    internal_message_handler::{BreakMessage, Emitter, InternalMessage},
};

const ONE_SECOND_AS_MS: u64 = 1000;

/// Spawn off a tokio thread, that loops continually, is essentially the internal timer that powers the whole application
/// Is save in the state struct, so that it can be aborted when settings change etc
pub fn tick_process(state: &Arc<Mutex<ApplicationState>>, sx: Sender<InternalMessage>) {
    if let Some(x) = &state.lock().tick_process {
        x.abort();
    }

    let mut last_updated = std::time::Instant::now();
    let mut menu_updated = std::time::Instant::now();
    let spawn_state = Arc::clone(state);

    spawn_state
        .lock()
        .sx
        .send(InternalMessage::UpdateMenuTimer)
        .unwrap_or_default();
    state.lock().tick_process = Some(tokio::task::spawn(async move {
        loop {
            let paused = spawn_state.lock().get_paused();
            if !paused && last_updated.elapsed().as_secs() >= 1 {
                let to_run = spawn_state.lock().session_status;
                let next_break_in = spawn_state.lock().tick();
                match to_run {
                    SessionStatus::Break(_) => {
                        spawn_state
                            .lock()
                            .sx
                            .send(InternalMessage::Emit(Emitter::OnBreak))
                            .unwrap_or_default();
                        if next_break_in < 1 {
                            spawn_state
                                .lock()
                                .sx
                                .send(InternalMessage::Break(BreakMessage::End))
                                .unwrap_or_default();
                        }
                    }
                    SessionStatus::Work => {
                        if menu_updated.elapsed().as_secs() >= 60 {
                            spawn_state
                                .lock()
                                .sx
                                .send(InternalMessage::UpdateMenuTimer)
                                .unwrap_or_default();
                            menu_updated = std::time::Instant::now();
                        }
                        if next_break_in < 1 {
                            sx.send(InternalMessage::Break(BreakMessage::Start))
                                .unwrap_or_default();
                        }
                    }
                }
            }
            last_updated = std::time::Instant::now();

            let elapsed = u64::try_from(last_updated.elapsed().as_millis()).unwrap_or(0);
            let to_sleep = if ONE_SECOND_AS_MS >= elapsed {
                ONE_SECOND_AS_MS - elapsed
            } else {
                ONE_SECOND_AS_MS
            };
            if to_sleep <= ONE_SECOND_AS_MS {
                tokio::time::sleep(std::time::Duration::from_millis(to_sleep)).await;
            }
        }
    }));
}
