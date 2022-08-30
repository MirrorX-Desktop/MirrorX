mod ffmpeg_encoder_config;

#[cfg(not(target_os = "macos"))]
mod ffmpeg;

#[cfg(not(target_os = "macos"))]
pub use ffmpeg::Encoder;

#[cfg(target_os = "macos")]
mod videotoolbox;

#[cfg(target_os = "macos")]
pub use videotoolbox::Encoder;