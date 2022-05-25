use log::{error, info};

#[test]
fn test_encode() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

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
        let mut bytes = 0;
        loop {
            match packet_rx.recv() {
                Ok(packet) => {
                    bytes += packet.data.len();
                }
                Err(_) => {
                    info!("packet_rx closed, {} B", bytes);
                    break;
                }
            };
        }
    });

    info!("start capture");
    desktop_duplicator.start()?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    desktop_duplicator.stop();
    info!("stop capture");

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
