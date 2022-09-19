mod frame;
pub use frame::DecodedFrame;

#[cfg(target_os = "windows")]
mod ffmpeg;

#[cfg(target_os = "windows")]
pub use ffmpeg::Decoder;

#[cfg(target_os = "macos")]
mod videotoolbox;

#[cfg(target_os = "macos")]
pub use videotoolbox::Decoder;
