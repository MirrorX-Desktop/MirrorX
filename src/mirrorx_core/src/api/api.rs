use std::{io::Write, sync::Once};

use once_cell::sync::OnceCell;

static ALREADY_INITIALIZED: OnceCell<()> = OnceCell::new();

pub fn init(config_dir: String) -> anyhow::Result<()> {
    if ALREADY_INITIALIZED.get().is_some() {
        return Ok(());
    }

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}({}#{})] {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                record.module_path().unwrap_or(""),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.level(),
                record.args(),
            )
        })
        .target(env_logger::Target::Stdout)
        .init();

    super::runtime::init()?;
    super::config::init(config_dir)?;
    super::service::init()?;

    let _ = ALREADY_INITIALIZED.set(());
    Ok(())
}

// Config

pub fn config_read_device_id() -> anyhow::Result<Option<String>> {
    super::config::read_device_id().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<()> {
    super::config::save_device_id(&device_id).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    super::config::read_device_id_expiration().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id_expiration(time_stamp: u32) -> anyhow::Result<()> {
    super::config::save_device_id_expiration(time_stamp).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_password() -> anyhow::Result<Option<String>> {
    super::config::read_device_password().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_password(device_password: String) -> anyhow::Result<()> {
    super::config::save_device_password(&device_password).map_err(|err| anyhow::anyhow!(err))
}

pub fn service_register_id() -> anyhow::Result<()> {
    super::service::device_register_id().map_err(|err| anyhow::anyhow!(err))
}

pub fn service_desktop_connect(ask_device_id: String) -> anyhow::Result<()> {
    super::service::desktop_connect(ask_device_id).map_err(|err| anyhow::anyhow!(err))
}

pub fn service_desktop_key_exchange_and_password_verify(
    ask_device_id: String,
    password: String,
) -> anyhow::Result<()> {
    super::service::desktop_key_exchange_and_password_verify(ask_device_id, password)
        .map_err(|err| anyhow::anyhow!(err))
}

pub fn utility_generate_device_password() -> String {
    crate::utility::rng::generate_device_password()
}
