use std::time::Duration;

use log::{error, info};

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_encode() -> anyhow::Result<()> {
    use crate::media::desktop_duplicator::macos::av_capture_screen_input::AVCaptureScreenInput;
    use crate::media::desktop_duplicator::macos::av_capture_session::{
        AVCaptureSession, AVCaptureSessionPreset,
    };
    use crate::media::desktop_duplicator::macos::av_capture_video_data_output::AVCaptureVideoDataOutput;
    use crate::media::desktop_duplicator::macos::bindings::*;

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // let (duplicator, duplicator_frame_rx) = media::desktop_duplicator::DesktopDuplicator::new(60)?;
    // let (mut encoder, packet_rx) =
    //     media::video_encoder::VideoEncoder::new("libx264", 60, 1920, 1080)?;

    // #[cfg(target_os = "windows")]
    // let (mut decoder, frame_rx) = media::video_decoder::VideoDecoder::new("h264_cuvid")?;
    // #[cfg(target_os = "macos")]
    // let (mut decoder, frame_rx) = media::video_decoder::VideoDecoder::new("h264")?;

    // std::thread::spawn(move || loop {
    //     match duplicator_frame_rx.recv() {
    //         Ok(frame) => {
    //             info!("duplicator frame len: {}", duplicator_frame_rx.len());
    //             if let Err(err) = encoder.encode(&frame) {
    //                 // error!("encode failed: {}", err);
    //                 break;
    //             }
    //         }
    //         Err(err) => {
    //             info!("duplicator_frame_rx closeda a ");
    //             break;
    //         }
    //     }
    // });

    // std::thread::spawn(move || loop {
    //     match packet_rx.recv() {
    //         Ok(packet) => {
    //             info!("packet len: {}", packet_rx.len());
    //             decoder.decode(&packet);
    //         }
    //         Err(err) => {
    //             info!("packet_rx closed");
    //             break;
    //         }
    //     };
    // });

    // std::thread::spawn(move || loop {
    //     match frame_rx.recv() {
    //         Ok(frame) => {
    //             info!("decode frame len: {}", frame_rx.len());
    //             drop(frame);
    //         }
    //         Err(err) => {
    //             info!("frame_rx closed");
    //             break;
    //         }
    //     };
    // });

    // info!("start capture");
    // duplicator.start_capture();
    // tokio::time::sleep(Duration::from_secs(3600)).await;
    // duplicator.stop_capture();
    // info!("stop capture");

    let mut capture_session = AVCaptureSession::new();
    capture_session.begin_configuration();
    capture_session.set_session_preset(AVCaptureSessionPreset::AVCaptureSessionPresetHigh);

    let capture_screen_input = AVCaptureScreenInput::new(0);
    capture_screen_input.set_captures_cursor(true);
    capture_screen_input.set_captures_mouse_clicks(true);
    capture_screen_input.set_min_frame_duration(unsafe {
        crate::media::desktop_duplicator::macos::bindings::CMTimeMake(1, 60)
    });

    if capture_session.can_add_input(&capture_screen_input) {
        info!("can add input");
        capture_session.add_input(capture_screen_input);
    } else {
        info!("can't add input");
    }

    let capture_video_data_output = AVCaptureVideoDataOutput::new(|cm_sample_buffer| unsafe {
        if !CMSampleBufferIsValid(cm_sample_buffer) {
            error!("invalid sample buffer");
            return;
        }

        let mut timing_info: CMSampleTimingInfo = std::mem::zeroed();
        let ret = CMSampleBufferGetSampleTimingInfo(cm_sample_buffer, 0, &mut timing_info);
        if ret != 0 {
            error!("CMSampleBufferGetSampleTimingInfo failed");
            return;
        }

        let image_buffer = CMSampleBufferGetImageBuffer(cm_sample_buffer);
        if image_buffer.is_null() {
            error!("CMSampleBufferGetImageBuffer failed");
            return;
        }

        let pix_fmt = CVPixelBufferGetPixelFormatType(image_buffer);

        let lock_result = CVPixelBufferLockBaseAddress(image_buffer, 1);
        if lock_result != 0 {
            error!("CVPixelBufferLockBaseAddress failed");
            return;
        }

        let width = CVPixelBufferGetWidth(image_buffer);
        let height = CVPixelBufferGetHeight(image_buffer);
        let y_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
        let y_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0);
        let uv_plane_stride = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
        let uv_plane_bytes_address = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1);

        info!(
            "captured width:{} height:{} y_plane_stride:{} uv_plane_stride:{}",
            width, height, y_plane_stride, uv_plane_stride
        );

        CVPixelBufferUnlockBaseAddress(image_buffer, 1);
    });

    if capture_session.can_add_output(&capture_video_data_output) {
        info!("can add output");
        capture_session.add_output(capture_video_data_output);
    } else {
        info!("can't add output");
    }

    capture_session.commit_configuration();

    capture_session.start_running();
    tokio::time::sleep(Duration::from_secs(10)).await;
    capture_session.stop_running();

    Ok(())
}

#[cfg(target_os = "windows")]
#[tokio::test]
async fn test_encode() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    // tokio::time::sleep(Duration::from_secs(30)).await;
    // std::thread::spawn( move || {
    //     let mut dup = match crate::media::desktop_duplicator::dxgi::DuplicatonManager::new(){
    //         Ok(d)=>d,
    //         Err(err)=>panic!("{}", err)
    //     };

    //     if let Err(err) = dup.capture_frame() {
    //         error!("{}", err);
    //     }
    // });

    // tokio::time::sleep(Duration::from_secs(10)).await;

    let mut dup = media::desktop_duplicator::windows::duplication::Duplication::new(0)?;

    for _ in 0..10 {
        dup.capture_frame()?;
        tokio::time::sleep(Duration::from_millis(60)).await;
    }
    Ok(())

    // unsafe {
    //     let factory = windows::Win32::Graphics::Dxgi::CreateDXGIFactory1::<
    //         windows::Win32::Graphics::Dxgi::IDXGIFactory1,
    //     >()?;

    //     for dxgi_adapter_enum_index in 0.. {
    //         if let Ok(adapter) = factory.EnumAdapters(dxgi_adapter_enum_index) {
    //             let adapter_desc = adapter.GetDesc()?;
    //             info!("{:?}", adapter_desc);

    //             for dxgi_output_enum_index in 0.. {
    //                 if let Ok(output) = adapter.EnumOutputs(dxgi_output_enum_index) {
    //                     let output_desc = output.GetDesc()?;
    //                     info!("{:?}", output_desc);
    //                 } else {
    //                     break;
    //                 }
    //             }
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(())
    // }
}
