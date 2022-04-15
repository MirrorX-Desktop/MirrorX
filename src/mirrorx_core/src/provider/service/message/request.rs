use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum RequestMessage {
    HeartBeatRequest(HeartBeatRequest),
    RegisterIdRequest(RegisterIdRequest),
    ConnectRequest(ConnectRequest),
    KeyExchangeAndVerifyPasswordRequest(KeyExchangeAndVerifyPasswordRequest),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct HeartBeatRequest {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RegisterIdRequest {
    pub device_id: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConnectRequest {
    pub offer_device_id: String,
    pub ask_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct KeyExchangeAndVerifyPasswordRequest {
    pub offer_device_id: String,
    pub ask_device_id: String,
    pub password_secret: Vec<u8>,
    pub exchange_pub_key: Vec<u8>,
    pub exchange_salt: Vec<u8>,
}
