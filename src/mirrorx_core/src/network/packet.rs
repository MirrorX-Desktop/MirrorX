use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Packet {
    pub call_id: u8,
    pub message: Message,
}
