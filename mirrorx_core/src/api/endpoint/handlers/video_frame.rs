use crate::{
    api::endpoint::{message::EndPointVideoFrame, EndPointID},
    component::{frame::DesktopDecodeFrame, video_decoder::decoder::VideoDecoder},
};
use tokio::sync::mpsc::Sender;

pub fn serve_video_decode(
    id: EndPointID,
    render_tx: Sender<DesktopDecodeFrame>,
) -> Sender<EndPointVideoFrame> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(120);

    tokio::task::spawn_blocking(move || {
        tracing::info!(?id, "video decode process");

        let mut decoder = VideoDecoder::new(render_tx);

        while let Some(video_frame) = rx.blocking_recv() {
            // let instant = std::time::Instant::now();
            if let Err(err) = decoder.decode(video_frame) {
                tracing::error!(?err, "decode video frame failed");
                break;
            }
            // let elapsed = instant.elapsed();
            // tracing::info!(?elapsed, "instant");
        }

        tracing::info!("video decode process exit");
    });

    tx
}
