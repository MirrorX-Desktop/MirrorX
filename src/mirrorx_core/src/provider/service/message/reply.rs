use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ReplyMessage {
    HeartBeatReply(HeartBeatReply),
    RegisterIdReply(RegisterIdReply),
    ConnectReply(ConnectReply),
    KeyExchangeAndVerifyPasswordReply(KeyExchangeAndVerifyPasswordReply),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HeartBeatReply {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RegisterIdReply {
    pub device_id: String,
    pub expire_at: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ConnectReply {
    pub offer_device_id: String,
    pub ask_device_id: String,
    pub pub_key_n: Vec<u8>,
    pub pub_key_e: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct KeyExchangeAndVerifyPasswordReply {
    pub offer_device_id: String,
    pub ask_device_id: String,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}
