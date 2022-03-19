use super::proto::{opcode::Opcode, ProtoMessage};
use futures::Future;
use lazy_static::lazy_static;
use std::{collections::HashMap, pin::Pin};

type MessageHandler =
    fn(
        Box<dyn ProtoMessage>,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Box<dyn ProtoMessage>>> + Send>>;

lazy_static! {
    pub static ref MESSAGE_HANDLERS: HashMap<Opcode, MessageHandler> = HashMap::new();
}
