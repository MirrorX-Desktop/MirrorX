use crate::utility::tokio_runtime::TOKIO_RUNTIME;
use crate::{provider, socket::endpoint::message::StartMediaTransmissionResponse};
use std::sync::{atomic::AtomicBool, Once};
use tracing::trace;

static LOGGER_INIT_ONCE: Once = Once::new();
static INIT_SUCCESS: AtomicBool = AtomicBool::new(false);

macro_rules! async_block_on {
    ($future:expr) => {{
        let (tx, rx) = crossbeam::channel::bounded(1);

        TOKIO_RUNTIME.spawn(async move { tx.try_send($future.await) });

        rx.recv()
            .map_err(|err| {
                anyhow::anyhow!("async_block_on: receive call result failed ({:?})", err)
            })?
            .map_err(|err| anyhow::anyhow!(err))
    }};
}

pub fn init(os_name: String, os_version: String, config_dir: String) -> anyhow::Result<()> {
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

    trace!(
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

    provider::config::init(config_dir)?;
    async_block_on!(provider::signaling::init("192.168.0.101:28000"))?;
    async_block_on!(provider::signaling::handshake())?;
    provider::signaling::begin_heartbeat();

    INIT_SUCCESS.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

// Config

pub fn config_read_device_id() -> anyhow::Result<Option<String>> {
    provider::config::read_device_id().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<()> {
    provider::config::save_device_id(&device_id).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    provider::config::read_device_id_expiration().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id_expiration(time_stamp: i32) -> anyhow::Result<()> {
    provider::config::save_device_id_expiration(&time_stamp).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_password() -> anyhow::Result<Option<String>> {
    provider::config::read_device_password().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_password(device_password: String) -> anyhow::Result<()> {
    provider::config::save_device_password(&device_password).map_err(|err| anyhow::anyhow!(err))
}

pub fn signaling_connect(remote_device_id: String) -> anyhow::Result<bool> {
    async_block_on! {
        provider::signaling::connect(remote_device_id)
    }
}

pub fn signaling_connection_key_exchange(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<()> {
    async_block_on! {
        provider::signaling::connection_key_exchange(remote_device_id, password)
    }
}

pub fn endpoint_start_media_transmission(
    remote_device_id: String,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> anyhow::Result<StartMediaTransmissionResponse> {
    async_block_on! {
        provider::endpoint::start_media_transmission(
            remote_device_id,
            texture_id,
            video_texture_ptr,
            update_frame_callback_ptr,
        )
    }
}
