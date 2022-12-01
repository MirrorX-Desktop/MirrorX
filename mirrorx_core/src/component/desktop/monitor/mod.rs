use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::{get_active_monitors, get_primary_monitor_params};

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{get_active_monitors, get_primary_monitor_params, NSScreen};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Monitor {
    pub id: String,
    pub name: String,
    pub refresh_rate: u8,
    pub width: u16,
    pub height: u16,
    pub is_primary: bool,
    pub screen_shot: Option<Vec<u8>>,
    pub left: u16,
    pub top: u16,
}
