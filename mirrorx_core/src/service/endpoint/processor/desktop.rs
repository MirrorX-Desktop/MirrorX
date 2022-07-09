use crate::{
    component::{
        desktop::{Duplicator, Frame},
        video_decoder::DecodedFrame,
    },
    error::MirrorXError,
    service::endpoint::ffi::create_callback_fn,
};
use crossbeam::channel::Sender;
use scopeguard::defer;
use std::{os::raw::c_void, time::Duration};
use tracing::{error, info};

pub fn start_desktop_capture_process(
    remote_device_id: String,
    capture_frame_tx: Sender<Frame>,
    display_id: &str,
    fps: u8,
) -> Result<Sender<()>, MirrorXError> {
    let mut desktop_duplicator = Duplicator::new(capture_frame_tx, display_id, fps)?;
    let (exit_tx, exit_rx) = crossbeam::channel::bounded(1);

    if let Err(err) = desktop_duplicator.start() {
        return Err(MirrorXError::Other(anyhow::anyhow!(
            "start desktop duplicator failed ({})",
            err
        )));
    }

    let _ = std::thread::Builder::new()
        .name(format!("audio_play_process:{}", remote_device_id))
        .spawn(move || {
            defer! {
                desktop_duplicator.stop();
                info!("desktop capture process exit");
            }

            let _ = exit_rx.recv();
        });

    Ok(exit_tx)
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
