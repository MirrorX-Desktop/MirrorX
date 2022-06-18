use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SignalingMessagePacketType {
    Request,
    Response,
    Push,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignalingMessagePacket {
    pub typ: SignalingMessagePacketType,
    pub call_id: Option<u8>,
    pub message: SignalingMessage,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SignalingMessage {
    Error(SignalingMessageError),
    HeartBeatRequest(HeartBeatRequest),
    HeartBeatResponse(HeartBeatResponse),
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    ConnectRequest(ConnectRequest),
    ConnectResponse(ConnectResponse),
    ConnectionKeyExchangeRequest(ConnectionKeyExchangeRequest),
    ConnectionKeyExchangeResponse(ConnectionKeyExchangeResponse),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SignalingMessageError {
    Internal,
    Mismatched,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HeartBeatRequest {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HeartBeatResponse {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HandshakeRequest {
    pub device_token: Option<(String, String)>, // (device_id, unique_id)
    pub device_native_id: String,

    #[serde(with = "serde_bytes")]
    pub device_native_id_salt: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HandshakeResponse {
    pub device_id: String,
    pub unique_id: String,
    pub device_id_expiration: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectRequest {
    pub remote_device_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectResponse {
    pub allow: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeActiveDeviceSecret<'a> {
    #[serde(with = "serde_bytes")]
    pub response_public_key_n: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub response_public_key_e: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub active_device_public_key: &'a [u8],

    #[serde(with = "serde_bytes")]
    pub active_device_nonce: &'a [u8],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangePassiveDeviceSecret<'a> {
    #[serde(with = "serde_bytes")]
    pub passive_device_public_key: &'a [u8],

    #[serde(with = "serde_bytes")]
    pub passive_device_nonce: &'a [u8],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeRequest {
    pub active_device_id: String,

    #[serde(with = "serde_bytes")]
    pub password_derive_salt: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub secret: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub secret_nonce: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeResponse {
    pub passive_device_id: String,
    pub exchange_data: Vec<u8>,
}
