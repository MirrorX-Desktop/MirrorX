use log::info;

use crate::network::{
    message::{DesktopConnectAskReq, DesktopConnectAskResp, Message},
    Client,
};

pub async fn handle_desktop_connect_ask(
    client: &Client,
    req: &DesktopConnectAskReq,
) -> anyhow::Result<Option<Message>> {
    info!("handle desktop connect ask: {:?}", req);

    Ok(Some(Message::DesktopConnectAskResp(
        DesktopConnectAskResp { agree: true },
    )))
}
