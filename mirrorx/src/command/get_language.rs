use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};
use serde::Serialize;
use tauri::Manager;

#[tauri::command]
#[tracing::instrument]
pub async fn get_language() -> CoreResult<String> {
    Ok(LocalStorage::current()?
        .kv()
        .get_language()?
        .unwrap_or("en".into()))
}
