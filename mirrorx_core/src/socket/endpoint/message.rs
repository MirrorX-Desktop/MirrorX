use crate::error::MirrorXError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessagePacketType {
    Request,
    Response,
    Push,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointMessagePacket {
    pub typ: EndPointMessagePacketType,
    pub call_id: Option<u16>,
    pub message: EndPointMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error(MirrorXError),
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    StartMediaTransmissionRequest(StartMediaTransmissionRequest),
    StartMediaTransmissionResponse(StartMediaTransmissionResponse),
    MediaFrame(MediaFrame),
}
//
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub enum EndPointMessageError {
//     Mismatched,
//     Internal,
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HandshakeRequest {
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HandshakeResponse {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StartMediaTransmissionRequest {
    // pub support_video_types:
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StartMediaTransmissionResponse {
    pub os_name: String,
    pub os_version: String,
    pub video_type: String,
    pub audio_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MediaFrame {
    pub data: Vec<u8>,
    pub timestamp: u64,
}
