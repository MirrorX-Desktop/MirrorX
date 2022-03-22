use crate::network::message::Message;
use crate::network::Client;
use log::error;

pub async fn process_message(client: &Client, packet: Message) -> anyhow::Result<Option<Message>> {
    match packet {
        // Message::HeartBeatReq(message) => handle_heart_beat(client, &message).await,
        _ => {
            error!("unknown message type");
            Ok(None)
        }
    }
}
