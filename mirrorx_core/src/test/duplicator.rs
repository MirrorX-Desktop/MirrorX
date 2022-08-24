use scopeguard::defer;

#[test]
#[cfg(target_os = "macos")]
fn test_duplicator() -> anyhow::Result<()> {
    use core_foundation::base::CFRelease;

    tracing_subscriber::fmt::init();

    unsafe {
        let (tx, rx) = crossbeam::channel::bounded(1);
        let display_id = core_graphics::display::CGMainDisplayID();
        let duplicator = crate::component::desktop::Duplicator::new(display_id, tx)?;
        duplicator.start()?;
        defer! {
           let _= duplicator.stop();
        }

        std::thread::spawn(move || loop {
            let capture_frame = match rx.recv() {
                Ok(capture_frame) => capture_frame,
                Err(err) => {
                    tracing::error!("recv surface failed({})", err);
                    return;
                }
            };

            tracing::info!(pts=?capture_frame.pts, "receive capture frame");

            CFRelease(capture_frame.pixel_buffer);
        });

        std::thread::sleep(std::time::Duration::from_secs(10));
        Ok(())
    }
}

#[test]
#[cfg(target_os = "windows")]
fn test_duplicator() -> anyhow::Result<()> {
    use std::io::Cursor;

    use image::ColorType;

    tracing_subscriber::fmt::init();
    unsafe {
        let version = windows::Win32::Media::MediaFoundation::MF_SDK_VERSION << 16
            | windows::Win32::Media::MediaFoundation::MF_API_VERSION;
        if let Err(err) = windows::Win32::Media::MediaFoundation::MFStartup(
            version,
            windows::Win32::Media::MediaFoundation::MFSTARTUP_NOSOCKET,
        ) {
            panic!("{}", err);
        } else {
            tracing::info!("MFStartup Ok");
        }

        defer! {
            let _ = windows::Win32::Media::MediaFoundation::MFShutdown();
        }

        let descriptors = crate::component::media_foundation::enumerator::enum_descriptors()?;
        if descriptors.len() == 0 {
            return Err(anyhow::anyhow!("descriptors is empty"));
        }

        tracing::info!("descriptiors: {:?}", descriptors);

        let monitors = crate::component::monitor::get_active_monitors()?;

        let mut duplicator = crate::component::desktop::Duplicator::new(&monitors[0].id)?;

        let mut video_encoder =
            crate::component::media_foundation::video_encoder::VideoEncoder::new(
                1920,
                1080,
                60,
                &descriptors[1],
                &duplicator.deivce(),
            )?;

        let capture_frame = duplicator.capture()?;

        video_encoder.encode(capture_frame)?;

        // tracing::info!(
        //     "width:{}, height:{}, bytes_length:{}, stride:{}",
        //     capture_frame.width,
        //     capture_frame.height,
        //     capture_frame.bytes.len(),
        //     capture_frame.stride
        // );

        // let mut png_bytes: Vec<u8> = Vec::with_capacity(capture_frame.bytes.len());

        // for chunk in &mut capture_frame.bytes.chunks_mut(4).into_iter() {
        //     chunk[0] = chunk[0] ^ chunk[2];
        //     chunk[2] = chunk[0] ^ chunk[2];
        //     chunk[0] = chunk[0] ^ chunk[2];
        // }

        // image::write_buffer_with_format(
        //     &mut Cursor::new(&mut png_bytes),
        //     &capture_frame.bytes,
        //     capture_frame.width as u32,
        //     capture_frame.height as u32,
        //     ColorType::Rgba8,
        //     image::ImageOutputFormat::Png,
        // )
        // .map_err(|err| anyhow::anyhow!(err))?;

        // let mut p = std::env::temp_dir();
        // p.push(format!("mirrorx_{}.png", chrono::Local::now().timestamp()));

        // tracing::info!("output png file path: {:?}", p.as_path().as_os_str());

        // std::fs::write(&p, &png_bytes)?;

        Ok(())
    }
}
