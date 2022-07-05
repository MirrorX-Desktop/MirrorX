#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::monitor::*;
#[cfg(target_os = "macos")]
pub use macos::ns_screen::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::display::*;

mod monitor;
pub use monitor::Monitor;
