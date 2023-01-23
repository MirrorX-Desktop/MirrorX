use crate::{command::AppState, window::create_desktop_window};
use mirrorx_core::{
    api::endpoint::{
        create_desktop_active_endpoint_client, create_file_manager_active_endpoint_client,
        id::EndPointID, EndPointStream,
    },
    component::lan::{
        discover::{Discover, Node},
        server::Server,
    },
    core_error,
    error::CoreResult,
    utility::lan_ip::get_lan_ip,
};
use std::net::{IpAddr, SocketAddr};
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_init(app_state: tauri::State<'_, AppState>, force: bool) -> CoreResult<()> {
    let mut lan_components = app_state.lan_components.lock().await;

    if force || lan_components.is_none() {
        let lan_ip = get_lan_ip().await?;
        let discover = Discover::new(lan_ip).await?;

        let old_components = lan_components.take();
        drop(old_components);

        *lan_components = Some((discover, Server::new(lan_ip).await?));
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, app_state, egui_plugin))]
pub async fn lan_connect(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
    addr: String,
    visit_desktop: bool,
) -> CoreResult<()> {
    let local_ip = get_lan_ip().await?;
    let remote_ip: IpAddr = addr
        .parse()
        .map_err(|_| core_error!("parse addr to IpAddr failed"))?;

    let window_label = if visit_desktop {
        format!("Desktop:{}", remote_ip.to_string().replace('.', "_"))
    } else {
        format!("FileManager:{}", remote_ip.to_string().replace('.', "_"))
    };

    let window_title = if visit_desktop {
        format!("MirrorX {}", remote_ip)
    } else {
        format!("MirrorX File Transfer {}", remote_ip)
    };

    let remote_addr = SocketAddr::new(remote_ip, 48001);

    let endpoint_id = EndPointID::LANID {
        local_ip,
        remote_ip,
    };

    if visit_desktop {
        let (client, render_frame_rx) = create_desktop_active_endpoint_client(
            endpoint_id,
            None,
            EndPointStream::ActiveTCP(remote_addr),
            None,
        )
        .await?;

        if let Err(err) = egui_plugin.create_window(
            window_label.clone(),
            Box::new(move |cc| {
                if let Some(gl_context) = cc.gl.as_ref() {
                    Box::new(create_desktop_window(
                        cc,
                        gl_context.clone(),
                        endpoint_id,
                        client,
                        render_frame_rx,
                    ))
                } else {
                    panic!("get gl context failed");
                }
            }),
            window_label,
            tauri_egui::eframe::NativeOptions {
                // hardware_acceleration: HardwareAcceleration::Required,
                ..Default::default()
            },
        ) {
            tracing::error!(?err, "create desktop window failed");
            return Err(core_error!("create remote desktop window failed"));
        }
    } else {
        let client = create_file_manager_active_endpoint_client(
            endpoint_id,
            None,
            EndPointStream::ActiveTCP(remote_addr),
            None,
        )
        .await?;

        app_state
            .files_endpoints
            .lock()
            .await
            .insert(remote_ip.to_string(), client)
            .await;

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            if let Err(err) = tauri::WindowBuilder::new(
                &app_handle,
                window_label,
                tauri::WindowUrl::App(format!("/files?device_id={remote_ip}").into()),
            )
            .center()
            .inner_size(960., 680.)
            .min_inner_size(960., 680.)
            .title(window_title)
            .build()
            {
                let _ = tx.send(Some(err));
            } else {
                let _ = tx.send(None);
            }
        });

        let create_result = rx.await.map_err(|_| core_error!("create window failed"))?;

        if let Some(err) = create_result {
            app_state
                .files_endpoints
                .lock()
                .await
                .invalidate(&remote_ip.to_string())
                .await;
            tracing::error!(?err, "create file manager window failed");
            return Err(core_error!("create remote file manager window failed"));
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_nodes_list(app_state: tauri::State<'_, AppState>) -> CoreResult<Vec<Node>> {
    let Some((ref discover, _)) = *app_state
        .lan_components
        .lock()
        .await else {
            return Err(core_error!("lan discover is empty"))
        };

    Ok(discover.nodes_snapshot())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_nodes_search(
    app_state: tauri::State<'_, AppState>,
    keyword: String,
) -> CoreResult<Vec<Node>> {
    let Some((ref discover, _)) = *app_state
        .lan_components
        .lock()
        .await else {
            return Err(core_error!("lan discover is empty"))
        };

    let mut nodes = discover.nodes_snapshot();
    let nodes_count = nodes.len();

    for i in 0..nodes_count {
        let hostname = &nodes[i].host_name;
        let ip = nodes[i].addr.to_string();
        if !(hostname.contains(&keyword) || ip.contains(&keyword)) {
            nodes.remove(i);
        }
    }

    Ok(nodes)
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_discoverable_set(
    app_state: tauri::State<'_, AppState>,
    discoverable: bool,
) -> CoreResult<()> {
    let Some((ref discover, _)) = *app_state
        .lan_components
        .lock()
        .await else {
            return Err(core_error!("lan discover is empty"))
        };

    discover.set_discoverable(discoverable);
    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_discoverable_get(app_state: tauri::State<'_, AppState>) -> CoreResult<bool> {
    let Some((ref discover, _)) = *app_state
        .lan_components
        .lock()
        .await else {
            return Err(core_error!("lan discover is empty"))
        };

    Ok(discover.discoverable())
}
