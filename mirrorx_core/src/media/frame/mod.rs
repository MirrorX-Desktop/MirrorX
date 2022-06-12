#[cfg(target_os = "macos")]
mod capture_frame_macos;

#[cfg(target_os="macos")]
pub use capture_frame_macos::CaptureFrame;

#[cfg(target_os="windows")]
mod capture_frame_windows;

#[cfg(target_os="windows")]
pub use capture_frame_windows::CaptureFrame;