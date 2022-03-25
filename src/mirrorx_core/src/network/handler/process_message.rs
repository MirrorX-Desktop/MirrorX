use crate::network::message::Message;
use crate::network::Client;
use log::error;

use super::desktop_connect_ask::handle_desktop_connect_ask;

pub async fn process_message(client: &Client, packet: Message) -> anyhow::Result<Option<Message>> {
    match packet {
        Message::DesktopConnectAskReq(message) => {
            handle_desktop_connect_ask(client, &message).await
        }
        _ => {
            error!("unknown message type");
            Ok(None)
        }
    }
}
