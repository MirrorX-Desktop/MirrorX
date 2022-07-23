use crate::service::endpoint::message::{GetDisplayInfoResponse, InputEvent};
use crate::utility::runtime::TOKIO_RUNTIME;
use crate::{api, service::endpoint::message::StartMediaTransmissionResponse};
use std::sync::{atomic::AtomicBool, Once};
use tracing::info;

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

pub fn init(os_version: String, config_dir: String) -> anyhow::Result<()> {
    LOGGER_INIT_ONCE.call_once(|| {
        tracing_subscriber::fmt::init();
    });

    info!(?os_version, ?config_dir, "init",);

    if INIT_SUCCESS.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }

    crate::constants::os::OS_VERSION.get_or_init(|| os_version);

    api::config::init(config_dir)?;
    async_block_on!(api::signaling::init("192.168.0.101:28000"))?;
    async_block_on!(api::signaling::handshake())?;
    api::signaling::begin_heartbeat();

    INIT_SUCCESS.store(true, std::sync::atomic::Ordering::SeqCst);

    Ok(())
}

pub fn config_read_device_id() -> anyhow::Result<Option<String>> {
    api::config::read_device_id().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id(device_id: String) -> anyhow::Result<()> {
    api::config::save_device_id(&device_id).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    api::config::read_device_id_expiration().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_id_expiration(time_stamp: i32) -> anyhow::Result<()> {
    api::config::save_device_id_expiration(&time_stamp).map_err(|err| anyhow::anyhow!(err))
}

pub fn config_read_device_password() -> anyhow::Result<Option<String>> {
    api::config::read_device_password().map_err(|err| anyhow::anyhow!(err))
}

pub fn config_save_device_password(device_password: String) -> anyhow::Result<()> {
    api::config::save_device_password(&device_password).map_err(|err| anyhow::anyhow!(err))
}

pub fn signaling_connect(remote_device_id: String) -> anyhow::Result<bool> {
    async_block_on! {
        api::signaling::connect(remote_device_id)
    }
}

pub fn signaling_connection_key_exchange(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<()> {
    async_block_on! {
        api::signaling::connection_key_exchange(remote_device_id, password)
    }
}

pub fn endpoint_get_display_info(
    remote_device_id: String,
) -> anyhow::Result<GetDisplayInfoResponse> {
    async_block_on! {
        api::endpoint::get_display_info(
            remote_device_id
        )
    }
}

pub fn endpoint_start_media_transmission(
    remote_device_id: String,
    expect_fps: u8,
    expect_display_id: String,
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> anyhow::Result<StartMediaTransmissionResponse> {
    async_block_on! {
        api::endpoint::start_media_transmission(
            remote_device_id,
            expect_fps,
            expect_display_id,
            texture_id,
            video_texture_ptr,
            update_frame_callback_ptr,
        )
    }
}

pub fn endpoint_input(remote_device_id: String, event: InputEvent) -> anyhow::Result<()> {
    async_block_on! {
        api::endpoint::input(remote_device_id, event)
    }
}

pub fn endpoint_manually_close(remote_device_id: String) {
    api::endpoint::manually_close(remote_device_id);
}

pub fn endpoint_close_notify(
    remote_device_id: String,
    sink: flutter_rust_bridge::StreamSink<()>,
) -> anyhow::Result<()> {
    let rx = api::endpoint::register_close_notificaton(remote_device_id)
        .map_err(|err| anyhow::anyhow!(err))?;
    let _ = rx.recv();
    sink.add(());
    Ok(())
}
