mod client;
mod handler;
pub mod message;
mod packet;
pub use client::Client;

use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BIN_CODER: WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding> =
        DefaultOptions::new()
            .with_little_endian()
            .with_varint_encoding();
}
