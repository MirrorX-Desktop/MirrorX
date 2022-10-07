use std::time::{Duration, Instant};

use crate::{
    api::endpoint::{flutter_message::FlutterMediaMessage, message::EndPointVideoFrame},
    component::video_decoder::video_decoder::VideoDecoder,
    utility::runtime::TOKIO_RUNTIME,
};
use dashmap::DashMap;
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use once_cell::sync::Lazy;
use scopeguard::defer;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;

static DECODERS: Lazy<DashMap<(i64, i64), Sender<EndPointVideoFrame>>> = Lazy::new(DashMap::new);

pub fn handle_video_frame(
    active_device_id: i64,
    passive_device_id: i64,
    video_frame: EndPointVideoFrame,
) {
    if let Some(tx) = DECODERS.get(&(active_device_id, passive_device_id)) {
        if let Err(err) = tx.try_send(video_frame) {
            match err {
                TrySendError::Full(_) => tracing::warn!(
                    ?active_device_id,
                    ?passive_device_id,
                    "video frame decode tx is full!"
                ),
                TrySendError::Closed(_) => {
                    tracing::info!(
                        ?active_device_id,
                        ?passive_device_id,
                        "video frame decode tx has closed"
                    );
                }
            };
        }
    }
}

pub fn serve_video_decode(
    active_device_id: i64,
    passive_device_id: i64,
    texture_id: i64,
    stream: StreamSink<FlutterMediaMessage>,
) {
    if !DECODERS.contains_key(&(active_device_id, passive_device_id)) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(180);
        DECODERS.insert((active_device_id, passive_device_id), tx);

        let (render_frame_tx, mut render_frame_rx) =
            tokio::sync::mpsc::channel::<(Duration, Instant, Vec<u8>)>(180);

        TOKIO_RUNTIME.spawn(async move {
            loop {
                let (frame_duration, begin, frame_buffer) = match render_frame_rx.recv().await {
                    Some(data) => data,
                    None => return,
                };

                let render_begin_instant = std::time::Instant::now();
                if !stream.add(FlutterMediaMessage::Video(ZeroCopyBuffer(frame_buffer))) {
                    tracing::error!("post frame_buffer to flutter side failed");
                    return;
                }
                let render_cost_time = render_begin_instant.elapsed();

                // if let Some(remaining_wait_time) = frame_duration.checked_sub(render_cost_time) {
                //     if let Some(extra_wait) = remaining_wait_time.checked_sub(begin.elapsed()) {
                //         tracing::trace!("extra wait: {:?}", extra_wait);
                //         tokio::time::sleep(extra_wait).await;
                //     }
                // }
            }
        });

        TOKIO_RUNTIME.spawn(async move {
            defer! {
                tracing::info!(?active_device_id, ?passive_device_id, "decode video frame process exit");
                DECODERS.remove(&(active_device_id, passive_device_id));
            }

            let mut decoder = VideoDecoder::new(texture_id, render_frame_tx);

            while let Some(video_frame) = rx.recv().await {
                if let Err(err) = decoder.decode(video_frame).await {
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
