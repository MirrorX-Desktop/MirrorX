use crate::{
    api::endpoint::{message::EndPointAudioFrame, EndPointID},
    component::{
        audio::player::new_play_stream_and_tx, audio_decoder::audio_decoder::AudioDecoder,
    },
};
use cpal::traits::StreamTrait;
use tokio::sync::mpsc::Receiver;

pub fn serve_audio_decode(id: EndPointID, mut decode_rx: Receiver<EndPointAudioFrame>) {
    tokio::task::spawn_blocking(move || loop {
        tracing::info!(?id, "audio decode process");

        let (stream, channels, sample_format, sample_rate, samples_tx) =
            match new_play_stream_and_tx() {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!(?err, "initialize audio play stream failed");
                    continue;
                }
            };

        let mut audio_decoder = AudioDecoder::new(channels, sample_format, sample_rate, samples_tx);

        if let Err(err) = stream.play() {
            tracing::error!(?err, "play audio stream failed");
            continue;
        }

        loop {
            match decode_rx.blocking_recv() {
                Some(audio_frame) => {
                    if let Err(err) = audio_decoder.decode(audio_frame) {
                        tracing::error!(?err, "decode audio frame failed");
                        break;
                    };
                }
                None => {
                    let _ = stream.pause();
                    tracing::error!("audio decode process exit");
                    return;
                }
            }
        }

        let _ = stream.pause();
    });
}
