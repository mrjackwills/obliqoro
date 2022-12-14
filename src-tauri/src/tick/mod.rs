use parking_lot::Mutex;
use std::sync::Arc;

use crate::{application_state::ApplicationState, internal_message_handler::InternalMessage};

/// Spawn off a tokio thread, that loops continually, well with a 250ms pause between each loop
/// The timer checking is then spawned off into another thread
/// The outer tread is saved into ApplicationState, so that it can be cancelled at any time
pub fn tick_process(state: &Arc<Mutex<ApplicationState>>) {
    if let Some(x) = &state.lock().tick_process {
        x.abort();
    }
    let spawn_state = Arc::clone(state);
    state
        .lock()
        .sx
        .send(InternalMessage::UpdateMenuTimer)
        .unwrap_or_default();
    state.lock().tick_process = Some(tokio::task::spawn(async move {
        loop {
            let spawn_state = Arc::clone(&spawn_state);
            tokio::spawn(async move {
                spawn_state.lock().tick_process();
            });
            // 500ms?
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    }));
}
