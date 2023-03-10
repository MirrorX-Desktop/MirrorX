use crate::{
    component::{frame::DesktopDecodeFrame, video_decoder::decoder::VideoDecoder},
    service::endpoint::{message::EndPointVideoFrame, EndPointID},
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
            if let Err(err) = decoder.decode(video_frame) {
                tracing::error!(?err, "decode video frame failed");
                break;
            }
        }

        tracing::info!("video decode process exit");
    });

    tx
}
