use super::UIState;
use mirrorx_core::{api::config::LocalStorage, core_error, error::CoreResult};

#[tauri::command]
#[tracing::instrument(skip(password, state))]
pub async fn set_current_domain_device_password(
    password: String,
    state: tauri::State<'_, UIState>,
) -> CoreResult<()> {
    let mut domain = state.domain.lock().await;
    let domain = domain
        .as_mut()
        .ok_or_else(|| core_error!("current domain is empty"))?;

    LocalStorage::current()?
        .domain()
        .set_domain_device_password(domain.id, &password)?;

    domain.password = password;

    Ok(())
}
