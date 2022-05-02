use serde::{Deserialize, Serialize};

use super::reply_error::ReplyError;

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
    pub device_id: String,
    pub token: String,
}
