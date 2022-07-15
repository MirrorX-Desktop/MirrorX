use crate::{
    component::{
        desktop::Frame,
        video_decoder::{DecodedFrame, VideoDecoder},
        video_encoder::VideoEncoder,
    },
    error::MirrorXError,
    service::endpoint::message::*,
    utility::runtime::TOKIO_RUNTIME,
};
use crossbeam::channel::{internal::SelectHandle, Receiver, Sender, TryRecvError};
use scopeguard::defer;
use std::collections::HashMap;
use tokio::select;
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
    let (encoder_name, options) = if cfg!(target_os = "macos") {
        (
            "h264_videotoolbox",
            HashMap::from([("realtime", "1"), ("allow_sw", "0")]),
        )
    } else if cfg!(target_os = "windows") {
        (
            "libx264",
            HashMap::from([("preset", "ultrafast"), ("tune", "zerolatency")]),
        )
    } else {
        panic!("unsupported platform")
    };

    let mut encoder = VideoEncoder::new(encoder_name, fps, width, height, options)?;

    std::thread::Builder::new()
        .name(format!("video_encode_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                info!(?remote_device_id, "video encode process exit");
                let _ = exit_tx.send(());
            }

            loop {
                crossbeam::select! {
                    recv(exit_rx) -> _ => {
                        return;
                    },
                    recv(capture_frame_rx) -> res => match res {
                        Ok(frame) => if let Err(err) = encoder.encode(frame, &packet_tx) {
                            error!(?err, "video frame encode failed");
                            return;
                        },
                        Err(_) => return,
                    }
                }
            }
        })
        .and_then(|_| Ok(()))
        .map_err(|err| {
            MirrorXError::Other(anyhow::anyhow!("spawn video encode process failed ({err})"))
        })
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

    std::thread::Builder::new()
        .name(format!("video_decode_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                info!(?remote_device_id, "video decode process exit");
                let _ = exit_tx.send(());
            }

            loop {
                crossbeam::select! {
                    recv(exit_rx) -> _ => {
                        return;
                    },
                    recv(video_frame_rx) -> res => match res {
                        Ok(frame) => if let Err(err) = decoder.decode(frame, &decoded_frame_tx) {
                            error!(?err, "video frame decode failed");
                            return;
                        },
                        Err(_) => return,
                    }
                }
            }
        })
        .and_then(|_| Ok(()))
        .map_err(|err| {
            MirrorXError::Other(anyhow::anyhow!("spawn video decode process failed ({err})"))
        })
}
