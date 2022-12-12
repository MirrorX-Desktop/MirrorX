#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod platform;
mod utility;
mod window;

use tauri::{Icon, Manager, SystemTray, SystemTrayEvent, WindowEvent};

static TRAY_ICON_MACOS: &[u8] = include_bytes!("../assets/icons/tray-macOS.png");

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tracing_subscriber::fmt::init();
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(command::AppState::new())
        .system_tray(if cfg!(target_os = "macos") {
            SystemTray::new()
                .with_icon(Icon::Raw(TRAY_ICON_MACOS.to_vec()))
                .with_icon_as_template(true)
        } else {
            SystemTray::new()
        })
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::DoubleClick { .. } = event {
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
            command::config::config_init,
            command::config::config_domain_get,
            command::config::config_domain_create,
            command::config::config_domain_delete,
            command::config::config_domain_list,
            command::config::config_domain_update,
            command::config::config_language_get,
            command::config::config_language_set,
            command::lan::lan_init,
            command::lan::lan_connect,
            command::lan::lan_nodes_list,
            command::signaling::signaling_connect,
            command::signaling::signaling_visit,
            command::utility::utility_generate_random_password,
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
