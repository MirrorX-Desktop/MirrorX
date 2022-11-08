use mirrorx_core::api::config::LocalStorage;
use mirrorx_core::error::CoreResult;
use serde::Serialize;

use crate::utility::format_device_id;

use super::UIState;

#[derive(Serialize)]
pub struct Response {
    pub total: u32,
    pub current_domain_name: String,
    pub domains: Vec<DomainModel>,
}

#[derive(Serialize)]
pub struct DomainModel {
    pub id: i64,
    pub name: String,
    pub addr: String,
    pub device_id: String,
    pub remarks: String,
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_domains(page: u32, state: tauri::State<'_, UIState>) -> CoreResult<Response> {
    let storage = LocalStorage::current()?;
    let (total, domains) = storage.domain().get_domains(page)?;

    let domains: Vec<DomainModel> = domains
        .iter()
        .map(|entity| DomainModel {
            id: entity.id,
            name: entity.name.to_owned(),
            addr: entity.addr.to_owned(),
            device_id: format_device_id(entity.device_id),
            remarks: entity.remarks.to_owned(),
        })
        .collect();

    let current_domain_name = state
        .domain
        .lock()
        .await
        .as_ref()
        .map(|domain| domain.name.to_owned())
        .unwrap_or_default();

    Ok(Response {
        total,
        current_domain_name,
        domains,
    })
}
