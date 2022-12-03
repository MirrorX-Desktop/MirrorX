use super::UIState;
use mirrorx_core::{
    component::lan::{discover::Discover, server::Server},
    error::CoreResult,
    utility::lan_ip::get_lan_ip,
};

#[tauri::command]
#[tracing::instrument(skip(state, window))]
pub async fn init_lan(
    force: bool,
    state: tauri::State<'_, UIState>,
    window: tauri::Window,
) -> CoreResult<()> {
    let mut current_discover = state.lan_discover.lock().await;
    let mut current_lan_server = state.lan_server.lock().await;

    if force || (current_discover.is_none() && current_lan_server.is_none()) {
        let lan_ip = get_lan_ip().await?;
        let (discover, mut event_rx) = Discover::new(lan_ip).await?;
        tokio::spawn(async move {
            loop {
                match event_rx.recv().await {
                    Some(_) => {
                        let _ = window.emit("update_lan_discover_nodes", ());
                    }
                    None => {
                        tracing::info!("discover notify process exit");
                        return;
                    }
                }
            }
        });

        let old_discover = current_discover.take();
        let old_lan_server = current_lan_server.take();

        drop(old_discover);
        drop(old_lan_server);

        *current_discover = Some(discover);
        *current_lan_server = Some(Server::new(lan_ip).await?);
    }

    Ok(())
}
