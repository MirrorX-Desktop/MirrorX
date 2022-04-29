use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ServerToClientMessage {
    HeartBeatReply(HeartBeatReply),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HeartBeatReply {
    pub time_stamp: u32,
}
