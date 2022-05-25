#[cfg(target_os = "macos")]
mod desktop_duplicator_macos;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use desktop_duplicator_macos::DesktopDuplicator;

#[cfg(target_os = "windows")]
mod windows;
