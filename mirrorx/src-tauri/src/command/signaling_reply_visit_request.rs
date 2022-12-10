use super::AppState;
use mirrorx_core::{core_error, error::CoreResult};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn signaling_reply_visit_request(
    allow: bool,
    active_device_id: String,
    passive_device_id: String,
    state: tauri::State<'_, AppState>,
) -> CoreResult<()> {
    let active_device_id: i64 = active_device_id.replace('-', "").parse()?;
    let passive_device_id: i64 = passive_device_id.replace('-', "").parse()?;

    let signaling_provider = state.signaling_client.lock().await;
    let signaling_provider = signaling_provider
        .as_ref()
        .ok_or_else(|| core_error!("current signaling provider is empty"))?;

    if let Err(err) = signaling_provider
        .visit_reply(mirrorx_core::api::signaling::VisitReplyRequest {
            active_device_id,
            passive_device_id,
            allow,
        })
        .await
    {
        tracing::error!(
            ?active_device_id,
            ?passive_device_id,
            ?allow,
            ?err,
            "signaling client reply visit request failed"
        );

        return Err(core_error!(
            "reply visit request failed, maybe remote device is offline or reply timeout"
        ));
    }

    Ok(())
}
