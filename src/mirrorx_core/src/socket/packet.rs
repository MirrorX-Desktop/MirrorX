use serde::{Deserialize, Serialize};

use super::message::{
    client_to_server::ClientToServerMessage, server_to_client::ServerToClientMessage,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Packet {
    /// (call_id, message)
    ClientToServer(u16, ClientToServerMessage),

    /// (call_id, message)
    ServerToClient(u16, ServerToClientMessage),

    /// (call_id, from_device_id, to_device_id, message_bytes)
    ClientToClient(u16, String, String, Vec<u8>),
}
