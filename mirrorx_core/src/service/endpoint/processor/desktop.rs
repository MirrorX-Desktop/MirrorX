use crate::{
    component::{
        desktop::{Duplicator, Frame},
        video_decoder::DecodedFrame,
    },
    error::MirrorXError,
    ffi::ffmpeg::avutil::av_gettime_relative,
    service::endpoint::ffi::create_callback_fn,
};
use crossbeam::channel::{Receiver, Sender, TryRecvError, TrySendError};
use scopeguard::defer;
use std::{os::raw::c_void, time::Duration};
use tracing::{error, info, trace};

#[cfg(target_os = "windows")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    capture_frame_tx: Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    let mut duplicator = Duplicator::new(display_id)?;

    let expected_wait_time = Duration::from_secs_f32(1f32 / (fps as f32));

    let _ = std::thread::Builder::new()
        .name(format!("desktop_capture_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                let _ = exit_tx.send(());
            }

            let epoch = unsafe { av_gettime_relative() };

            loop {
                let process_time_start = std::time::Instant::now();

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

                match duplicator.capture() {
                    Ok(mut frame) => unsafe {
                        frame.capture_time = av_gettime_relative() - epoch;

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
                            info!("desktop frame channel disconnected");
                            break;
                        }
                    },
                    Err(err) => {
                        error!(?err, "capture desktop frame failed");
                        break;
                    }
                };

                if let Some(actual_wait_time) =
                    expected_wait_time.checked_sub(process_time_start.elapsed())
                {
                    std::thread::sleep(actual_wait_time);
                }
            }

            info!(?remote_device_id, "desktop capture process exit");
        });

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn start_desktop_capture_process(
    remote_device_id: String,
    exit_tx: Sender<()>,
    exit_rx: Receiver<()>,
    capture_frame_tx: Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<(), MirrorXError> {
    let mut duplicator = Duplicator::new(capture_frame_tx, display_id, fps)?;

    let _ = std::thread::Builder::new()
        .name(format!("desktop_capture_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                let _ = exit_tx.send(());
                info!(?remote_device_id, "desktop capture process exit");
            }

            if let Err(err) = duplicator.start() {
                error!(?err, "duplicator start failed");
                return;
            }

            defer! {
                duplicator.stop();
            }

            let _ = exit_rx.recv();
        });

    Ok(())
}

pub fn start_desktop_render_process(
    remote_device_id: String,
    decoded_video_frame_rx: crossbeam::channel::Receiver<DecodedFrame>,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) {
    let update_callback_fn = unsafe { create_callback_fn(update_frame_callback_ptr) };

    let _ = std::thread::Builder::new()
        .name(format!("video_render_process:{}", remote_device_id))
        .spawn(move || {
            loop {
                let decoded_video_frame = match decoded_video_frame_rx.recv() {
                    Ok(frame) => frame,
                    Err(_) => {
                        info!(?remote_device_id, "video decoded channel is closed");
                        break;
                    }
                };

                #[cfg(target_os = "macos")]
                unsafe {
                    update_callback_fn(
                        texture_id,
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.0,
                    );
                }

                #[cfg(target_os = "windows")]
                unsafe {
                    update_callback_fn(
                        video_texture_ptr as *mut c_void,
                        decoded_video_frame.0.as_ptr(),
                        1920,
                        1080,
                    );
                }
            }

            info!(?remote_device_id, "video render process exit");
        });
}
