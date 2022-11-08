use super::UIState;
use mirrorx_core::{api::config::LocalStorage, error::CoreResult};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn switch_primary_domain(id: i64, state: tauri::State<'_, UIState>) -> CoreResult<()> {
    let mut domain = state.domain.lock().await;
    if let Some(domain) = domain.as_ref() {
        if domain.id == id {
            return Ok(());
        }
    }

    let storage = LocalStorage::current()?;
    storage.domain().set_domain_is_primary(id)?;
    let new_domain = storage.domain().get_domain_by_id(id)?;
    *domain = Some(new_domain);

    Ok(())
}
