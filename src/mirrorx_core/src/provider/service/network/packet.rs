use serde::{Deserialize, Serialize};

use crate::provider::service::message::{
    reply::ReplyMessage, reply_error::ReplyError, request::RequestMessage,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestPacket {
    pub call_id: u8,
    pub to_device_id: Option<String>,
    pub payload: RequestMessage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplyPacket {
    pub call_id: u8,
    pub to_device_id: Option<String>,
    pub payload: Result<ReplyMessage, ReplyError>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Packet {
    pub request_packet: Option<RequestPacket>,
    pub reply_packet: Option<ReplyPacket>,
}
