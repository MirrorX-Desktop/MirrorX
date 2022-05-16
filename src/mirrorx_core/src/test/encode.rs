use std::time::Duration;

use log::{error, info};

use crate::media::{self};

#[tokio::test]
async fn test_encode() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let (duplicator, duplicator_frame_rx) = media::desktop_duplicator::DesktopDuplicator::new(60)?;
    let (mut encoder, packet_rx) =
        media::video_encoder::VideoEncoder::new("libx264", 60, 1920, 1080)?;
    let (mut decoder, frame_rx) = media::video_decoder::VideoDecoder::new("h264")?;

    std::thread::spawn(move || loop {
        match duplicator_frame_rx.recv() {
            Ok(frame) => {
                info!("duplicator frame len: {}", duplicator_frame_rx.len());
                if let Err(err) = encoder.encode(&frame) {
                    // error!("encode failed: {}", err);
                    break;
                }
            }
            Err(err) => {
                info!("duplicator_frame_rx closeda a ");
                break;
            }
        }
    });

    std::thread::spawn(move || loop {
        match packet_rx.recv() {
            Ok(packet) => {
                info!("packet len: {}", packet_rx.len());
                decoder.decode(&packet);
            }
            Err(err) => {
                info!("packet_rx closed");
                break;
            }
        };
    });

    std::thread::spawn(move || loop {
        match frame_rx.recv() {
            Ok(frame) => {
                info!("decode frame len: {}", frame_rx.len());
                drop(frame);
            }
            Err(err) => {
                info!("frame_rx closed");
                break;
            }
        };
    });

    info!("start capture");
    duplicator.start_capture();
    tokio::time::sleep(Duration::from_secs(3600)).await;
    duplicator.stop_capture();
    info!("stop capture");

    Ok(())
}
