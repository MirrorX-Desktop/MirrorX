#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod api;
mod event;
mod platform;
mod utility;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(api::UIState::default())
        .setup(|app| {
            if let Some(win) = app.get_window("main") {
                #[cfg(target_os = "macos")]
                {
                    use platform::window_ext::WindowExt;
                    win.expand_title_bar();
                }
            }

            Ok(())
        })
        // invoke_handler should be called only once!
        .invoke_handler(tauri::generate_handler![
            api::init_config,
            api::init_signaling_client,
            api::get_config_primary_domain,
            api::get_config_device_id,
            api::get_config_device_password,
            api::generate_random_password,
            api::set_config_device_password,
            api::signaling_visit_request,
            api::signaling_reply_visit_request,
            api::signaling_key_exchange
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
