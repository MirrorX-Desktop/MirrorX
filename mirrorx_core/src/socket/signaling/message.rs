use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SignalingMessagePacket {
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
    ConnectRemoteRequest(ConnectRemoteRequest),
    ConnectRemoteResponse(ConnectRemoteResponse),
    ConnectionKeyExchangeRequest(ConnectionKeyExchangeRequest),
    ConnectionKeyExchangeResponse(ConnectionKeyExchangeResponse),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SignalingMessageError {
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HandshakeResponse {
    pub device_id: String,
    pub unique_id: String,
    pub device_id_expiration: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectRemoteRequest {
    pub remote_device_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectRemoteResponse {
    pub allow: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeAskDeviceData {
    pub salt: Vec<u8>,
    pub secret: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeAskDeviceSecret {
    pub offer_device_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub nonce: [u8; 8],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeOfferDeviceSecret {
    pub public_key: Vec<u8>,
    pub nonce: [u8; 8],
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeRequest {
    pub exchange_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ConnectionKeyExchangeResponse {
    pub exchange_data: Vec<u8>,
}
