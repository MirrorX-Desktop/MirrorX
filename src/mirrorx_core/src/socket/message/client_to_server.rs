use serde::{Deserialize, Serialize};

use super::reply_error::ReplyError;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ClientToServerMessage {
    HeartBeatRequest(HeartBeatRequest),
    Error(ReplyError),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HeartBeatRequest {
    pub time_stamp: u32,
}
