#[cfg(target_os = "macos")]
mod desktop_duplicator_macos;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use desktop_duplicator_macos::DesktopDuplicator;

#[cfg(target_os = "windows")]
mod desktop_duplicator_windows;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use desktop_duplicator_windows::DesktopDuplicator;
