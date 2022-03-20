use std::sync::Arc;

use crate::network::{
    proto::{DesktopConnectAskReq, ProtoMessage},
    Client,
};

pub async fn handle_desktop_connect_ask_req(
    client: &Client,
    req: &DesktopConnectAskReq,
) -> anyhow::Result<Option<Box<dyn ProtoMessage>>> {
    Ok(None)
}
