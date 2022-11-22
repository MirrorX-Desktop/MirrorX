#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod platform;
mod utility;
mod window;

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(command::UIState::new())
        .system_tray(system_tray)
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
            command::set_domain_remarks,
            command::set_language,
            command::get_language,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
