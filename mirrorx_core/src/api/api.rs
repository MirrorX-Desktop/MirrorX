use once_cell::sync::Lazy;
use tokio::runtime::{Builder, Runtime};

use super::http::device_register;
use crate::{
    provider::{
        self, endpoint::EndPointProvider, http::HTTPProvider, runtime::RuntimeProvider,
        signal_a::SocketProvider,
    },
    socket::endpoint::client_to_client::StartMediaTransmissionReply,
    utility::token::parse_register_token,
};
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Once},
};

static LOGGER_INIT_ONCE: Once = Once::new();
static INIT_SUCCESS: AtomicBool = AtomicBool::new(false);

static rt: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .thread_name("MirrorXCoreTokioRuntime")
        .enable_all()
        .build()
        .unwrap()
});

macro_rules! async_block_on {
    ($future:expr) => {
        let (tx, rx) = crossbeam::channel::bounded(1);

        rt.spawn(async move { tx.send($future.await) });

        rx.recv()?
    };
}

pub async fn init(os_name: String, os_version: String, config_dir: String) -> anyhow::Result<()> {
    LOGGER_INIT_ONCE.call_once(|| {
        // env_logger::Builder::new()
        //     .filter_level(log::LevelFilter::Info)
        //     .format(|buf, record| {
        //         writeln!(
        //             buf,
        //             "[{}] [{}({}#{})] {} {}",
        //             chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
        //             record.module_path().unwrap_or(""),
        //             record.file().unwrap_or(""),
        //             record.line().unwrap_or(0),
        //             record.level(),
        //             record.args(),
        //         )
        //     })
        //     .target(env_logger::Target::Stdout)
        //     .init();
        tracing_subscriber::fmt::init();
    });

    tracing::trace!(
        os = ?os_name,
        os_version = ?os_version,
        config_dir = ?config_dir,
        "init",
    );

    if INIT_SUCCESS.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }

    crate::constants::OS_NAME.get_or_init(|| os_name);
    crate::constants::OS_VERSION.get_or_init(|| os_version);

    ConfigProvider::make_current(Path::new(&config_dir))?;
    RuntimeProvider::make_current()?;
    HTTPProvider::make_current()?;
    EndPointProvider::make_current()?;

    // ensure device id is valid
    let device_id = ConfigProvider::current()?.read_device_id()?;
    let resp = RuntimeProvider::current()?.block_on(device_register(device_id))?;
    let (device_id, expiration, _) = parse_register_token(&resp.token)?;

    tracing::trace!(
        device_id = ?device_id,
        expiration = ?expiration,
        "init register",
    );

    ConfigProvider::current()?.save_device_id(&device_id)?;
    ConfigProvider::current()?.save_device_id_expiration(&expiration)?;

    // handshake to server
    SocketProvider::make_current("192.168.0.101:40001", &resp.token)?;

    INIT_SUCCESS.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

// Config

pub fn config_read_device_id() -> anyhow::Result<Option<String>> {
    provider::config::read_device_id()
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<()> {
    provider::config::save_device_id(&device_id)
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    provider::config::read_device_id_expiration()
}

pub fn config_save_device_id_expiration(time_stamp: u32) -> anyhow::Result<()> {
    provider::config::save_device_id_expiration(&time_stamp)
}

pub fn config_read_device_password() -> anyhow::Result<Option<String>> {
    provider::config::read_device_password()
}

pub fn config_save_device_password(device_password: String) -> anyhow::Result<()> {
    provider::config::save_device_password(&device_password)
}

pub fn signaling_handshake(token: String) -> anyhow::Result<()> {
    async_block_on! {
        provider::signaling::handshake(token)
    }
}

pub fn signaling_heartbeat() -> anyhow::Result<()> {
    async_block_on! {
        provider::signaling::heartbeat()
    }
}

// pub fn desktop_connect(remote_device_id: String) -> anyhow::Result<()> {
//     RuntimeProvider::current()?.block_on(super::socket::desktop_connect(remote_device_id))
// }

// pub fn desktop_key_exchange_and_password_verify(
//     remote_device_id: String,
//     password: String,
// ) -> anyhow::Result<bool> {
//     RuntimeProvider::current()?.block_on(super::socket::desktop_key_exchange_and_password_verify(
//         remote_device_id,
//         password,
//     ))
// }

// pub fn desktop_start_media_transmission(
//     remote_device_id: String,
//     texture_id: i64,
//     video_texture_ptr: i64,
//     update_frame_callback_ptr: i64,
// ) -> anyhow::Result<StartMediaTransmissionReply> {
//     RuntimeProvider::current()?.block_on(super::socket::desktop_start_media_transmission(
//         remote_device_id,
//         texture_id,
//         video_texture_ptr,
//         update_frame_callback_ptr,
//     ))
// }

pub fn utility_generate_device_password() -> String {
    crate::utility::rng::generate_device_password()
}
