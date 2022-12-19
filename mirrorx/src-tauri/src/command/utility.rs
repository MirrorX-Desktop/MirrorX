use mirrorx_core::{error::CoreResult, utility::os::GraphicsCards};
use serde::Serialize;

#[tauri::command]
#[tracing::instrument]
pub fn utility_generate_random_password() -> String {
    mirrorx_core::utility::rand::generate_random_password()
}

#[tauri::command]
#[tracing::instrument]
pub fn utility_detect_os_platform() -> String {
    let info = os_info::get();
    let mut type_name = info.os_type().to_string();
    if os_info::Type::Windows == info.os_type() {
        if let Some(windows_edition) = info.edition() {
            type_name = windows_edition.to_string();
        }
    }
    let arch = info.bitness();
    let version = info.version();

    format!("{type_name} {arch} {version}")
}

#[tauri::command]
#[tracing::instrument]
pub fn utility_enum_graphics_cards() -> CoreResult<Vec<GraphicsCards>> {
    mirrorx_core::utility::os::enum_graphics_cards()
}

#[tauri::command]
#[tracing::instrument(skip(window))]
pub fn utility_hide_macos_zoom_button(window: tauri::Window) {
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::NSWindow;
        use cocoa::appkit::NSWindowButton;
        use objc::{msg_send, runtime::YES, sel, sel_impl};

        if let Ok(ns_window) = window.ns_window() {
            unsafe {
                let id = ns_window as cocoa::base::id;
                let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
                let _: () = msg_send![zoom_button, setHidden: YES];
            }
        }
    }
}
