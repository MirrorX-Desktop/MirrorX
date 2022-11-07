use crate::utility::format_device_id;

use super::UIState;
use mirrorx_core::{api::signaling::ResourceType, core_error, error::CoreResult};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PopupDialogInputRemotePasswordEvent {
    pub active_device_id: String,
    pub passive_device_id: String,
}

#[tauri::command]
#[tracing::instrument(skip(state, window))]
pub async fn signaling_visit_request(
    remote_device_id: String,
    state: tauri::State<'_, UIState>,
    window: tauri::Window,
) -> CoreResult<()> {
    let remote_device_id: i64 = remote_device_id.replace('-', "").parse()?;

    let domain = state.domain.lock().await;
    let domain = domain
        .as_ref()
        .ok_or_else(|| core_error!("current domain is empty"))?;

    let signaling_provider = state.signaling_client.lock().await;
    let signaling_provider = signaling_provider
        .as_ref()
        .ok_or_else(|| core_error!("current signaling provider is empty"))?;

    let resp = signaling_provider
        .visit(mirrorx_core::api::signaling::VisitRequest {
            local_device_id: domain.device_id,
            remote_device_id,
            resource_type: ResourceType::Desktop,
        })
        .await?;

    if resp.allow {
        if let Err(err) = window.emit(
            "popup_dialog_input_remote_password",
            PopupDialogInputRemotePasswordEvent {
                active_device_id: format_device_id(domain.device_id),
                passive_device_id: format_device_id(remote_device_id),
            },
        ) {
            tracing::error!(
                ?err,
                "window emit 'pop_dialog_input_remote_password' event failed"
            );
        }

        Ok(())
    } else {
        Err(core_error!("remote device reject your visit request"))
    }
}
