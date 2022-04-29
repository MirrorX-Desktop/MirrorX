use crate::provider::config::ConfigProvider;
use anyhow::anyhow;
use log::{error, warn};
use std::path::Path;

pub fn init(dir: String) -> anyhow::Result<()> {
    let provider = ConfigProvider::new(Path::new(&dir))?;

    if let Err(_) = crate::instance::CONFIG_PROVIDER_INSTANCE.set(provider) {
        warn!("config already initialized");
    }

    Ok(())
}

pub fn read_device_id() -> anyhow::Result<Option<String>> {
    config_provider_do(|provider| provider.read_device_id())
}

pub fn save_device_id(device_id: &str) -> anyhow::Result<()> {
    config_provider_do(|provider| provider.save_device_id(device_id))
}

pub fn read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    config_provider_do(|provider| provider.read_device_id_expiration())
}

pub fn save_device_id_expiration(time_stamp: u32) -> anyhow::Result<()> {
    config_provider_do(|provider| provider.save_device_id_expiration(&time_stamp))
}

pub fn read_device_password() -> anyhow::Result<Option<String>> {
    config_provider_do(|provider| provider.read_device_password())
}

pub fn save_device_password(device_password: &str) -> anyhow::Result<()> {
    config_provider_do(|provider| provider.save_device_password(device_password))
}

#[inline]
fn config_provider_do<T, R>(op: T) -> anyhow::Result<R>
where
    T: Fn(&ConfigProvider) -> anyhow::Result<R>,
{
    let provider = crate::instance::CONFIG_PROVIDER_INSTANCE
        .get()
        .ok_or(anyhow!(
            "config_provider_do: config provider not initialized"
        ))?;

    op(provider)
}
