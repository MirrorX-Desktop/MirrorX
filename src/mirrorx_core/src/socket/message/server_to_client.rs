use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum ServerToClientMessage {
    Error,
    HeartBeatReply(HeartBeatReply),
    HandshakeReply(HandshakeReply),
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HeartBeatReply {
    pub time_stamp: u32,
}

impl Display for HeartBeatReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeartBeatReply {{ time_stamp: {} }}", self.time_stamp)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum HandshakeStatus {
    Accepted,
    Repeated,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct HandshakeReply {
    pub status: HandshakeStatus,
}

impl Display for HandshakeReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HandshakeReply {{ status: {:?} }}", self.status)
    }
}
