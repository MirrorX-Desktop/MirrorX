use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Message {
    LoginReq(LoginReq),
    LoginResp(LoginResp),
    HeartBeatReq(HeartBeatReq),
    HeartBeatResp(HeartBeatResp),
    DesktopConnectOfferReq(DesktopConnectOfferReq),
    DesktopConnectOfferResp(DesktopConnectOfferResp),
    DesktopConnectAskReq(DesktopConnectAskReq),
    DesktopConnectAskResp(DesktopConnectAskResp),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LoginReq {
    pub pub_key: [u8; 32],
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct LoginResp {
    pub device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HeartBeatReq {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HeartBeatResp {
    pub time_stamp: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOfferReq {
    pub device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOfferResp {
    pub allow: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskReq {
    pub offer_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskResp {
    pub agree: bool,
}
