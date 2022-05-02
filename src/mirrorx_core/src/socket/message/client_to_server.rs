use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ClientToServerMessage {
    HeartBeatRequest(HeartBeatRequest),
    HandshakeRequest(HandshakeRequest),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HeartBeatRequest {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HandshakeRequest {
    pub token: String,
}
