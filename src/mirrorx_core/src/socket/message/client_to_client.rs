use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ClientToClientMessage {
    Error,
    ConnectRequest(ConnectRequest),
    ConnectReply(ConnectReply),
    KeyExchangeAndVerifyPasswordRequest(KeyExchangeAndVerifyPasswordRequest),
    KeyExchangeAndVerifyPasswordReply(KeyExchangeAndVerifyPasswordReply),
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ConnectRequest {}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ConnectReply {
    pub pub_key_n: Vec<u8>,
    pub pub_key_e: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct KeyExchangeAndVerifyPasswordRequest {
    pub password_secret: Vec<u8>,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Clone, Debug)]
pub struct KeyExchangeAndVerifyPasswordReply {
    pub success: bool,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}
