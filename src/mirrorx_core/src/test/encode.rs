use std::time::Duration;

use log::{error, info};

use crate::media::{self};

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_encode() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let (duplicator, duplicator_frame_rx) = media::desktop_duplicator::DesktopDuplicator::new(60)?;
    let (mut encoder, packet_rx) =
        media::video_encoder::VideoEncoder::new("libx264", 60, 1920, 1080)?;

    #[cfg(target_os = "windows")]
    let (mut decoder, frame_rx) = media::video_decoder::VideoDecoder::new("h264_cuvid")?;
    #[cfg(target_os = "macos")]
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
            Ok(frame) => {
                info!("decode frame len: {}", frame_rx.len());
                drop(frame);
            }
            Err(err) => {
                info!("frame_rx closed");
                break;
            }
        };
    });

    info!("start capture");
    duplicator.start_capture();
    tokio::time::sleep(Duration::from_secs(3600)).await;
    duplicator.stop_capture();
    info!("stop capture");

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
