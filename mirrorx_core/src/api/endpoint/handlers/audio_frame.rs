use crate::{
    api::endpoint::message::EndPointAudioFrame, component::audio::player::AudioPlayer,
    utility::runtime::TOKIO_RUNTIME,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, OutputCallbackInfo, StreamConfig,
};
use dashmap::DashMap;
use once_cell::{sync::Lazy, unsync::OnceCell};
use rtrb::{Consumer, Producer};
use scopeguard::defer;
use std::time::Duration;
use tokio::sync::mpsc::{error::TrySendError, Sender};

static DECODERS: Lazy<DashMap<(i64, i64), Sender<EndPointAudioFrame>>> = Lazy::new(DashMap::new);

pub fn handle_audio_frame(
    active_device_id: i64,
    passive_device_id: i64,
    audio_frame: EndPointAudioFrame,
) {
    if let Some(tx) = DECODERS.get(&(active_device_id, passive_device_id)) {
        if let Err(err) = tx.try_send(audio_frame) {
            match err {
                TrySendError::Full(_) => tracing::warn!(
                    ?active_device_id,
                    ?passive_device_id,
                    "audio frame decode tx is full!"
                ),
                TrySendError::Closed(_) => {
                    tracing::info!(
                        ?active_device_id,
                        ?passive_device_id,
                        "audio frame decode tx has closed"
                    );
                }
            }
        }
    }
}

pub fn serve_audio_decode(active_device_id: i64, passive_device_id: i64) {
    if !DECODERS.contains_key(&(active_device_id, passive_device_id)) {
        let (audio_frame_tx, mut audio_frame_rx) = tokio::sync::mpsc::channel(64);
        DECODERS.insert((active_device_id, passive_device_id), audio_frame_tx);

        TOKIO_RUNTIME.spawn(async move {
            let mut audio_decoder = AudioPlayer::new();

            loop {
                match audio_frame_rx.recv().await {
                    Some(audio_frame) => {
                        if let Err(err) = audio_decoder.play_samples(audio_frame) {
                            tracing::error!(?err, "audio decoder decode failed");
                            return;
                        }
                    }
                    None => {
                        return;
                    }
                }
            }
        });
    }
}
