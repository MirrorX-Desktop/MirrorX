use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EndPointMessagePacket {
    call_id: Option<u16>,
    message: EndPointMessage,
}

impl EndPointMessagePacket {
    pub fn new(call_id: Option<u16>, message: EndPointMessage) -> Self {
        Self { call_id, message }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EndPointMessage {
    Error,

    StartMediaTransmissionRequest(StartMediaTransmissionRequest),
    StartMediaTransmissionReply(StartMediaTransmissionReply),
    MediaFrame(MediaFrame),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectRequest {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectReply {
    pub pub_key_n: Vec<u8>,
    pub pub_key_e: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KeyExchangeAndVerifyPasswordRequest {
    pub password_secret: Vec<u8>,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KeyExchangeAndVerifyPasswordReply {
    pub password_correct: bool,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}

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
