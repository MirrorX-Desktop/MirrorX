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
