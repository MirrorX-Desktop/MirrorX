use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use super::{
    handler::desktop_connect_ask::{DesktopConnectAskReq, DesktopConnectAskResp},
    Client,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Message {
    None,
    Error(MessageError),
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
pub enum MessageError {
    InternalError,
    Timeout,
    InvalidArguments,
    MismatchedResponseMessage,
    RemoteClientOfflineOrNotExist,
}

impl Message {
    pub async fn handle(self, client: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        match self {
            Message::DesktopConnectAskReq(message) => message.handle(client).await,
            _ => {
                error!("unknown message type");
                Ok(Message::None)
            }
        }
    }
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
