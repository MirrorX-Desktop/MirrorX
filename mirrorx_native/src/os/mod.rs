#![allow(non_upper_case_globals)]

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::*;
