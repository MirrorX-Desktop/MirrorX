use super::http::device_register;
use crate::{
    // media::bindings::macos::*,
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
use tracing::info;

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

pub fn begin_video(
    texture_id: i64,
    video_texture_ptr: i64,
    update_frame_callback_ptr: i64,
) -> anyhow::Result<()> {
    // #[cfg(target_os = "macos")]
    // unsafe {
    //     let main_display_id = core_graphics::display::CGMainDisplayID();
    // }

    // let mut encoder =
    //     crate::media::video_encoder::VideoEncoder::new("h264_videotoolbox", 60, 1920, 1080)?;
    // encoder.set_opt("profile", "high", 0)?;
    // encoder.set_opt("level", "5.2", 0)?;
    // // if encoder_name == "libx264" {
    // //     encoder.set_opt("preset", "ultrafast", 0)?;
    // //     encoder.set_opt("tune", "zerolatency", 0)?;
    // //     encoder.set_opt("sc_threshold", "499", 0)?;
    // // } else {
    // encoder.set_opt("realtime", "1", 0)?;
    // encoder.set_opt("allow_sw", "0", 0)?;
    // // }

    // let packet_rx = encoder.open()?;

    // let (mut desktop_duplicator, capture_frame_rx) =
    //     crate::media::desktop_duplicator::DesktopDuplicator::new(60)?;

    // let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

    // let frame_rx = decoder.open()?;

    // std::thread::spawn(move || unsafe {
    //     loop {
    //         let capture_frame = match capture_frame_rx.recv() {
    //             Ok(frame) => frame,
    //             Err(err) => {
    //                 tracing::error!(?err, "capture_frame_rx.recv");
    //                 return;
    //             }
    //         };

    //         let image_buffer = capture_frame.cv_pixel_buffer;

    //         // let pix_fmt = CVPixelBufferGetPixelFormatType(image_buffer);

    //         let lock_result = CVPixelBufferLockBaseAddress(image_buffer, 1);
    //         if lock_result != 0 {
    //             tracing::error!("CVPixelBufferLockBaseAddress failed");
    //             return;
    //         }

    //         let width = CVPixelBufferGetWidth(image_buffer);
    //         let height = CVPixelBufferGetHeight(image_buffer);
    //         let y_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
    //         let y_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
    //         // let y_plane_height = CVPixelBufferGetHeightOfPlane(image_buffer, 0);

    //         let uv_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
    //         let uv_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

    //         encoder.encode(
    //             width as i32,
    //             height as i32,
    //             y_plane_bytes_address as *mut u8,
    //             y_plane_stride as i32,
    //             uv_plane_bytes_address as *mut u8,
    //             uv_plane_stride as i32,
    //             0, // timing_info.decode_timestamp.value,
    //             0, // timing_info.decode_timestamp.time_scale,
    //             0, // timing_info.presentation_timestamp.value,
    //             0, // timing_info.presentation_timestamp.time_scale,
    //         );

    //         CVPixelBufferUnlockBaseAddress(image_buffer, 1);
    //     }
    // });

    // std::thread::spawn(move || {
    //     let mut total_bytes = 0;
    //     loop {
    //         match packet_rx.recv() {
    //             Ok(packet) => {
    //                 total_bytes += packet.data.len();
    //                 decoder.decode(
    //                     packet.data.as_ptr(),
    //                     packet.data.len() as i32,
    //                     packet.dts,
    //                     packet.pts,
    //                 );
    //             }
    //             Err(_) => {
    //                 tracing::info!(total_packet_bytes = total_bytes, "packet_rx closed");
    //                 break;
    //             }
    //         };
    //     }
    // });

    // std::thread::spawn(move || loop {
    //     match frame_rx.recv() {
    //         Ok(frame) => unsafe {
    //             let video_texture_ptr = video_texture_ptr as *mut c_void;
    //             let update_frame_callback_ptr = update_frame_callback_ptr as *mut c_void;
    //             let update_frame_callback = std::mem::transmute::<
    //                 *mut c_void,
    //                 unsafe extern "C" fn(
    //                     texture_id: i64,
    //                     video_texture_ptr: *mut c_void,
    //                     new_frame_ptr: *mut c_void,
    //                 ),
    //             >(update_frame_callback_ptr);

    //             update_frame_callback(texture_id, video_texture_ptr, frame.0);
    //         },
    //         Err(_) => {
    //             tracing::info!("frame_rx closed");
    //             break;
    //         }
    //     };
    // });

    // RuntimeProvider::current()?.spawn(async move {
    //     tracing::info!("start capture");
    //     let _ = desktop_duplicator.start();
    //     tokio::time::sleep(Duration::from_secs(3600)).await;
    //     desktop_duplicator.stop();
    //     tracing::info!("stop capture");
    // });

    Ok(())
}
