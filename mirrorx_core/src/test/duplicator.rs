// use scopeguard::defer;

// #[test]
// #[cfg(target_os = "macos")]
// fn test_duplicator() -> anyhow::Result<()> {
//     use core_foundation::base::CFRelease;

//     tracing_subscriber::fmt::init();

//     unsafe {
//         let (tx, rx) = crossbeam::channel::bounded(1);
//         let display_id = core_graphics::display::CGMainDisplayID();
//         let duplicator = crate::component::desktop::Duplicator::new(display_id, tx)?;
//         duplicator.start()?;
//         defer! {
//            let _= duplicator.stop();
//         }

//         std::thread::spawn(move || loop {
//             let capture_frame = match rx.recv() {
//                 Ok(capture_frame) => capture_frame,
//                 Err(err) => {
//                     tracing::error!("recv surface failed({})", err);
//                     return;
//                 }
//             };

//             tracing::info!(pts=?capture_frame.pts, "receive capture frame");

//             CFRelease(capture_frame.pixel_buffer);
//         });

//         std::thread::sleep(std::time::Duration::from_secs(10));
//         Ok(())
//     }
// }

// #[test]
// #[cfg(target_os = "windows")]
// fn test_duplicator() -> anyhow::Result<()> {
//     use bytes::Buf;

//     tracing_subscriber::fmt::init();

//     let (monitor_id, _, _) = crate::component::desktop::monitor::get_primary_monitor_params()?;

//     let (mut duplicator, _) = crate::component::desktop::Duplicator::new(Some(monitor_id))?;

//     let capture_frame = duplicator.capture()?;

//     let dump_path = std::env::temp_dir().join("first_image");
//     tracing::info!(?dump_path, "dump path");
//     std::fs::write(
//         dump_path,
//         bytes::Bytes::copy_from_slice(&capture_frame.luminance_bytes)
//             .chain(bytes::Bytes::copy_from_slice(
//                 &capture_frame.chrominance_bytes,
//             ))
//             .chunk(),
//     )
//     .unwrap();

//     Ok(())
// }
