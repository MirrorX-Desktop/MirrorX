use crate::{
    api::endpoint::{message::EndPointVideoFrame, EndPointID},
    component::{frame::DesktopDecodeFrame, video_decoder::video_decoder::VideoDecoder},
};

pub fn serve_video_decode(
    id: EndPointID,
    render_tx: crossbeam::channel::Sender<DesktopDecodeFrame>,
) -> crossbeam::channel::Sender<EndPointVideoFrame> {
    let (tx, rx) = crossbeam::channel::bounded(120);

    tokio::task::spawn_blocking(move || {
        tracing::info!(?id, "video decode process");

        let mut decoder = VideoDecoder::new(render_tx);

        while let Ok(video_frame) = rx.recv() {
            let instant = std::time::Instant::now();
            if let Err(err) = decoder.decode(video_frame) {
                tracing::error!(?err, "decode video frame failed");
                break;
            }
            let elapsed = instant.elapsed();
            tracing::info!(?elapsed, "instant");
        }

        tracing::info!("video decode process exit");
    });

    tx
}
