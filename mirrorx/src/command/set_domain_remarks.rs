use mirrorx_core::{api::config::LocalStorage, error::CoreResult};

#[tauri::command]
#[tracing::instrument]
pub async fn set_domain_remarks(id: i64, remarks: String) -> CoreResult<()> {
    LocalStorage::current()?
        .domain()
        .set_domain_remarks(id, &remarks)
}
