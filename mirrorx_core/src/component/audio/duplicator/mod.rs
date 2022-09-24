#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::Duplicator;

#[cfg(target_os = "macos")]
mod macos;
