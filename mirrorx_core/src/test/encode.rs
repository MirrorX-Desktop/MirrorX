use tracing::info;

#[cfg(target_os = "macos")]
use crate::media::bindings::macos::*;

#[test]
#[cfg(target_os="macos")]
fn test_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let encoder_name = if cfg!(target_os = "windows") {
        "libx264"
    } else {
        "h264_videotoolbox"
    };

    let mut encoder = crate::media::video_encoder::VideoEncoder::new(encoder_name, 60, 1920, 1080)?;
    encoder.set_opt("profile", "high", 0)?;
    encoder.set_opt("level", "5.2", 0)?;
    if encoder_name == "libx264" {
        encoder.set_opt("preset", "ultrafast", 0)?;
        encoder.set_opt("tune", "zerolatency", 0)?;
        encoder.set_opt("sc_threshold", "499", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let packet_rx = encoder.open()?;

    let (mut desktop_duplicator, capture_frame_rx) =
        crate::media::desktop_duplicator::DesktopDuplicator::new(60)?;

    let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

    let frame_rx = decoder.open()?;

    std::thread::spawn(move || unsafe {
        loop {
            let capture_frame = match capture_frame_rx.recv() {
                Ok(frame) => frame,
                Err(err) => {
                    tracing::error!(?err, "capture_frame_rx.recv");
                    return;
                }
            };

            let image_buffer = capture_frame.cv_pixel_buffer;

            // let pix_fmt = CVPixelBufferGetPixelFormatType(image_buffer);
            let color_matrix = CVBufferGetAttachment(
                image_buffer,
                kCVImageBufferYCbCrMatrixKey,
                std::ptr::null_mut(),
            );

            if color_matrix == kCVImageBufferYCbCrMatrix_ITU_R_601_4 as *const _ {
                info!("604")
            } else if color_matrix == kCVImageBufferYCbCrMatrix_ITU_R_709_2 as *const _ {
                info!("709")
            } else if color_matrix == kCVImageBufferYCbCrMatrix_ITU_R_2020 as *const _ {
                info!("2020")
            } else if color_matrix == kCVImageBufferYCbCrMatrix_SMPTE_240M_1995 as *const _ {
                info!("240M")
            } else {
                info!("unknown")
            }

            let lock_result = CVPixelBufferLockBaseAddress(image_buffer, 1);
            if lock_result != 0 {
                tracing::error!("CVPixelBufferLockBaseAddress failed");
                return;
            }

            let width = CVPixelBufferGetWidth(image_buffer);
            let height = CVPixelBufferGetHeight(image_buffer);
            let y_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
            let y_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
            // let y_plane_height = CVPixelBufferGetHeightOfPlane(image_buffer, 0);

            let uv_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
            let uv_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

            encoder.encode(
                width as i32,
                height as i32,
                y_plane_bytes_address as *mut u8,
                y_plane_stride as i32,
                uv_plane_bytes_address as *mut u8,
                uv_plane_stride as i32,
                0, // timing_info.decode_timestamp.value,
                0, // timing_info.decode_timestamp.time_scale,
                0, // timing_info.presentation_timestamp.value,
                0, // timing_info.presentation_timestamp.time_scale,
            );

            CVPixelBufferUnlockBaseAddress(image_buffer, 1);
        }
    });

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
            Ok(frame) => {
                tracing::info!("receive frame decoded");
            }
            Err(_) => {
                tracing::info!("frame_rx closed");
                break;
            }
        };
    });

    tracing::info!("start capture");
    desktop_duplicator.start()?;

    std::thread::sleep(std::time::Duration::from_secs(30));

    desktop_duplicator.stop();
    tracing::info!("stop capture");

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}

#[test]
fn test_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let encoder_name = if cfg!(target_os = "windows") {
        "libx264"
    } else {
        "h264_videotoolbox"
    };

    let mut encoder = crate::media::video_encoder::VideoEncoder::new(encoder_name, 60, 1920, 1080)?;
    encoder.set_opt("profile", "high", 0)?;
    encoder.set_opt("level", "5.2", 0)?;
    if encoder_name == "libx264" {
        encoder.set_opt("preset", "ultrafast", 0)?;
        encoder.set_opt("tune", "zerolatency", 0)?;
        encoder.set_opt("sc_threshold", "499", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let packet_rx = encoder.open()?;

    let (mut desktop_duplicator, capture_frame_rx) =
        crate::media::desktop_duplicator::DesktopDuplicator::new(60)?;

    let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264_qsv")?;

    let frame_rx = decoder.open()?;

    std::thread::spawn(move || unsafe {
        loop {
            let capture_frame = match capture_frame_rx.recv() {
                Ok(frame) => frame,
                Err(err) => {
                    tracing::error!(?err, "capture_frame_rx.recv");
                    return;
                }
            };

            encoder.encode(
                capture_frame.width(),
                capture_frame.height(),
                capture_frame.luminance_buffer().as_ptr(),
                capture_frame.luminance_stride() as i32,
                capture_frame.chrominance_buffer().as_ptr(),
                capture_frame.chrominance_stride() as i32,
                0, // timing_info.decode_timestamp.value,
                0, // timing_info.decode_timestamp.time_scale,
                0, // timing_info.presentation_timestamp.value,
                0, // timing_info.presentation_timestamp.time_scale,
            );

            capture_frame.notify();
        }
    });

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
            Ok(frame) => {
                tracing::info!("receive frame decoded");
            }
            Err(_) => {
                tracing::info!("frame_rx closed");
                break;
            }
        };
    });

    tracing::info!("start capture");
    desktop_duplicator.start()?;

    std::thread::sleep(std::time::Duration::from_secs(30));

    desktop_duplicator.stop();
    tracing::info!("stop capture");

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
