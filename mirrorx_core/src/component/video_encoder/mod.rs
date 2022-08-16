use crate::{error::MirrorXError, service::endpoint::message::EndPointMessagePacket};

mod video_encoder;

// #[cfg(not(target_os = "macos"))]
// pub use video_encoder::VideoEncoder;

#[cfg(target_os = "macos")]
pub mod videotoolbox;
