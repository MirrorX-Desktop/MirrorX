use crate::{
    component::{
        desktop::Frame,
        video_decoder::{DecodedFrame, VideoDecoder},
        video_encoder::VideoEncoder,
    },
    error::MirrorXError,
    service::endpoint::message::*,
};
use crossbeam::channel::{Receiver, Sender, TryRecvError};
use scopeguard::defer;
use std::collections::HashMap;
use tracing::{error, info, trace, warn};

pub fn start_video_encode_process(
    remote_device_id: String,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    width: i32,
    height: i32,
    fps: i32,
    capture_frame_rx: Receiver<Frame>,
    packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
) -> Result<(), MirrorXError> {
    let encoder_name = if cfg!(target_os = "macos") {
        "h264_videotoolbox"
    } else if cfg!(target_os = "windows") {
        "libx264"
    } else {
        panic!("unsupported platform")
    };

    let mut encoder = VideoEncoder::new(encoder_name, fps, width, height)?;

    encoder.set_opt("profile", "baseline", 0)?;
    encoder.set_opt("level", "5.2", 0)?;

    if encoder_name == "libx264" {
        encoder.set_opt("preset", "ultrafast", 0)?;
        encoder.set_opt("tune", "zerolatency", 0)?;
        // encoder.set_opt("sc_threshold", "0", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let _ = std::thread::Builder::new()
        .name(format!("video_encode_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                info!(?remote_device_id, "video encode process exit");
                let _ = exit_tx.send(());
            }

            loop {
                match exit_rx.try_recv() {
                    Ok(_) => {
                        info!("process exit channel received signal");
                        break;
                    }
                    Err(err) => {
                        if err == TryRecvError::Disconnected {
                            info!("process exit channel disconnected");
                            break;
                        }
                    }
                };

                let frame = match capture_frame_rx.recv() {
                    Ok(v) => v,
                    Err(_) => {
                        info!("capture channel closed");
                        break;
                    }
                };

                match encoder.encode(frame, &packet_tx) {
                    Ok(frames) => frames,
                    Err(err) => {
                        error!(?err, "video frame decode failed");
                        break;
                    }
                };
            }
        });

    Ok(())
}

pub fn start_video_decode_process(
    remote_device_id: String,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    width: i32,
    height: i32,
    fps: i32,
    video_frame_rx: Receiver<VideoFrame>,
    decoded_frame_tx: Sender<DecodedFrame>,
) -> Result<(), MirrorXError> {
    let (decoder_name, options) = if cfg!(target_os = "macos") {
        ("h264", HashMap::new())
    } else if cfg!(target_os = "windows") {
        (
            "h264_d3d11va",
            HashMap::new(), // HashMap::from([("async_depth", "1"), ("gpu_copy", "on")]),
        )
    } else {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "unsupport platform decode"
        )));
    };

    let decoder = VideoDecoder::new(decoder_name, width, height, fps, options)?;

    let _ = std::thread::Builder::new()
        .name(format!("video_decode_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                info!(?remote_device_id, "video decode process exit");
                let _ = exit_tx.send(());
            }

            loop {
                match exit_rx.try_recv() {
                    Ok(_) => {
                        info!("process exit channel received signal");
                        break;
                    }
                    Err(err) => {
                        if err == TryRecvError::Disconnected {
                            info!("process exit channel disconnected");
                            break;
                        }
                    }
                };

                let video_frame = match video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        info!("video frame receiver closed");
                        break;
                    }
                };

                match decoder.decode(video_frame, &decoded_frame_tx) {
                    Ok(frames) => frames,
                    Err(err) => {
                        error!(?err, "video frame decode failed");
                        break;
                    }
                };
            }
        });

    Ok(())
}
