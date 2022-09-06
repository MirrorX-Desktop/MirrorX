mod frame;

#[cfg(target_os = "windows")]
mod video_decoder;

pub use frame::DecodedFrame;

#[cfg(target_os = "windows")]
pub use video_decoder::VideoDecoder;

#[cfg(target_os = "macos")]
pub mod videotoolbox;
