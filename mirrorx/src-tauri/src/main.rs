#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod command;
mod utility;
mod window;

#[cfg(target_os = "macos")]
use tauri::Icon;

use tauri::{App, Manager, SystemTray, SystemTrayEvent, WindowEvent};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[cfg(target_os = "macos")]
static TRAY_ICON_MACOS: &[u8] = include_bytes!("../assets/icons/tray-macOS.png");

#[tokio::main]
#[tracing::instrument]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let app = build_app();

    let log_dir = app
        .path_resolver()
        .app_log_dir()
        .expect("get app log dir failed")
        .join("logs");

    let file_appender = tracing_appender::rolling::daily(&log_dir, "mirrorx.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    let console_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stderr);

    tracing_subscriber::Registry::default()
        .with(EnvFilter::from("info,tao=info"))
        .with(console_layer)
        .with(file_layer)
        .init();

    tracing::info!(path = ?log_dir, "log dir");

    app.run(|app_handle, event| match event {
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

fn build_app() -> App {
    let tray = SystemTray::new();
    #[cfg(target_os = "macos")]
    let tray = tray
        .with_icon(Icon::Raw(TRAY_ICON_MACOS.to_vec()))
        .with_icon_as_template(true);

    tauri::Builder::default()
        .manage(command::AppState::new())
        .system_tray(tray)
        .enable_macos_default_menu(false)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::DoubleClick { .. } = event {
                app.windows().values().for_each(|window| {
                    let _ = window.show();
                    let _ = window.unminimize();
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
                    "about" => {
                        let _ = app.emit_all("/dialog/about", ());
                    }
                    _ => {}
                }
            }
        })
        .on_menu_event(|event| {
            if event.menu_item_id() == "about" {
                let _ = event.window().emit("/dialog/about", ());
            }

            if event.menu_item_id() == "quit" {
                std::process::exit(0)
            }
        })
        .setup(|app| {
            app.wry_plugin(tauri_egui::EguiPluginBuilder::new(app.handle()));
            let app_name = app.package_info().name.clone();

            let handle = app.handle();
            std::thread::spawn(move || {
                let builder = tauri::WindowBuilder::new(
                    &handle,
                    "main",
                    tauri::WindowUrl::App("/home".into()),
                )
                .center()
                .always_on_top(true)
                .title(&app_name)
                .fullscreen(false)
                .resizable(false)
                .maximized(false)
                .inner_size(360., 640.);

                #[cfg(target_os = "macos")]
                {
                    use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

                    let mut menu = Menu::new();
                    menu = menu.add_submenu(Submenu::new(
                        app_name.clone(),
                        Menu::new()
                            .add_item(CustomMenuItem::new("about", "About MirrorX"))
                            .add_native_item(MenuItem::Separator)
                            .add_item(CustomMenuItem::new("quit", "Quit")),
                    ));

                    builder
                        .menu(menu)
                        .hidden_title(true)
                        .title_bar_style(tauri::TitleBarStyle::Overlay)
                        .build()
                        .unwrap();
                }

                #[cfg(target_os = "windows")]
                {
                    builder
                        .decorations(false)
                        .transparent(true)
                        .build()
                        .unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::config::config_init,
            command::config::config_domain_get,
            command::config::config_domain_get_id_and_names,
            command::config::config_domain_create,
            command::config::config_domain_delete,
            command::config::config_domain_list,
            command::config::config_domain_update,
            command::config::config_language_get,
            command::config::config_language_set,
            command::config::config_theme_get,
            command::config::config_theme_set,
            command::lan::lan_init,
            command::lan::lan_connect,
            command::lan::lan_nodes_list,
            command::lan::lan_nodes_search,
            command::lan::lan_discoverable_get,
            command::lan::lan_discoverable_set,
            command::signaling::signaling_connect,
            command::signaling::signaling_visit,
            command::utility::utility_generate_random_password,
            command::utility::utility_detect_os_platform,
            command::utility::utility_enum_graphics_cards,
            command::utility::utility_hide_macos_zoom_button,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}
