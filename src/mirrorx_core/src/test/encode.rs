#[test]
fn test_encode() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // std_logger::init();

    let encoder_name = if cfg!(target_os = "windows") {
        "libx264"
    } else {
        "h264_videotoolbox"
    };

    let (encoder, packet_rx) =
        crate::media::video_encoder::VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

    let mut desktop_duplicator =
        crate::media::desktop_duplicator::DesktopDuplicator::new(60, encoder)?;

    std::thread::spawn(move || {
        let mut total_bytes = 0;
        loop {
            match packet_rx.recv() {
                Ok(packet) => {
                    total_bytes += packet.data.len();
                }
                Err(_) => {
                    tracing::info!(total_packet_bytes = total_bytes, "packet_rx closed");
                    break;
                }
            };
        }
    });

    tracing::info!("start capture");
    desktop_duplicator.start()?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    desktop_duplicator.stop();
    tracing::info!("stop capture");

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
