use super::api_error::APIError;

pub fn init(config_dir: String) -> anyhow::Result<()> {
    super::runtime::init()?;
    super::config::init(config_dir)?;
    super::service::init()?;
    Ok(())
}

// Config

pub fn config_read_device_id() -> anyhow::Result<Option<String>, APIError> {
    super::config::read_device_id()
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<(), APIError> {
    super::config::save_device_id(&device_id)
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>, APIError> {
    super::config::read_device_id_expiration()
}

pub fn config_save_device_id_expiration(time_stamp: u32) -> anyhow::Result<(), APIError> {
    super::config::save_device_id_expiration(time_stamp)
}

pub fn read_device_password() -> anyhow::Result<Option<String>, APIError> {
    super::config::read_device_password()
}

pub fn save_device_password(device_password: &str) -> anyhow::Result<(), APIError> {
    super::config::save_device_password(device_password)
}

pub fn service_register_id() -> anyhow::Result<(), APIError> {
    super::service::device_register_id()
}
