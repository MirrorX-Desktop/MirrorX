use std::time::Duration;
use tracing::{error, info};

use crate::component::{desktop::Duplicator, monitor, video_encoder::VideoEncoder};

#[test]
fn test_capture_and_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let encoder_name: &str;

    if cfg!(target_os = "macos") {
        encoder_name = "h264_videotoolbox";
    } else if cfg!(target_os = "windows") {
        encoder_name = "libx264";
    } else {
        panic!("unsupported platform");
    }

    let mut encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

    encoder.set_opt("profile", "high", 0)?;
    encoder.set_opt("level", "5.2", 0)?;

    if encoder_name == "libx264" {
        encoder.set_opt("preset", "ultrafast", 0)?;
        encoder.set_opt("tune", "zerolatency", 0)?;
        encoder.set_opt("sc_threshold", "499", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let monitors = monitor::get_active_monitors()?;
    let monitor = match monitors.iter().find(|v| v.is_primary) {
        Some(v) => v,
        None => {
            return Err(anyhow::anyhow!("no primary monitor"));
        }
    };

    let (capture_tx, capture_rx) = crossbeam::channel::bounded(1);
    let mut duplicator = Duplicator::new(capture_tx, &monitor.id, monitor.refresh_rate)?;

    let (error_tx, error_rx) = crossbeam::channel::bounded(1);

    let encode_error_tx = error_tx.clone();
    std::thread::spawn(move || {
        // make sure the media_transmission after start_media_transmission send
        std::thread::sleep(Duration::from_secs(1));

        if let Err(err) = duplicator.start() {
            error!(?err, "DesktopDuplicator start capture failed");
            return;
        }

        loop {
            let capture_frame = match capture_rx.recv() {
                Ok(frame) => {
                    info!(width=?frame.width,height=?frame.height,luminance_len=?frame.luminance_buffer.len(),luminance_stride=?frame.luminance_stride,chrominance_len=?frame.chrominance_buffer.len(),chrominance_stride=?frame.chrominance_stride,"capture");
                    frame
                }
                Err(err) => {
                    tracing::error!(?err, "capture_rx.recv");
                    break;
                }
            };

            // encode will block current thread until capture_frame released (FFMpeg API 'avcodec_send_frame' finished)
            match encoder.encode(capture_frame) {
                Ok(frames) => {
                    for frame in frames {
                        info!(len=?frame.len(), "encoded frame");
                    }
                }
                Err(err) => {
                    error!(err=?err,"video encode failed");
                    let _ = encode_error_tx.try_send(err);
                }
            };
        }

        duplicator.stop();
    });

    // std::thread::spawn(move || loop {
    //     match packet_rx.recv() {
    //         Ok(packet) => {
    //             info!(
    //                 packet_data_length = packet.0.len(),
    //                 "receive encode video packet"
    //             );
    //         }
    //         Err(err) => {
    //             error!(err=?err, "packet receive failed");
    //             let _ = error_tx.try_send(crate::error::MirrorXError::Other(anyhow::anyhow!(err)));
    //             break;
    //         }
    //     };
    // });

    if let Ok(err) = error_rx.recv_timeout(Duration::from_secs(10)) {
        Err(anyhow::anyhow!(err))
    } else {
        Ok(())
    }
}
