use std::sync::Arc;

use crate::{
    provider::{config::ConfigProvider, http::HTTPProvider},
    socket::{endpoint::EndPoint, Streamer},
};
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

pub static CONFIG_PROVIDER_INSTANCE: OnceCell<ConfigProvider> = OnceCell::new();

pub static RUNTIME_INSTANCE: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("MirrorXCoreTokioRuntime")
        .enable_all()
        .build()
        .expect("create tokio runtime failed")
});

pub static SOCKET_ENDPOINT_MAP: Lazy<DashMap<String, EndPoint>> = Lazy::new(|| DashMap::new());

pub static STREAMER_INSTANCE: Lazy<Arc<Streamer>> = Lazy::new(|| {
    RUNTIME_INSTANCE
        .block_on(Streamer::connect("192.168.0.101:40001"))
        .expect("create socket streamer failed")
});

pub static HTTP_INSTANCE: Lazy<HTTPProvider> =
    Lazy::new(|| HTTPProvider::new().expect("create http client failed"));

pub static BINCODE_INSTANCE: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});
