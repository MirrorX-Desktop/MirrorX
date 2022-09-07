use crate::{
    component::video_decoder::DecodedFrame,
    error::{CoreError, CoreResult},
    service::endpoint::message::*,
    utility::runtime::TOKIO_RUNTIME,
};
use async_broadcast::TryRecvError;
use crossbeam::channel::{Receiver, Sender};
use scopeguard::defer;

pub fn start_video_encode_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    width: i32,
    height: i32,
    fps: i32,
    mut capture_frame_rx: crossbeam::channel::Receiver<
        crate::component::capture_frame::CaptureFrame,
    >,
    mut packet_tx: tokio::sync::mpsc::Sender<EndPointMessagePacket>,
) -> CoreResult<()> {
    #[cfg(target_os = "macos")]
    let mut encoder = crate::component::video_encoder::Encoder::new(width, height)?;
    #[cfg(not(target_os = "macos"))]
    let mut encoder = crate::component::video_encoder::Encoder::new(
        FFMPEGEncoderType::Libx264,
        width,
        height,
        fps,
    )?;

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?remote_device_id, "video encode process exit");
            let _ = exit_tx.try_broadcast(());
        }

        loop {
            match exit_rx.try_recv() {
                Ok(_) => return,
                Err(err) => {
                    if err == TryRecvError::Closed {
                        return;
                    }
                }
            };

            match capture_frame_rx.recv() {
                Ok(capture_frame) => {
                    if let Err(err) = encoder.encode(capture_frame, &mut packet_tx) {
                        tracing::error!(?err, "video frame encode failed");
                        return;
                    }
                }
                Err(_) => return,
            };
        }
    });

    Ok(())
}

pub fn start_video_decode_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    width: i32,
    height: i32,
    fps: i32,
    mut video_frame_rx: Receiver<VideoFrame>,
    decoded_frame_tx: Sender<DecodedFrame>,
) -> CoreResult<()> {
    // let (decoder_name, options) = if cfg!(target_os = "macos") {
    //     ("h264", HashMap::new())
    // } else if cfg!(target_os = "windows") {
    //     (
    //         "h264_d3d11va",
    //         HashMap::new(), // HashMap::from([("async_depth", "1"), ("gpu_copy", "on")]),
    //     )
    // } else {
    //     return Err(MirrorXError::Other(anyhow::anyhow!(
    //         "unsupport platform decode"
    //     )));
    // };

    // let mut decoder = crate::component::video_decoder::videotoolbox::Decoder::new();

    // TOKIO_RUNTIME.spawn_blocking(move || {
    //     let tx_ptr = Box::into_raw(Box::new(decoded_frame_tx));

    //     defer! {
    //             info!(?remote_device_id, "video decode process exit");
    //             let _ = unsafe { Box::from_raw(tx_ptr) };
    //             let _ = exit_tx.try_broadcast(());
    //     }

    //     loop {
    //         match exit_rx.try_recv() {
    //             Ok(_) => break,
    //             Err(err) => {
    //                 if err.is_closed() || err.is_overflowed() {
    //                     break;
    //                 }
    //             }
    //         };

    //         match video_frame_rx.recv_timeout(Duration::from_secs(1)) {
    //             Ok(video_frame) => {
    //                 if let Err(err) = decoder.decode(video_frame, tx_ptr) {
    //                     error!(?err, "video frame decode failed");
    //                     break;
    //                 }
    //             }
    //             Err(err) => {
    //                 if err.is_disconnected() {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // });

    Ok(())
}
