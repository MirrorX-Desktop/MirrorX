use log::info;

use super::http::device_register;
use crate::{
    provider::{
        config::ConfigProvider, endpoint::EndPointProvider, http::HTTPProvider,
        runtime::RuntimeProvider, socket::SocketProvider,
    },
    utility::token::parse_register_token,
};
use std::{io::Write, path::Path, sync::Once};

static LOGGER_INIT_ONCE: Once = Once::new();

pub fn init(config_dir: String) -> anyhow::Result<()> {
    LOGGER_INIT_ONCE.call_once(|| {
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
    });

    ConfigProvider::make_current(Path::new(&config_dir))?;
    RuntimeProvider::make_current()?;
    HTTPProvider::make_current()?;
    EndPointProvider::make_current()?;

    // ensure device id is valid
    let device_id = ConfigProvider::current()?.read_device_id()?;
    let resp = RuntimeProvider::current()?.block_on(device_register(device_id))?;
    let (device_id, expiration, _) = parse_register_token(&resp.token)?;
    info!(
        "init: register success, device_id: {}, expiration: {}",
        device_id, expiration
    );

    ConfigProvider::current()?.save_device_id(&device_id)?;
    ConfigProvider::current()?.save_device_id_expiration(&expiration)?;

    // handshake to server
    SocketProvider::make_current("192.168.0.101:40001", &resp.token)?;

    Ok(())
}

// Config

pub fn config_read_device_id() -> anyhow::Result<Option<String>> {
    ConfigProvider::current()?.read_device_id()
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<()> {
    ConfigProvider::current()?.save_device_id(&device_id)
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    ConfigProvider::current()?.read_device_id_expiration()
}

pub fn config_save_device_id_expiration(time_stamp: u32) -> anyhow::Result<()> {
    ConfigProvider::current()?.save_device_id_expiration(&time_stamp)
}

pub fn config_read_device_password() -> anyhow::Result<Option<String>> {
    ConfigProvider::current()?.read_device_password()
}

pub fn config_save_device_password(device_password: String) -> anyhow::Result<()> {
    ConfigProvider::current()?.save_device_password(&device_password)
}

pub fn socket_desktop_connect(remote_device_id: String) -> anyhow::Result<()> {
    RuntimeProvider::current()?.block_on(super::socket::desktop_connect(remote_device_id))
}

pub fn socket_desktop_key_exchange_and_password_verify(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<bool> {
    RuntimeProvider::current()?.block_on(super::socket::desktop_key_exchange_and_password_verify(
        remote_device_id,
        password,
    ))
}

pub fn utility_generate_device_password() -> String {
    crate::utility::rng::generate_device_password()
}
