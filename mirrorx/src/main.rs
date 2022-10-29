#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod platform;

fn main() {
    tauri::Builder::default()
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
