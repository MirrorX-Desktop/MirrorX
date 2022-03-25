use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Message {
    DeviceGoesOnlineReq(DeviceGoesOnlineReq),
    DeviceGoesOnlineResp(DeviceGoesOnlineResp),
    HeartBeatReq(HeartBeatReq),
    HeartBeatResp(HeartBeatResp),
    DesktopConnectOfferReq(DesktopConnectOfferReq),
    DesktopConnectOfferResp(DesktopConnectOfferResp),
    DesktopConnectAskReq(DesktopConnectAskReq),
    DesktopConnectAskResp(DesktopConnectAskResp),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DeviceGoesOnlineReq {
    pub device_id: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DeviceGoesOnlineResp {
    pub device_id: String,
    pub device_id_expire_time_stamp: u32,
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
    pub offer_device_id: String,
    pub ask_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOfferResp {
    pub agree: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskReq {
    pub offer_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskResp {
    pub agree: bool,
}
