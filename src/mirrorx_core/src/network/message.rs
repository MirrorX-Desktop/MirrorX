use std::sync::Arc;

use log::error;
use serde::{Deserialize, Serialize};

use super::{
    handler::{
        desktop_connect_ask::{DesktopConnectAskReq, DesktopConnectAskResp},
        desktop_connect_ask_auth::{DesktopConnectAskAuthReq, DesktopConnectAskAuthResp},
        desktop_connect_open_stream::{DesktopConnectOpenStreamReq, DesktopConnectOpenStreamResp},
    },
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
    DesktopConnectOfferAuthReq(DesktopConnectOfferAuthReq),
    DesktopConnectOfferAuthResp(DesktopConnectOfferAuthResp),
    DesktopConnectAskAuthReq(DesktopConnectAskAuthReq),
    DesktopConnectAskAuthResp(DesktopConnectAskAuthResp),
    DesktopConnectOpenStreamReq(DesktopConnectOpenStreamReq),
    DesktopConnectOpenStreamResp(DesktopConnectOpenStreamResp),
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
            Message::DesktopConnectAskAuthReq(message) => message.handle(client).await,
            Message::DesktopConnectOpenStreamReq(message) => message.handle(client).await,
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
    pub password_auth_public_key_n: Vec<u8>,
    pub password_auth_public_key_e: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOfferAuthReq {
    pub offer_device_id: String,
    pub ask_device_id: String,
    pub secret_message: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOfferAuthResp {
    pub password_correct: bool,
}
