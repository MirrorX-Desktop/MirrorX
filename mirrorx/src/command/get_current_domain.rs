use super::UIState;
use crate::utility::format_device_id;
use mirrorx_core::{core_error, error::CoreResult};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub name: String,
    pub device_id: String,
    pub password: String,
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_current_domain(state: tauri::State<'_, UIState>) -> CoreResult<Response> {
    let domain = state.domain.lock().await;
    let domain = domain
        .as_ref()
        .ok_or_else(|| core_error!("current domain is empty"))?;

    Ok(Response {
        name: domain.name.to_owned(),
        device_id: format_device_id(domain.device_id),
        password: domain.password.to_owned(),
    })
}
