use std::path::Path;

use log::{error, warn};

use crate::{api::api_error::APIError, provider::config::ConfigProvider};

pub fn init(dir: String) -> anyhow::Result<()> {
    let provider = ConfigProvider::new(Path::new(&dir))?;

    if let Err(_) = crate::instance::CONFIG_PROVIDER_INSTANCE.set(provider) {
        warn!("config already initialized");
    }

    Ok(())
}

pub fn read_device_id() -> anyhow::Result<Option<String>, APIError> {
    config_provider_do(|provider| {
        provider.read_device_id().map_err(|err| {
            error!("read device id failed: {:?}", err);
            APIError::ConfigReadFailed
        })
    })
}

pub fn save_device_id(device_id: &str) -> anyhow::Result<(), APIError> {
    config_provider_do(|provider| {
        provider.save_device_id(device_id).map_err(|err| {
            error!("save device id failed: {:?}", err);
            APIError::ConfigSaveFailed
        })
    })
}

pub fn read_device_id_expiration() -> anyhow::Result<Option<u32>, APIError> {
    config_provider_do(|provider| {
        provider.read_device_id_expiration().map_err(|err| {
            error!("read device id expiration failed: {:?}", err);
            APIError::ConfigReadFailed
        })
    })
}

pub fn save_device_id_expiration(time_stamp: u32) -> anyhow::Result<(), APIError> {
    config_provider_do(|provider| {
        provider
            .save_device_id_expiration(&time_stamp)
            .map_err(|err| {
                error!("save device id expiration failed: {:?}", err);
                APIError::ConfigSaveFailed
            })
    })
}

pub fn read_device_password() -> anyhow::Result<Option<String>, APIError> {
    config_provider_do(|provider| {
        provider.read_device_password().map_err(|err| {
            error!("read device password failed: {:?}", err);
            APIError::ConfigReadFailed
        })
    })
}

pub fn save_device_password(device_password: &str) -> anyhow::Result<(), APIError> {
    config_provider_do(|provider| {
        provider
            .save_device_password(device_password)
            .map_err(|err| {
                error!("save device password failed: {:?}", err);
                APIError::ConfigSaveFailed
            })
    })
}

#[inline]
fn config_provider_do<T, R>(op: T) -> anyhow::Result<R, APIError>
where
    T: Fn(&ConfigProvider) -> anyhow::Result<R, APIError>,
{
    crate::instance::CONFIG_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| APIError::ConfigNotInitialized)
        .and_then(op)
}
