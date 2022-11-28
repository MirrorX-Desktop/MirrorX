use super::UIState;
use mirrorx_core::{component::lan_discover::LanDiscover, error::CoreResult};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn init_lan_discover(state: tauri::State<'_, UIState>) -> CoreResult<()> {
    *state.lan_discover.lock().await = Some(LanDiscover::new(0, 0).await?);
    Ok(())
}
