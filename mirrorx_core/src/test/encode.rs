use scopeguard::defer;
use std::time::Duration;
use tracing::{error, info};

use crate::{
    component::{desktop::Duplicator, monitor},
    service::endpoint::processor::{
        desktop::start_desktop_capture_process, video::start_video_encode_process,
    },
};

#[tokio::test]
async fn test_capture_and_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let monitors = monitor::get_active_monitors()?;
    let monitor = match monitors.iter().find(|v| v.is_primary) {
        Some(v) => v,
        None => {
            return Err(anyhow::anyhow!("no primary monitor"));
        }
    };

    let (exit_tx, exit_rx) = async_broadcast::broadcast(1);
    let (packet_tx, mut packet_rx) = tokio::sync::mpsc::channel(16);
    let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(16);

    start_desktop_capture_process(
        String::from("remote_test"),
        exit_tx.clone(),
        exit_rx.clone(),
        capture_frame_tx,
        Some(monitor.id.clone()),
        monitor.refresh_rate,
    )?;

    start_video_encode_process(
        String::from("remote_test"),
        exit_tx,
        exit_rx,
        monitor.width as i32,
        monitor.height as i32,
        monitor.refresh_rate as i32,
        capture_frame_rx,
        packet_tx,
    )?;

    tokio::spawn(async move {
        loop {
            match packet_rx.recv().await {
                Some(packet) => {
                    tracing::info!("recevie packet");
                }
                None => {
                    tracing::info!("break loop");
                    break;
                }
            }
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    Ok(())
}
