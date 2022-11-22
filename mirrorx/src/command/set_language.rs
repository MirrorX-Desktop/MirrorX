use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};
use serde::Serialize;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem};

#[derive(Serialize, Clone)]
struct UpdateLanguageEvent {
    pub language: String,
}

#[tauri::command]
#[tracing::instrument(skip(app))]
pub async fn set_language(app: AppHandle, language: String) -> CoreResult<()> {
    LocalStorage::current()?.kv().set_language(&language)?;
    app.emit_all(
        "update_language",
        UpdateLanguageEvent {
            language: language.clone(),
        },
    )
    .map_err(|err| {
        tracing::error!(?err, "emit event 'update_language' failed");
        core_error!("emit event 'update_language' failed")
    })?;

    let (quit_text, show_text, hide_text) = match language.as_str() {
        "en" => ("Quit", "Show", "Hide"),
        "zh" => ("退出", "显示", "隐藏"),
        _ => return Ok(()),
    };

    let quit = CustomMenuItem::new("quit", quit_text);
    let show = CustomMenuItem::new("show", show_text);
    let hide = CustomMenuItem::new("hide", hide_text);
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    if let Err(err) = app.tray_handle().set_menu(tray_menu) {
        tracing::error!(?err, "set new tray menu failed");
    }

    Ok(())
}
