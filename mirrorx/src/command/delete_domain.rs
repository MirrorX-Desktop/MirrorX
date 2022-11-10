use mirrorx_core::{api::config::LocalStorage, error::CoreResult};

#[tauri::command]
#[tracing::instrument]
pub async fn delete_domain(id: i64) -> CoreResult<()> {
    let storage = LocalStorage::current()?;
    storage.domain().delete_domain(id)
}
