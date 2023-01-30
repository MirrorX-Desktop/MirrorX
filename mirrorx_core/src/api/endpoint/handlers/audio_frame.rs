use crate::{
    api::endpoint::{message::EndPointAudioFrame, EndPointID},
    component::audio::{
        decoder::AudioDecoder,
        player::{default_output_config, new_play_stream_and_tx},
    },
};
use cpal::traits::StreamTrait;
use tokio::sync::mpsc::Receiver;

pub fn serve_audio_decode(id: EndPointID, mut decode_rx: Receiver<EndPointAudioFrame>) {
    tokio::task::spawn_blocking(move || loop {
        tracing::info!(?id, "audio decode process");

        let Ok(config) = default_output_config() else {
            tracing::error!("get default audio output config failed");
            return;
        };

        tracing::info!(?config, "default output config");

        let mut audio_decoder = AudioDecoder::new(
            config.channels() as _,
            config.sample_format(),
            config.sample_rate(),
        );

        let mut stream = None;
        let mut samples_tx = None;

        loop {
            match decode_rx.blocking_recv() {
                Some(audio_frame) => {
                    match audio_decoder.decode(audio_frame) {
                        Ok(buffer) => {
                            // because active endpoint always output 48000hz and 480 samples per channel after
                            // opus encode, so here we simply div (48000/480)=100 to get samples count after
                            // resample.
                            let valid_min_samples_per_channel = config.sample_rate().0 / 100;

                            if stream.is_none() {
                                let buffer_size = buffer.len()
                                    / (config.channels() as usize)
                                    / config.sample_format().sample_size();

                                // drop the beginning frames
                                if buffer_size < (valid_min_samples_per_channel as usize) {
                                    continue;
                                }

                                tracing::info!(?buffer_size, "use buffer size");

                                match new_play_stream_and_tx(
                                    config.channels(),
                                    config.sample_format(),
                                    config.sample_rate(),
                                    buffer_size as u32,
                                ) {
                                    Ok((play_stream, audio_sample_tx)) => {
                                        if let Err(err) = play_stream.play() {
                                            tracing::error!(?err, "play audio stream failed");
                                            return;
                                        }

                                        stream = Some(play_stream);
                                        samples_tx = Some(audio_sample_tx);
                                    }
                                    Err(err) => {
                                        tracing::error!(
                                            ?err,
                                            "initialize audio play stream failed"
                                        );
                                        continue;
                                    }
                                };
                            }

                            if let Some(ref samples_tx) = samples_tx {
                                if samples_tx.blocking_send(buffer).is_err() {
                                    tracing::error!("send audio play buffer failed");
                                    return;
                                }
                            }
                        }
                        Err(err) => {
                            tracing::error!(?err, "decode audio frame failed");
                            break;
                        }
                    };
                }
                None => {
                    if let Some(ref stream) = stream {
                        let _ = stream.pause();
                    }

                    tracing::error!("audio decode process exit");
                    return;
                }
            }
        }

        if let Some(ref stream) = stream {
            let _ = stream.pause();
        }
    });
}
