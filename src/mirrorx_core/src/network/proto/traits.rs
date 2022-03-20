use crate::network::util::BytesWriter;
use downcast_rs::{impl_downcast, Downcast};
use std::mem::MaybeUninit;

impl_downcast!(ProtoMessage);

pub trait ProtoMessage: Send + Sync + Downcast {
    fn opcode(&self) -> u16;

    fn default() -> Self
    where
        Self: Sized + ProtoMessage,
    {
        unsafe { MaybeUninit::<Self>::zeroed().assume_init() }
    }

    fn encode(&self, writer: &mut BytesWriter);

    fn decode(&mut self, buf: &[u8]) -> anyhow::Result<()>;
}
