use crate::utility::runtime::TOKIO_RUNTIME;
use crate::{
    component::{
        desktop::{Duplicator, Frame},
        video_decoder::DecodedFrame,
    },
    error::MirrorXError,
    service::endpoint::ffi::create_callback_fn,
};
use crossbeam::channel::{Receiver, Sender};
use scopeguard::defer;
use std::{os::raw::c_void, time::Duration};
use tracing::{error, info, trace, warn};

#[cfg(target_os = "windows")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    capture_frame_tx: crossbeam::channel::Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    use tokio::sync::mpsc::error::TrySendError;

    let mut duplicator = Duplicator::new(display_id)?;

    let expected_wait_time = Duration::from_secs_f32(1f32 / (fps as f32));

    TOKIO_RUNTIME.spawn(async move {
        defer! {
            info!(?remote_device_id, "desktop capture process exit");
            let _ = exit_tx.try_broadcast(());
        }

        let mut interval = tokio::time::interval(expected_wait_time);
        let epoch = unsafe { crate::ffi::ffmpeg::avutil::av_gettime_relative() };

        loop {
            tokio::select! {
                _ = exit_rx.recv() => break,
                _ = interval.tick() => match duplicator.capture() {
                    Ok(mut frame) => unsafe {
                        frame.capture_time = crate::ffi::ffmpeg::avutil::av_gettime_relative() - epoch;

                        trace!(
                            width=?frame.width,
                            height=?frame.height,
                            chrominance_len=?frame.chrominance_buffer.len(),
                            chrominance_stride=?frame.chrominance_stride,
                            luminance_len=?frame.luminance_buffer.len(),
                            luminance_stride=?frame.luminance_stride,
                            capture_time=?frame.capture_time,
                            "desktop capture frame",
                        );

                        if let Err(_) = capture_frame_tx.send(frame) {
                            // match err{
                            //     TrySendError::Full(_) => warn!("desktop frame if full"),
                            //     TrySendError::Closed(_) => return,
                            // };
                            return;
                        }
                    },
                    Err(err) => {
                        error!(?err, "capture desktop frame failed");
                        return;
                    }
                }
            }

            // select! {
            //     recv(exit_rx) -> _ => {
            //         return;
            //     },
            //     recv(interval) -> _ =>  match duplicator.capture() {
            //         Ok(mut frame) => unsafe {
            //             frame.capture_time = av_gettime_relative() - epoch;

            //             trace!(
            //                 width=?frame.width,
            //                 height=?frame.height,
            //                 chrominance_len=?frame.chrominance_buffer.len(),
            //                 chrominance_stride=?frame.chrominance_stride,
            //                 luminance_len=?frame.luminance_buffer.len(),
            //                 luminance_stride=?frame.luminance_stride,
            //                 capture_time=?frame.capture_time,
            //                 "desktop capture frame",
            //             );

            //             if let Err(_) = capture_frame_tx.send(frame) {
            //                 return;
            //             }
            //         },
            //         Err(err) => {
            //             error!(?err, "capture desktop frame failed");
            //             return;
            //         }
            //     }
            // }
        }
    });

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    capture_frame_tx: tokio::sync::mpsc::Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    let mut duplicator = Duplicator::new(capture_frame_tx, display_id, fps)?;

    // std::thread::Builder::new()
    //     .name(format!("desktop_capture_process:{}", remote_device_id))
    //     .spawn(move || {
    //         defer! {
    //             info!(?remote_device_id, "desktop capture process exit");
    //             let _ = exit_tx.send(());
    //         }

    //         if let Err(err) = duplicator.start() {
    //             error!(?err, "duplicator start failed");
    //             return;
    //         }

    //         let _ = exit_rx.recv();

    //         duplicator.stop();
    //     })
    //     .and_then(|_| Ok(()))
    //     .map_err(|err| {
    //         MirrorXError::Other(anyhow::anyhow!(
    //             "spawn desktop capture process failed ({err})"
    //         ))
    //     })

    TOKIO_RUNTIME.spawn(async move {
        defer! {
            info!(?remote_device_id, "desktop capture process exit");
            let _ = exit_tx.try_broadcast(());
        }

        if let Err(err) = duplicator.start() {
            error!(?err, "duplicator start failed");
            return;
        }

        let _ = exit_rx.recv().await;

        duplicator.stop();
    });

    Ok(())
}

pub fn start_desktop_render_process(
    remote_device_id: String,
    decoded_video_frame_rx: crossbeam::channel::Receiver<DecodedFrame>,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> Result<(), MirrorXError> {
    let update_callback_fn = unsafe { create_callback_fn(update_frame_callback_ptr) };

    std::thread::Builder::new()
        .name(format!("desktop_render_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let decoded_video_frame = match decoded_video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        info!(?remote_device_id, "video decoded channel is closed");
                        break;
                    }
                };

                // #[cfg(target_os = "macos")]
                // unsafe {
                //     update_callback_fn(
                //         texture_id,
                //         video_texture_ptr as *mut c_void,
                //         decoded_video_frame.0,
                //     );
                // }

                // #[cfg(target_os = "windows")]
                unsafe {
                    update_callback_fn(
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.buffer.as_ptr(),
                        decoded_video_frame.width as usize,
                        decoded_video_frame.height as usize,
                    );
                }
            }

            info!(?remote_device_id, "video render process exit");
        })
        .and_then(|_| Ok(()))
        .map_err(|err| {
            MirrorXError::Other(anyhow::anyhow!(
                "spawn desktop render process failed ({err})"
            ))
        })
}
