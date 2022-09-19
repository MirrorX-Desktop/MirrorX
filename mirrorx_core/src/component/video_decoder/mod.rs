mod frame;
pub use frame::DecodedFrame;

#[cfg(target_os = "windows")]
mod video_decoder;

#[cfg(target_os = "windows")]
pub use video_decoder::VideoDecoder;

#[cfg(target_os = "macos")]
mod videotoolbox;

#[cfg(target_os = "macos")]
pub use videotoolbox::Decoder;
