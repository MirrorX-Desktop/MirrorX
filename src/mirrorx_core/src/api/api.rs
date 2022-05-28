use super::http::device_register;
use crate::{
    provider::{
        config::ConfigProvider, endpoint::EndPointProvider, frame_stream::FrameStreamProvider,
        http::HTTPProvider, runtime::RuntimeProvider, socket::SocketProvider,
    },
    socket::message::client_to_client::StartMediaTransmissionReply,
    utility::token::parse_register_token,
};
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use libc::c_void;
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Once},
    time::Duration,
};

static LOGGER_INIT_ONCE: Once = Once::new();
static INIT_SUCCESS: AtomicBool = AtomicBool::new(false);

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
    FrameStreamProvider::make_current()?;

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

pub fn begin_video(texture_id: i64, callback_ptr: i64) -> anyhow::Result<()> {
    let mut encoder =
        crate::media::video_encoder::VideoEncoder::new("h264_videotoolbox", 60, 1920, 1080)?;
    encoder.set_opt("profile", "high", 0)?;
    encoder.set_opt("level", "5.2", 0)?;
    // if encoder_name == "libx264" {
    //     encoder.set_opt("preset", "ultrafast", 0)?;
    //     encoder.set_opt("tune", "zerolatency", 0)?;
    //     encoder.set_opt("sc_threshold", "499", 0)?;
    // } else {
    encoder.set_opt("realtime", "1", 0)?;
    encoder.set_opt("allow_sw", "0", 0)?;
    // }

    let packet_rx = encoder.open()?;

    let mut desktop_duplicator =
        crate::media::desktop_duplicator::DesktopDuplicator::new(60, encoder)?;

    let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

    let frame_rx = decoder.open()?;

    std::thread::spawn(move || {
        let mut total_bytes = 0;
        loop {
            match packet_rx.recv() {
                Ok(packet) => {
                    total_bytes += packet.data.len();
                    tracing::info!(total_bytes = total_bytes, "send");
                    decoder.decode(
                        packet.data.as_ptr(),
                        packet.data.len() as i32,
                        packet.dts,
                        packet.pts,
                    );
                }
                Err(_) => {
                    tracing::info!(total_packet_bytes = total_bytes, "packet_rx closed");
                    break;
                }
            };
        }
    });

    std::thread::spawn(move || loop {
        match frame_rx.recv() {
            Ok(frame) => unsafe {
                let a = callback_ptr as *mut c_void;
                let f =
                    std::mem::transmute::<*mut c_void, unsafe extern "C" fn(i64, *mut c_void)>(a);

                f(texture_id, frame.0);
                tracing::info!("finish");
            },
            Err(_) => {
                tracing::info!("frame_rx closed");
                break;
            }
        };
    });

    RuntimeProvider::current()?.spawn(async move {
        tracing::info!("start capture");
        let _ = desktop_duplicator.start();
        tokio::time::sleep(Duration::from_secs(3600)).await;
        desktop_duplicator.stop();
        tracing::info!("stop capture");
    });

    Ok(())
}

// #[cfg(not(test))]
// extern "C" {
//     #[link(name = "dispatch_frame", kind = "static")]
//     pub fn dispatch_frame(
//         flutter_texture_id: i64,
//         frame_id: *mut c_void,
//         // width: u16,
//         // height: u16,
//         // is_full_color_range: bool,
//         // y_plane_buffer_address: *const u8,
//         // y_plane_stride: u32,
//         // uv_plane_buffer_address: *const u8,
//         // uv_plane_stride: u32,
//         // dts: i64,
//         // pts: i64,
//     ) -> bool;
// }

// #[no_mangle]
// pub extern "C" fn notify_release(frame_id: c_ulong) {
//     info!("release frame");
//     FrameMap.remove(&(frame_id as u64));
// }
