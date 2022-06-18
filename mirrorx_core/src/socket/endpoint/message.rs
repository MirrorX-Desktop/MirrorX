use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointMessagePacket {
    pub call_id: Option<u16>,
    pub message: EndPointMessage,
}

impl EndPointMessagePacket {
    pub fn new(call_id: Option<u16>, message: EndPointMessage) -> Self {
        Self { call_id, message }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error,
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    StartMediaTransmissionRequest(StartMediaTransmissionRequest),
    StartMediaTransmissionReply(StartMediaTransmissionReply),
    MediaFrame(MediaFrame),
}

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
pub struct StartMediaTransmissionReply {
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
