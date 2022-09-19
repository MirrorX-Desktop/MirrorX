use super::handshake::EndPointMediaMessage;
use crate::api::endpoint::message::EndPointAudioFrame;
use flutter_rust_bridge::StreamSink;

pub async fn handle_audio_frame(
    active_device_id: i64,
    passive_device_id: i64,
    audio_frame: EndPointAudioFrame,
) {
}
