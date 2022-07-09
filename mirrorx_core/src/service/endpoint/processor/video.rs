use crate::{
    component::{
        desktop::Frame,
        video_decoder::{DecodedFrame, VideoDecoder},
        video_encoder::VideoEncoder,
    },
    error::MirrorXError,
    service::endpoint::message::*,
};
use crossbeam::channel::{Receiver, Sender};
use std::collections::HashMap;
use tracing::{error, info, warn};

pub fn start_video_encode_process(
    remote_device_id: String,
    width: i32,
    height: i32,
    fps: i32,
    capture_frame_rx: crossbeam::channel::Receiver<Frame<'static>>,
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
        encoder.set_opt("sc_threshold", "0", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let _ = std::thread::Builder::new()
        .name(format!("video_encode_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let frame = match capture_frame_rx.recv() {
                    Ok(v) => v,
                    Err(_) => {
                        info!("capture channel closed");
                        break;
                    }
                };

                let frames = match encoder.encode(frame) {
                    Ok(frames) => frames,
                    Err(err) => {
                        error!(?err, "video frame decode failed");
                        break;
                    }
                };

                for frame in frames {
                    let packet = EndPointMessagePacket {
                        typ: EndPointMessagePacketType::Push,
                        call_id: None,
                        message: EndPointMessage::VideoFrame(VideoFrame {
                            buffer: frame,
                            timestamp: 0,
                        }),
                    };

                    if let Err(err) = packet_tx.try_send(packet) {
                        match err {
                            tokio::sync::mpsc::error::TrySendError::Full(_) => {
                                warn!("network send channel is full")
                            }
                            tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                                error!("network send channel is closed");
                                break;
                            }
                        }
                    }
                }
            }

            info!(?remote_device_id, "video encode process exit");
        });

    Ok(())
}

pub fn start_video_decode_process(
    remote_device_id: String,
    width: i32,
    height: i32,
    video_frame_rx: Receiver<VideoFrame>,
    decoded_frame_tx: Sender<DecodedFrame>,
) -> Result<(), MirrorXError> {
    let (decoder_name, options) = if cfg!(target_os = "macos") {
        ("h264", HashMap::new())
    } else if cfg!(target_os = "windows") {
        (
            "h264_qsv",
            HashMap::from([("async_depth", "1"), ("gpu_copy", "on")]),
        )
    } else {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "unsupport platform decode"
        )));
    };

    let decoder = VideoDecoder::new(decoder_name, width, height, options)?;

    let _ = std::thread::Builder::new()
        .name(format!("video_decode_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let video_frame = match video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        info!("video frame receiver closed");
                        break;
                    }
                };

                let frames = match decoder.decode(video_frame.buffer, 0, 0) {
                    Ok(frames) => frames,
                    Err(err) => {
                        error!(?err, "video frame decode failed");
                        break;
                    }
                };

                for frame in frames {
                    if let Err(err) = decoded_frame_tx.try_send(frame) {
                        match err {
                            crossbeam::channel::TrySendError::Full(_) => {
                                warn!("video decoded frame channel is full")
                            }
                            crossbeam::channel::TrySendError::Disconnected(_) => {
                                info!("video decoded frame channel closed");
                                break;
                            }
                        }
                    }
                }
            }

            info!(?remote_device_id, "video decode process exit");
        });

    Ok(())
}
