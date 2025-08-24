use std::sync::Arc;
use tokio::sync::broadcast::Sender;

use crate::backend_message_handler::InternalMessage;

/// Spawn off a tokio thread, that loops continually, well with a 250ms pause between each loop
/// TODO remove application state, just have xs?
/// The outer tread is saved into ApplicationState, so that it can be cancelled at any time
pub fn heartbeat_process(sx: &Sender<InternalMessage>) {
    let (sx, thread_sx) = (sx.clone(), sx.clone());
    let heartbeat_process = Arc::new(tokio::task::spawn(async move {
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
            thread_sx
                .send(InternalMessage::Hearbteat(
                    crate::backend_message_handler::Hearbteat::OnHeartbeat(cpu_usage),
                ))
                .ok();
            thread_sx
                .send(InternalMessage::Hearbteat(
                    crate::backend_message_handler::Hearbteat::UpdateTimer,
                ))
                .ok();
            tokio::time::sleep(std::time::Duration::from_millis(
                u64::try_from(250u128.saturating_sub(loop_instant.elapsed().as_millis()))
                    .unwrap_or(250),
            ))
            .await;
            loop_instant = std::time::Instant::now();
        }
    }));
    sx.send(InternalMessage::Hearbteat(
        crate::backend_message_handler::Hearbteat::Update(heartbeat_process),
    ))
    .ok();
}
