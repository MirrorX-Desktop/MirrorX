use crate::{
    api::endpoint::{message::EndPointAudioFrame, EndPointID},
    component::audio::player::AudioPlayer,
};
use tokio::sync::mpsc::Receiver;

pub fn serve_audio_decode(id: EndPointID, mut decode_rx: Receiver<EndPointAudioFrame>) {
    tokio::task::spawn_blocking(move || loop {
        tracing::info!(?id, "audio decode process");

        let mut audio_player = AudioPlayer::default();

        loop {
            match decode_rx.blocking_recv() {
                Some(audio_frame) => {
                    if let Err(err) = audio_player.play_samples(audio_frame) {
                        tracing::error!(?err, "audio decoder process play samples failed, process will initialize a new player");
                        break;
                    }
                }
                None => {
                    tracing::error!("audio decode process exit");
                    return;
                }
            }
        }
    });
}
