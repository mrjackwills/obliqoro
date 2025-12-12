use async_channel::Sender;
use tokio_util::sync::CancellationToken;

use crate::message_handler::MsgI;

async fn heartbeat_loop(sx: Sender<MsgI>) {
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

        _ = tokio::try_join!(
            sx.send(MsgI::HeartBeat(crate::message_handler::MsgHB::OnHeartbeat(
                cpu_usage,
            ))),
            sx.send(MsgI::HeartBeat(crate::message_handler::MsgHB::UpdateTimer))
        );
        tokio::time::sleep(std::time::Duration::from_millis(
            u64::try_from(250u128.saturating_sub(loop_instant.elapsed().as_millis()))
                .unwrap_or(250),
        ))
        .await;
        loop_instant = std::time::Instant::now();
    }
}

/// Spawn off a tokio thread, that loops continually, well with a 250ms pause between each loop
/// The outer tread is saved into ApplicationState, so that it can be cancelled at any time
pub async fn heartbeat_process(sx: &Sender<MsgI>) {
    let token = CancellationToken::new();
    let (sx, thread_sx, thread_token) = (sx.clone(), sx.clone(), token.clone());

    tokio::task::spawn(async move {
        thread_token
            .run_until_cancelled(heartbeat_loop(thread_sx))
            .await;
    });
    sx.send(MsgI::HeartBeat(crate::message_handler::MsgHB::Update(
        token,
    )))
    .await
    .ok();
}
