#[cfg_attr(target_os = "macos", path = "macos.rs")]
mod platform;

use crate::error::CoreResult;
use serde::{Deserialize, Serialize};

pub const RESIZE_FACTOR: f32 = 0.2f32;

#[derive(Serialize, Deserialize)]
pub struct Display {
    pub id: String,
    pub name: String,
    pub rect: Rect,
    pub refresh_rate: u8,
    pub screenshot: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rect {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
}

impl Display {
    pub fn enum_all_available_displays() -> CoreResult<Vec<Display>> {
        platform::enum_all_available_displays()
    }
}

#[test]
fn test_enum_all_available_displays() {
    let displays = Display::enum_all_available_displays().unwrap();
    for (i, display) in displays.iter().enumerate() {
        println!(
            "display#{i}, id:{}, name:{}, rect:{:?}, refresh_rate:{}, screenshot_size:{}",
            display.id,
            display.name,
            display.rect,
            display.refresh_rate,
            display.screenshot.as_ref().map_or(0, |data| data.len())
        );

        if let Some(ref s) = display.screenshot {
            let _ = std::fs::write(format!("/Users/chenbaiyu/{i}.png"), s);
        }
    }
}
