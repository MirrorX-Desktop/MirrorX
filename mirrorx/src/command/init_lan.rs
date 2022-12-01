use super::UIState;
use mirrorx_core::{
    component::lan::{discover::Discover, server::Server},
    error::CoreResult,
    utility::lan_ip::get_lan_ip,
};

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn init_lan(state: tauri::State<'_, UIState>) -> CoreResult<()> {
    let lan_ip = get_lan_ip().await?;
    *state.lan_discover.lock().await = Some(Discover::new(lan_ip).await?);
    *state.lan_server.lock().await = Some(Server::new(lan_ip).await?);
    Ok(())
}
