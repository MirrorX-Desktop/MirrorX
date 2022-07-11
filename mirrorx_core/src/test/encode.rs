use scopeguard::defer;
use std::time::Duration;
use tracing::{error, info};

use crate::{
    component::{desktop::Duplicator, monitor, video_encoder::VideoEncoder},
    service::endpoint::processor::{
        desktop::start_desktop_capture_process, video::start_video_encode_process,
    },
};

#[test]
fn test_capture_and_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let monitors = monitor::get_active_monitors()?;
    let monitor = match monitors.iter().find(|v| v.is_primary) {
        Some(v) => v,
        None => {
            return Err(anyhow::anyhow!("no primary monitor"));
        }
    };

    let (exit_tx, exit_rx) = crossbeam::channel::unbounded();
    let (packet_tx, mut packet_rx) = tokio::sync::mpsc::channel(16);
    let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(16);

    start_desktop_capture_process(
        String::from("remote_test"),
        exit_tx.clone(),
        exit_rx.clone(),
        capture_frame_tx,
        &monitor.id,
        monitor.refresh_rate,
    )?;

    start_video_encode_process(
        String::from("remote_test"),
        exit_tx.clone(),
        exit_rx.clone(),
        monitor.width as i32,
        monitor.height as i32,
        monitor.refresh_rate as i32,
        capture_frame_rx,
        packet_tx,
    )?;

    std::thread::spawn(move || {
        let start_time = std::time::Instant::now();
        defer! {
            info!(elapsed = ?start_time.elapsed(), "receive dudration");
        }

        loop {
            match packet_rx.blocking_recv() {
                Some(packet) => {}
                None => break,
            }
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(30));
    Ok(())
}
