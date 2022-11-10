use mirrorx_core::{api::config::LocalStorage, error::CoreResult};

#[tauri::command]
#[tracing::instrument]
pub async fn get_language() -> CoreResult<String> {
    Ok(LocalStorage::current()?
        .kv()
        .get_language()?
        .unwrap_or_else(|| "en".into()))
}
