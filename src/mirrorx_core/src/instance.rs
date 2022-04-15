use crate::provider::{
    config::ConfigProvider, runtime::RuntimeProvider, service::service::ServiceProvider,
};
use bincode::{
    config::{LittleEndian, VarintEncoding, WithOtherEndian, WithOtherIntEncoding},
    DefaultOptions, Options,
};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

pub static CONFIG_PROVIDER_INSTANCE: OnceCell<ConfigProvider> = OnceCell::new();
pub static RUNTIME_PROVIDER_INSTANCE: OnceCell<RuntimeProvider> = OnceCell::new();
pub static SERVICE_PROVIDER_INSTANCE: OnceCell<ServiceProvider> = OnceCell::new();

pub static BINCODE_INSTANCE: Lazy<
    WithOtherIntEncoding<WithOtherEndian<DefaultOptions, LittleEndian>, VarintEncoding>,
> = Lazy::new(|| {
    bincode::DefaultOptions::new()
        .with_little_endian()
        .with_varint_encoding()
});
