use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ServerToClientMessage {
    HeartBeatReply(HeartBeatReply),
    HandshakeReply(HandshakeReply),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HeartBeatReply {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum HandshakeStatus {
    Accepted,
    Repeated,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HandshakeReply {
    pub status: HandshakeStatus,
}
