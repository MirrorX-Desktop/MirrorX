mod frame;

#[cfg(target_os = "windows")]
mod video_decoder;

pub use frame::DecodedFrame;

#[cfg(target_os = "windows")]
pub use video_decoder::VideoDecoder;

pub mod videotoolbox;
