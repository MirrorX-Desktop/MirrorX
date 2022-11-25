#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod platform;
mod utility;
mod window;

use tauri::{Manager, SystemTray, SystemTrayEvent, WindowEvent};

#[tracing::instrument]
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(command::UIState::new())
        .system_tray(SystemTray::new())
        .on_system_tray_event(|app, event| {
           if let SystemTrayEvent::DoubleClick { position: _, size: _, ..} = event {
                app.windows().values().for_each(|window| {
                    let _ = window.show();
                })
            }
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => std::process::exit(0),
                    "show" => app.windows().values().for_each(|window| {
                        let _ = window.show();
                    }),
                    "hide" => app.windows().values().for_each(|window| {
                        let _ = window.hide();
                    }),
                    _ => {}
                }
            }
        })
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
        .run(|app_handle, event| match event {
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if label == "main" {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        if let Some(window) = app_handle.get_window(&label) {
                            let _ = window.hide();
                            api.prevent_close();
                        }
                    }
                }
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
