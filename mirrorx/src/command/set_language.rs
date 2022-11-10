use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};
use serde::Serialize;
use tauri::Manager;

#[derive(Serialize, Clone)]
struct UpdateLanguageEvent {
    pub language: String,
}

#[tauri::command]
#[tracing::instrument(skip(app))]
pub async fn set_language(language: String, app: tauri::AppHandle) -> CoreResult<()> {
    LocalStorage::current()?.kv().set_language(&language)?;
    app.emit_all("update_language", UpdateLanguageEvent { language })
        .map_err(|err| {
            tracing::error!(?err, "emit event 'update_language' failed");
            core_error!("emit event 'update_language' failed")
        })?;

    Ok(())
}
