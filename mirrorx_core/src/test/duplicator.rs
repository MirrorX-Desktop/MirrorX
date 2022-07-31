#[test]
#[cfg(target_os = "macos")]
fn test_duplicator() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    unsafe {
        let (tx, rx) = crossbeam::channel::bounded(16);
        let display_id = core_graphics::display::CGMainDisplayID();
        let duplicator = crate::component::desktop::Duplicator::new(display_id, tx)?;
        duplicator.start()?;

        std::thread::spawn(move || loop {
            let surface_wrapper = match rx.recv() {
                Ok(surface_ref) => surface_ref,
                Err(err) => {
                    tracing::error!("recv surface failed({})", err);
                    return;
                }
            };

            tracing::info!("receive surface");

            crate::ffi::os::macos::io_surface::IOSurfaceDecrementUseCount(surface_wrapper.surface);
            core_foundation::base::CFRelease(surface_wrapper.surface);
        });

        std::thread::sleep(std::time::Duration::from_secs(30));
        Ok(())
    }
}
