use crate::{
    component::{desktop::Duplicator, video_decoder::DecodedFrame},
    error::CoreError,
    service::endpoint::ffi::create_callback_fn,
};
use crate::{core_error, utility::runtime::TOKIO_RUNTIME};
use crossbeam::channel::{Receiver, Sender};
use scopeguard::defer;
use std::{os::raw::c_void, time::Duration};

#[cfg(target_os = "windows")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    capture_frame_tx: crossbeam::channel::Sender<crate::component::capture_frame::CaptureFrame>,
    display_id: Option<String>,
    fps: u8,
) -> Result<(), CoreError> {
    use crate::component::capture_frame::CaptureFrame;
    use std::ops::Sub;
    use tokio::sync::mpsc::error::TrySendError;

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?remote_device_id, "desktop capture process exit");
            let _ = exit_tx.try_broadcast(());
        }

        let mut duplicator = match Duplicator::new(display_id) {
            Ok(duplicator) => duplicator,
            Err(err) => {
                tracing::error!(?err, "create Duplicator failed");
                return;
            }
        };

        let expected_wait_time = Duration::from_secs_f32(1f32 / (fps as f32));

        loop {
            let begin = std::time::Instant::now();

            match exit_rx.try_recv() {
                Ok(_) => break,
                Err(err) => {
                    if err.is_closed() || err.is_overflowed() {
                        break;
                    }
                }
            };

            let frame = match duplicator.capture() {
                Ok(frame) => frame,
                Err(err) => {
                    tracing::error!(?err, "duplicator capture frame failed");
                    break;
                }
            };

            if let Err(err) = capture_frame_tx.try_send(frame) {
                if err.is_full() {
                    tracing::warn!("duplicator capture frame tx is full");
                } else if err.is_disconnected() {
                    break;
                }
            }

            if let Some(wait_duration) = expected_wait_time.checked_sub(begin.elapsed()) {
                std::thread::sleep(wait_duration);
            }
        }
    });

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    capture_frame_tx: crossbeam::channel::Sender<CaptureFrame>,
    display_id: &str,
    fps: u8,
) -> Result<(), CoreError> {
    let display_id = match display_id.parse::<u32>() {
        Ok(display_id) => display_id,
        Err(err) => return Err(CoreError::Other(anyhow::anyhow!(err))),
    };

    let mut duplicator = Duplicator::new(display_id, capture_frame_tx)?;

    // std::thread::Builder::new()
    //     .name(format!("desktop_capture_process:{}", remote_device_id))
    //     .spawn(move || {
    //         defer! {
    //             info!(?remote_device_id, "desktop capture process exit");
    //             let _ = exit_tx.try_broadcast(());
    //         }

    //         if let Err(err) = duplicator.start() {
    //             error!(?err, "duplicator start failed");
    //             return;
    //         }

    //         let _ = exit_rx.recv().await;
    //         tracing::info!("recv exit");

    //         duplicator.stop();
    //     })
    //     .and_then(|_| Ok(()))
    //     .map_err(|err| {
    //         MirrorXError::Other(anyhow::anyhow!(
    //             "spawn desktop capture process failed ({err})"
    //         ))
    //     });

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
) -> Result<(), CoreError> {
    let update_callback_fn = unsafe { create_callback_fn(update_frame_callback_ptr) };

    std::thread::Builder::new()
        .name(format!("desktop_render_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let decoded_video_frame = match decoded_video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        tracing::info!(?remote_device_id, "video decoded channel is closed");
                        break;
                    }
                };

                #[cfg(target_os = "macos")]
                unsafe {
                    update_callback_fn(video_texture_ptr as *mut c_void, decoded_video_frame.0);
                }

                #[cfg(target_os = "windows")]
                unsafe {
                    update_callback_fn(
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.buffer.as_ptr(),
                        decoded_video_frame.width as usize,
                        decoded_video_frame.height as usize,
                    );
                }
            }

            tracing::info!(?remote_device_id, "video render process exit");
        })
        .and_then(|_| Ok(()))
        .map_err(|err| core_error!("spawn desktop render process failed ({})", err))
}
