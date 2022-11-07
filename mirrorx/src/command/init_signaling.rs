use super::UIState;
use crate::utility::format_device_id;
use mirrorx_core::{
    api::{
        config::LocalStorage,
        signaling::{PublishMessage, ResourceType, SignalingProvider},
    },
    core_error,
    error::CoreResult,
};
use serde::Serialize;
use tokio::sync::mpsc::Receiver;

#[derive(Debug, Clone, Serialize)]
pub struct PopupDialogVisitRequestEvent {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub resource_type: String,
}

#[tauri::command]
#[tracing::instrument(skip(state, window))]
pub async fn init_signaling(
    force: bool,
    state: tauri::State<'_, UIState>,
    window: tauri::Window,
) -> CoreResult<()> {
    if state.signaling_client.lock().await.is_some() && !force {
        return Ok(());
    }

    let mut domain = state.domain.lock().await;
    let domain = domain
        .as_mut()
        .ok_or_else(|| core_error!("current domain is empty"))?;

    let (publish_message_tx, publish_message_rx) = tokio::sync::mpsc::channel(8);
    let (signaling_provider, should_update_domain_device_id) =
        SignalingProvider::dial(domain, publish_message_tx).await?;

    if should_update_domain_device_id {
        LocalStorage::current()?
            .domain()
            .set_domain_device_id(domain.id, domain.device_id)?;
    }

    *state.signaling_client.lock().await = Some(signaling_provider);

    start_signaling_publish_event_handle(publish_message_rx, window);

    Ok(())
}

fn start_signaling_publish_event_handle(
    mut publish_message_rx: Receiver<PublishMessage>,
    window: tauri::Window,
) {
    tokio::spawn(async move {
        loop {
            match publish_message_rx.recv().await {
                Some(publish_message) => match publish_message {
                    mirrorx_core::api::signaling::PublishMessage::VisitRequest {
                        active_device_id,
                        passive_device_id,
                        resource_type,
                    } => {
                        if let Err(err) = window.emit(
                            "popup_dialog_visit_request",
                            PopupDialogVisitRequestEvent {
                                active_device_id: format_device_id(active_device_id),
                                passive_device_id: format_device_id(passive_device_id),
                                resource_type: if let ResourceType::Desktop = resource_type {
                                    "desktop"
                                } else {
                                    "files"
                                }
                                .into(),
                            },
                        ) {
                            tracing::error!(?err, "window emit 'pop_dialog_visit_request' failed");
                        }
                    }
                },
                None => {
                    tracing::error!("publish message channel is closed");
                    return;
                }
            }
        }
    });
}
