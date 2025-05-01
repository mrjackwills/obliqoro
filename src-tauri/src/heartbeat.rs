use parking_lot::Mutex;
use std::sync::Arc;

use crate::{application_state::ApplicationState, internal_message_handler::InternalMessage};

/// Spawn off a tokio thread, that loops continually, well with a 250ms pause between each loop
/// The outer tread is saved into ApplicationState, so that it can be cancelled at any time
pub fn heartbeat_process(state: &Arc<Mutex<ApplicationState>>) {
    if let Some(x) = &state.lock().heartbeat_process {
        x.abort();
    }
    let spawn_state = Arc::clone(state);
    state.lock().sx.send(InternalMessage::UpdateMenuTimer).ok();
    state.lock().heartbeat_process = Some(tokio::task::spawn(async move {
        let mut sys = sysinfo::System::new();
        let mut loop_instant = std::time::Instant::now();
        let mut cpu_instant = std::time::Instant::now();

        loop {
            let cpu_usage = if cpu_instant.elapsed().as_millis() >= 1000 {
                sys.refresh_cpu_usage();
                let cpu_usage = sys.global_cpu_usage();
                cpu_instant = std::time::Instant::now();
                Some(cpu_usage)
            } else {
                None
            };

            spawn_state.lock().on_heartbeat(cpu_usage);

            tokio::time::sleep(std::time::Duration::from_millis(
                u64::try_from(250u128.saturating_sub(loop_instant.elapsed().as_millis()))
                    .unwrap_or(250),
            ))
            .await;
            loop_instant = std::time::Instant::now();
        }
    }));
}
