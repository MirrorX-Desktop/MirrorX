use crate::{
    api::endpoint::{flutter_message::FlutterMediaMessage, message::EndPointVideoFrame},
    component::video_decoder::video_decoder::VideoDecoder,
    utility::runtime::TOKIO_RUNTIME,
};
use dashmap::DashMap;
use flutter_rust_bridge::StreamSink;
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

        TOKIO_RUNTIME.spawn(async move {
            defer! {
                tracing::info!(?active_device_id, ?passive_device_id, "decode video frame process exit");
                DECODERS.remove(&(active_device_id, passive_device_id));
            }

            let mut decoder = VideoDecoder::new(texture_id, stream);

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
