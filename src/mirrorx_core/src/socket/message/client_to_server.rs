use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ClientToServerMessage {
    HeartBeatRequest(HeartBeatRequest),
    HandshakeRequest(HandshakeRequest),
}

impl Display for ClientToServerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HeartBeatRequest {
    pub time_stamp: u32,
}

impl Display for HeartBeatRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeartBeatRequest {{ time_stamp: {} }}", self.time_stamp)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HandshakeRequest {
    pub token: String,
}

impl Display for HandshakeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandshakeRequest {{ token: {} }}", self.token)
    }
}
