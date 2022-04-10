use super::{duplicator, video_encoder};

pub struct Producer {
    duplicator: duplicator::Duplicator,
    video_encoder: video_encoder::VideoEncoder,
}

impl Producer {
    pub fn new(encoder_name: String) -> anyhow::Result<Self> {
        let (duplicator, frame_rx) = duplicator::Duplicator::new()?;
        let (video_encoder, nalu_rx) = video_encoder::VideoEncoder::new(encoder_name, frame_rx)?;

        Ok(Producer {
            duplicator,
            video_encoder,
        })
    }

    pub fn start(&self) {
        self.duplicator.start_capture();
    }

    pub fn stop(&self) {
        self.duplicator.stop_capture()
    }
}
