use crate::{
    api::endpoint::message::EndPointAudioFrame, component::audio::decoder::AudioPlayer,
    utility::runtime::TOKIO_RUNTIME,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, OutputCallbackInfo, StreamConfig,
};
use crossbeam::channel::{bounded, Sender};
use dashmap::DashMap;
use once_cell::{sync::Lazy, unsync::OnceCell};
use rtrb::{Consumer, Producer};
use scopeguard::defer;
use std::time::Duration;

static DECODERS: Lazy<DashMap<(i64, i64), crossbeam::channel::Sender<EndPointAudioFrame>>> =
    Lazy::new(DashMap::new);

pub async fn handle_audio_frame(
    active_device_id: i64,
    passive_device_id: i64,
    audio_frame: EndPointAudioFrame,
) {
    if let Some(tx) = DECODERS.get(&(active_device_id, passive_device_id)) {
        if tx.try_send(audio_frame).is_err() {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                "send audio frame failed"
            );
        }
    }
}

pub fn serve_audio_decode(active_device_id: i64, passive_device_id: i64) {
    if !DECODERS.contains_key(&(active_device_id, passive_device_id)) {
        let (audio_frame_tx, audio_frame_rx) = bounded(1024);
        DECODERS.insert((active_device_id, passive_device_id), audio_frame_tx);

        TOKIO_RUNTIME.spawn_blocking(move || {
            let mut audio_decoder = AudioPlayer::new();

            loop {
                match audio_frame_rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(audio_frame) => {
                        if let Err(err) = audio_decoder.play_samples(audio_frame) {
                            tracing::error!(?err, "audio decoder decode failed");
                            return;
                        }
                    }
                    Err(err) => {
                        if err.is_disconnected() {
                            return;
                        }
                    }
                }
            }
        });
    }
}
