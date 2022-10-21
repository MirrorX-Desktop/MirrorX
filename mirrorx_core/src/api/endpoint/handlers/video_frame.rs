use crate::{
    api::endpoint::{message::EndPointVideoFrame, EndPointID},
    component::video_decoder::video_decoder::VideoDecoder,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::time::{Duration, Instant};
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

pub fn serve_video_decode(id: EndPointID, texture_id: i64) {
    if !DECODERS.contains_key(&id) {
        let (tx, mut rx) = tokio::sync::mpsc::channel(180);
        DECODERS.insert(id, tx);

        let (render_frame_tx, mut render_frame_rx) =
            tokio::sync::mpsc::channel::<(Duration, Instant, Vec<u8>)>(180);

        // todo: pass frame buffer to window
        tokio::spawn(async move {
            // loop {
            //     let (frame_duration, begin, frame_buffer) = match render_frame_rx.recv().await {
            //         Some(data) => data,
            //         None => return,
            //     };

            //     let render_begin_instant = std::time::Instant::now();
            //     // if !stream.add(FlutterMediaMessage::Video(ZeroCopyBuffer(frame_buffer))) {
            //     //     tracing::error!("post frame_buffer to flutter side failed");
            //     //     return;
            //     // }
            //     let render_cost_time = render_begin_instant.elapsed();

            //     // if let Some(remaining_wait_time) = frame_duration.checked_sub(render_cost_time) {
            //     //     if let Some(extra_wait) = remaining_wait_time.checked_sub(begin.elapsed()) {
            //     //         tracing::trace!("extra wait: {:?}", extra_wait);
            //     //         tokio::time::sleep(extra_wait).await;
            //     //     }
            //     // }
            // }
        });

        tokio::spawn(async move {
            let span = tracing::info_span!("video decode process", id = ?id);
            let _enter = span.enter();

            let mut decoder = VideoDecoder::new(texture_id, render_frame_tx);

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
