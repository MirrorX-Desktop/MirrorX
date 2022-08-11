use crate::{
    component::monitor,
    service::endpoint::{
        message::EndPointMessage,
        processor::{
            desktop::start_desktop_capture_process,
            video::{start_video_decode_process, start_video_encode_process},
        },
    },
};
use tracing::{error, info};

#[tokio::test]
async fn test_capture_and_encode_and_decode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // let monitors = monitor::get_active_monitors()?;
    // let monitor = match monitors.iter().find(|v| v.is_primary) {
    //     Some(v) => v,
    //     None => {
    //         return Err(anyhow::anyhow!("no primary monitor"));
    //     }
    // };

    // let (exit_tx, exit_rx) = async_broadcast::broadcast(1);
    // let (packet_tx, mut packet_rx) = tokio::sync::mpsc::channel(16);
    // let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(1);
    // let (video_frame_tx, video_frame_rx) = tokio::sync::mpsc::channel(16);
    // let (decoded_frame_tx, decoded_frame_rx) = crossbeam::channel::bounded(16);

    // start_desktop_capture_process(
    //     String::from("remote_test"),
    //     exit_tx.clone(),
    //     exit_rx.clone(),
    //     capture_frame_tx,
    //     &monitor.id,
    //     monitor.refresh_rate,
    // )?;

    // start_video_encode_process(
    //     String::from("remote_test"),
    //     exit_tx.clone(),
    //     exit_rx.clone(),
    //     monitor.width as i32,
    //     monitor.height as i32,
    //     monitor.refresh_rate as i32,
    //     capture_frame_rx,
    //     packet_tx,
    // )?;

    // start_video_decode_process(
    //     String::from("remote_test"),
    //     exit_tx,
    //     exit_rx,
    //     monitor.width as i32,
    //     monitor.height as i32,
    //     monitor.refresh_rate as i32,
    //     video_frame_rx,
    //     decoded_frame_tx,
    // )?;

    // tokio::spawn(async move {
    //     loop {
    //         match packet_rx.recv().await {
    //             Some(packet) => {
    //                 if let EndPointMessage::VideoFrame(frame) = packet.message {
    //                     if let Err(err) = video_frame_tx.send(frame).await {
    //                         tracing::error!("video frame send failed ({})", err);
    //                     } else {
    //                         tracing::info!("send video frame");
    //                     }
    //                 }
    //             }
    //             None => break,
    //         }
    //     }
    // });

    // std::thread::spawn(move || loop {
    //     match decoded_frame_rx.recv() {
    //         Ok(frame) => {
    //             tracing::info!("decode decodedframe");
    //         }
    //         Err(err) => panic!("receive decoded frame failed ({})", err),
    //     }
    // });

    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    Ok(())
}
