use crate::{
    api::endpoint::{message::EndPointVideoFrame, EndPointID},
    component::{frame::DesktopDecodeFrame, video_decoder::video_decoder::VideoDecoder},
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::{error::TrySendError, Sender};
use tracing::Instrument;

static DECODERS: Lazy<DashMap<EndPointID, Sender<EndPointVideoFrame>>> = Lazy::new(DashMap::new);

pub fn handle_video_frame(id: EndPointID, video_frame: EndPointVideoFrame) {
    if let Some(tx) = DECODERS.get(&id) {
        tracing::info_span!("handle_video_frame", id = ?id).in_scope(|| {
            if let Err(err) = tx.try_send(video_frame) {
                match err {
                    TrySendError::Full(_) => tracing::warn!("video frame decode tx is full!"),
                    TrySendError::Closed(_) => {
                        tracing::info!("video frame decode tx has closed");
                    }
                };
            }
        });
    }
}

pub fn serve_video_decode(id: EndPointID, frame_tx: Sender<DesktopDecodeFrame>) {
    if !DECODERS.contains_key(&id) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(180);
        DECODERS.insert(id, tx);

        tokio::spawn(async move {
            let span = tracing::info_span!("video decode process", id = ?id);
            let _enter = span.enter();

            let mut decoder = VideoDecoder::new(frame_tx);

            async {
                while let Some(video_frame) = rx.recv().await {
                    if let Err(err) = decoder.decode(video_frame).await {
                        tracing::error!(?err, "decode video frame failed");
                        break;
                    }
                }
            }
            .instrument(span.clone())
            .await;

            let _ = DECODERS.remove(&id);
            tracing::info!("video decode process exit");
        });
    }
}
