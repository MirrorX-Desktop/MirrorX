use super::UIState;
use mirrorx_core::{component::lan::Node, core_error, error::CoreResult};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_lan_discover_nodes(state: tauri::State<'_, UIState>) -> CoreResult<Vec<Node>> {
    let res = state
        .lan_discover
        .lock()
        .await
        .as_ref()
        .ok_or(core_error!("lan discover is empty"))?
        .nodes_snapshot()
        .await;

    tracing::info!(?res, "nodes");

    Ok(res)
}
