#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod platform;
mod utility;
mod window;

use tauri::Manager;

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(command::UIState::new())
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));

            if let Some(win) = app.get_window("main") {
                #[cfg(target_os = "macos")]
                {
                    use platform::window_ext::WindowExt;
                    win.expand_title_bar();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::init_config,
            command::init_signaling,
            command::get_current_domain,
            command::generate_random_password,
            command::set_current_domain_device_password,
            command::signaling_visit_request,
            command::signaling_reply_visit_request,
            command::signaling_key_exchange,
            command::get_domains,
            command::add_domain,
            command::delete_domain,
            command::switch_primary_domain,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
