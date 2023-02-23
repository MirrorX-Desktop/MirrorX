#[cfg_attr(target_os = "macos", path = "macos.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
mod display_impl;

use crate::error::CoreResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Display {
    pub id: String,
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u16,
    // #[cfg(target_os = "windows")]
    // #[serde(skip)]
    // pub adapter: Option<windows::Win32::Graphics::Dxgi::IDXGIAdapter1>,

    // #[cfg(target_os = "windows")]
    // #[serde(skip)]
    // pub output: Option<windows::Win32::Graphics::Dxgi::IDXGIOutput>,
}

impl Display {
    pub fn enum_all_available_displays() -> CoreResult<Vec<Display>> {
        display_impl::enum_all_available_displays()
    }

    #[cfg(target_os = "windows")]
    pub fn query_display(
        display_id: &str,
    ) -> CoreResult<(
        Display,
        windows::Win32::Graphics::Dxgi::IDXGIAdapter1,
        windows::Win32::Graphics::Dxgi::IDXGIOutput,
    )> {
        display_impl::query_display(display_id)
    }
}

#[test]
fn test_enum_all_available_displays() {
    let displays = Display::enum_all_available_displays().unwrap();
    for (i, display) in displays.iter().enumerate() {
        println!(
            "display#{i}, id:{}, left:{}, top:{}, width:{}, height:{}",
            display.id, display.left, display.top, display.width, display.height,
        );
    }
}
