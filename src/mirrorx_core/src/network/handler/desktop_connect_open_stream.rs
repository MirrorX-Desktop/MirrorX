use std::sync::Arc;

use serde::{Serialize, Deserialize};

use crate::network::{Client, message::{Message, MessageError}};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOpenStreamReq{
    pub offer_device_id:String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectOpenStreamResp{
    
}

impl DesktopConnectOpenStreamReq {
    pub async fn handle(self, _: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        Ok(Message::DesktopConnectOpenStreamResp(DesktopConnectOpenStreamResp{}))
    }
}