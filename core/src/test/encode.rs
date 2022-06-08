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

    let mut desktop_duplicator =
        crate::media::desktop_duplicator::DesktopDuplicator::new(60, encoder)?;

    let mut decoder = crate::media::video_decoder::VideoDecoder::new("h264")?;

    let frame_rx = decoder.open()?;

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
