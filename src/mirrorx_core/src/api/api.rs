use dashmap::DashMap;
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use log::{error, info};
use once_cell::sync::{Lazy, OnceCell};

use super::http::device_register;
use crate::{
    media::{self, video_frame::VideoFrame},
    provider::{
        config::ConfigProvider,
        endpoint::EndPointProvider,
        frame_stream::{self, FrameStreamProvider},
        http::HTTPProvider,
        runtime::RuntimeProvider,
        socket::SocketProvider,
    },
    socket::message::client_to_client::StartMediaTransmissionReply,
    utility::token::parse_register_token,
};
use std::{
    ffi::c_void,
    io::Write,
    os::raw::{c_int, c_long, c_longlong, c_ulong},
    path::Path,
    sync::{atomic::AtomicBool, Once},
    time::Duration,
};

static LOGGER_INIT_ONCE: Once = Once::new();
static INIT_SUCCESS: AtomicBool = AtomicBool::new(false);
static FrameMap: Lazy<dashmap::DashMap<u64, VideoFrame>> = Lazy::new(|| DashMap::new());

pub fn init(os_name: String, os_version: String, config_dir: String) -> anyhow::Result<()> {
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

    info!(
        "init: os_name={}, os_version={}, config_dir={}",
        os_name, os_version, config_dir
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
    FrameStreamProvider::make_current()?;

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

    INIT_SUCCESS.store(true, std::sync::atomic::Ordering::SeqCst);

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

pub fn desktop_connect(remote_device_id: String) -> anyhow::Result<()> {
    RuntimeProvider::current()?.block_on(super::socket::desktop_connect(remote_device_id))
}

pub fn desktop_key_exchange_and_password_verify(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<bool> {
    RuntimeProvider::current()?.block_on(super::socket::desktop_key_exchange_and_password_verify(
        remote_device_id,
        password,
    ))
}

pub fn desktop_start_media_transmission(
    remote_device_id: String,
) -> anyhow::Result<StartMediaTransmissionReply> {
    RuntimeProvider::current()?.block_on(super::socket::desktop_start_media_transmission(
        remote_device_id,
    ))
}

pub fn desktop_register_frame_stream(
    stream_sink: StreamSink<ZeroCopyBuffer<Vec<u8>>>,
    remote_device_id: String,
) -> anyhow::Result<()> {
    FrameStreamProvider::current()?.add(remote_device_id, stream_sink);
    Ok(())
}

pub fn utility_generate_device_password() -> String {
    crate::utility::rng::generate_device_password()
}

pub fn begin_video(texture_id: i64) -> anyhow::Result<()> {
    let (duplicator, duplicator_frame_rx) = media::desktop_duplicator::DesktopDuplicator::new(60)?;
    let (mut encoder, packet_rx) =
        media::video_encoder::VideoEncoder::new("libx264", 60, 1920, 1080)?;
    let (mut decoder, frame_rx) = media::video_decoder::VideoDecoder::new("h264")?;

    std::thread::spawn(move || loop {
        match duplicator_frame_rx.recv() {
            Ok(frame) => {
                info!("duplicator frame len: {}", duplicator_frame_rx.len());
                if let Err(err) = encoder.encode(&frame) {
                    // error!("encode failed: {}", err);
                    break;
                }
            }
            Err(err) => {
                info!("duplicator_frame_rx closeda a ");
                break;
            }
        }
    });

    std::thread::spawn(move || loop {
        match packet_rx.recv() {
            Ok(packet) => {
                info!("packet len: {}", packet_rx.len());
                decoder.decode(&packet);
            }
            Err(err) => {
                info!("packet_rx closed");
                break;
            }
        };
    });

    std::thread::spawn(move || loop {
        match frame_rx.recv() {
            Ok(frame) => unsafe {
                dispatch_frame(
                    texture_id,
                    0,
                    frame.width,
                    frame.height,
                    frame.is_full_color_range,
                    frame.y_plane_buffer.as_ptr(),
                    frame.y_plane_stride,
                    frame.uv_plane_buffer.as_ptr(),
                    frame.uv_plane_stride,
                    frame.dts,
                    frame.pts,
                );
            },
            Err(err) => {
                info!("frame_rx closed");
                break;
            }
        };
    });

    RuntimeProvider::current()?.spawn(async move {
        info!("start capture");
        duplicator.start_capture();
        tokio::time::sleep(Duration::from_secs(3600)).await;
        duplicator.stop_capture();
        info!("stop capture");
    });

    Ok(())
}

/// cbindgen:ignore
extern "C" {
    pub fn dispatch_frame(
        flutter_texture_id: c_long,
        frame_id: c_ulong,
        width: u16,
        height: u16,
        is_full_color_range: bool,
        y_plane_buffer_address: *const u8,
        y_plane_stride: u32,
        uv_plane_buffer_address: *const u8,
        uv_plane_stride: u32,
        dts: i64,
        pts: i64,
    ) -> bool;
}

// #[no_mangle]
// pub extern "C" fn notify_release(frame_id: c_ulong) {
//     info!("release frame");
//     FrameMap.remove(&(frame_id as u64));
// }
