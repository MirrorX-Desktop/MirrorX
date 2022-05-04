use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ClientToClientMessage {
    Error,
    ConnectRequest(ConnectRequest),
    ConnectReply(ConnectReply),
    KeyExchangeAndVerifyPasswordRequest(KeyExchangeAndVerifyPasswordRequest),
    KeyExchangeAndVerifyPasswordReply(KeyExchangeAndVerifyPasswordReply),
    StartMediaTransmissionRequest(StartMediaTransmissionRequest),
    StartMediaTransmissionReply(StartMediaTransmissionReply),
}

impl Display for ClientToClientMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ConnectRequest {}

impl Display for ConnectRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectRequest {{ }}",)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ConnectReply {
    pub pub_key_n: Vec<u8>,
    pub pub_key_e: Vec<u8>,
}

impl Display for ConnectReply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConnectReply {{ pub_key_n: {:02X?}, pub_key_e: {:02X?} }}",
            self.pub_key_n, self.pub_key_e
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct KeyExchangeAndVerifyPasswordRequest {
    pub password_secret: Vec<u8>,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}

impl Display for KeyExchangeAndVerifyPasswordRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KeyExchangeAndVerifyPasswordRequest {{ password_secret: {:02X?}, exchange_pub_key: {:02X?}, exchange_salt: {:02X?} }}",
            self.password_secret, self.exchange_pub_key,self.exchange_salt
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct KeyExchangeAndVerifyPasswordReply {
    pub password_correct: bool,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}

impl Display for KeyExchangeAndVerifyPasswordReply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KeyExchangeAndVerifyPasswordReply {{ password_correct: {}, exchange_pub_key: {:02X?}, exchange_salt: {:02X?} }}",
            self.password_correct, self.exchange_pub_key,self.exchange_salt
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct StartMediaTransmissionRequest {
    // pub support_video_types:
}

impl Display for StartMediaTransmissionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StartMediaTransmissionRequest {{  }}",)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum HostType {
    Unknown,
    Windows,
    #[allow(non_camel_case_types)]
    macOS,
    #[allow(non_camel_case_types)]
    iOS,
    Android,
    Ubuntu,
    Debian,
    Arch,
    Manjaro,
    Deepin,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct StartMediaTransmissionReply {
    pub host_type: HostType,
    pub host_major_version: String,
    pub video_type: String,
    pub audio_type: String,
}

impl Display for StartMediaTransmissionReply {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StartMediaTransmissionReply {{ host_type: {:#?}, host_major_version: {}, video_type: {}, audio_type: {} }}",
            self.host_type, self.host_major_version, self.video_type, self.audio_type
        )
    }
}
