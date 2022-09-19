use super::handshake::EndPointMediaMessage;
use crate::{
    api::endpoint::message::EndPointVideoFrame, component::video_decoder::Decoder,
    utility::runtime::TOKIO_RUNTIME,
};
use dashmap::DashMap;
use flutter_rust_bridge::StreamSink;
use once_cell::sync::Lazy;
use scopeguard::defer;
use tokio::sync::mpsc::Sender;

static DECODERS: Lazy<DashMap<(i64, i64), Sender<EndPointVideoFrame>>> = Lazy::new(DashMap::new);

pub async fn handle_video_frame(
    active_device_id: i64,
    passive_device_id: i64,
    video_frame: EndPointVideoFrame,
) {
    if let Some(tx) = DECODERS.get(&(active_device_id, passive_device_id)) {
        if tx.send(video_frame).await.is_err() {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                "send video frame failed"
            );
        }
    }
}

pub fn serve_decoder(
    active_device_id: i64,
    passive_device_id: i64,
    stream: StreamSink<EndPointMediaMessage>,
) {
    if !DECODERS.contains_key(&(active_device_id, passive_device_id)) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(180);
        DECODERS.insert((active_device_id, passive_device_id), tx);

        TOKIO_RUNTIME.spawn_blocking(move || {
            defer! {
                tracing::info!(?active_device_id, ?passive_device_id, "decode video frame process exit");
                DECODERS.remove(&(active_device_id, passive_device_id));
            }

            let mut decoder = Decoder::new(stream);

            while let Some(video_frame) = rx.blocking_recv() {
                if let Err(err) = decoder.decode(video_frame) {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "decode video frame failed"
                    );
                    break;
                }
            }
        });
    }
}

// use std::os::raw::c_void;

// #[cfg(target_os = "macos")]
// use crate::ffi::os::macos::core_video::CVPixelBufferRef;

// use super::handshake::EndPointMediaMessage;

// #[cfg(target_os = "macos")]
// pub const unsafe fn create_callback_fn(
//     callback_ptr: i64,
// ) -> unsafe extern "C" fn(*mut c_void, CVPixelBufferRef) {
//     std::mem::transmute::<
//         *const c_void,
//         unsafe extern "C" fn(video_texture_ptr: *mut c_void, pixel_buffer: CVPixelBufferRef),
//     >(callback_ptr as *const c_void)
// }

// #[cfg(target_os = "windows")]
// pub const unsafe fn create_callback_fn(
//     callback_ptr: i64,
// ) -> unsafe extern "C" fn(*mut c_void, *const u8, usize, usize) {
//     std::mem::transmute::<
//         *const c_void,
//         unsafe extern "C" fn(
//             video_texture_ptr: *mut c_void,
//             frame_buffer: *const u8,
//             frame_width: usize,
//             frame_height: usize,
//         ),
//     >(callback_ptr as *const c_void)
// }
