use crate::{
    api::endpoint::{message::EndPointAudioFrame, EndPointID},
    component::audio::player::AudioPlayer,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::{error::TrySendError, Sender};

static DECODERS: Lazy<DashMap<EndPointID, Sender<EndPointAudioFrame>>> = Lazy::new(DashMap::new);

pub fn handle_audio_frame(id: EndPointID, audio_frame: EndPointAudioFrame) {
    if let Some(tx) = DECODERS.get(&id) {
        let _entered = tracing::info_span!("handle_audio_frame", id = ?id).entered();
        if let Err(err) = tx.try_send(audio_frame) {
            match err {
                TrySendError::Full(_) => tracing::warn!("audio frame decode tx is full!"),
                TrySendError::Closed(_) => {
                    tracing::info!("audio frame decode tx has closed");
                }
            }
        }
    }
}

pub fn serve_audio_decode(id: EndPointID) {
    if !DECODERS.contains_key(&id) {
        let (audio_frame_tx, mut audio_frame_rx) = tokio::sync::mpsc::channel(64);
        DECODERS.insert(id, audio_frame_tx);

        tokio::spawn(async move {
            let span = tracing::info_span!("audio decode process", id = ?id);

            let mut audio_decoder = AudioPlayer::new();

            loop {
                let audio_frame = audio_frame_rx.recv().await;

                let _entered = span.clone().entered();

                match audio_frame {
                    Some(audio_frame) => {
                        if let Err(err) = audio_decoder.play_samples(audio_frame) {
                            tracing::error!(?err, "audio decoder decode failed");
                            return;
                        }
                    }
                    None => {
                        DECODERS.remove(&id);
                        tracing::info!("audio decode process exit");
                        return;
                    }
                }
            }
        });
    }
}
