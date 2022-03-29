use std::sync::Arc;

use log::info;
use serde::{Deserialize, Serialize};

use crate::network::{
    message::{Message, MessageError},
    Client,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskReq {
    pub offer_device_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DesktopConnectAskResp {
    pub agree: bool,
}

impl DesktopConnectAskReq {
    pub async fn handle(self, client: Arc<Client>) -> anyhow::Result<Message, MessageError> {
        info!("handle desktop connect ask: {:?}", self);

        Ok(Message::DesktopConnectAskResp(DesktopConnectAskResp {
            agree: true,
        }))
    }
}
