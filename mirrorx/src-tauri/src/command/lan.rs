use crate::{command::AppState, window::create_desktop_window};
use mirrorx_core::{
    api::endpoint::{
        create_desktop_active_endpoint_client, create_file_manager_active_endpoint_client,
        id::EndPointID, EndPointStream,
    },
    component::lan::{LANProvider, Node},
    core_error,
    error::CoreResult,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_init(app_state: tauri::State<'_, AppState>, force: bool) -> CoreResult<()> {
    let mut lan_provider = app_state.lan_provider.lock().await;

    if force || lan_provider.is_none() {
        *lan_provider = Some(LANProvider::new().await?);
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
    let remote_ip: IpAddr = addr
        .parse()
        .map_err(|_| core_error!("parse addr to IpAddr failed"))?;

    let window_label = if visit_desktop {
        format!("Desktop:{}", remote_ip.to_string().replace('.', "_"))
    } else {
        format!("FileManager:{}", remote_ip.to_string().replace('.', "_"))
    };

    let window_title = if visit_desktop {
        format!("MirrorX {remote_ip}")
    } else {
        format!("MirrorX File Transfer {remote_ip}")
    };

    let remote_addr = SocketAddr::new(remote_ip, 48001);

    let endpoint_id = EndPointID::LANID {
        local_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
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
    if let Some(ref discover) = *app_state.lan_provider.lock().await {
        Ok(discover.nodes().await)
    } else {
        Err(core_error!("lan discover is empty"))
    }
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_nodes_search(
    app_state: tauri::State<'_, AppState>,
    keyword: String,
) -> CoreResult<Vec<Node>> {
    if let Some(ref discover) = *app_state.lan_provider.lock().await {
        let mut nodes = discover.nodes().await;
        let nodes_count = nodes.len();

        for i in 0..nodes_count {
            if !nodes[i].display_name.contains(&keyword) {
                nodes.remove(i);
            }

            for ip in nodes[i].addrs.keys() {
                if !ip.to_string().contains(&keyword) {
                    nodes.remove(i);
                    break;
                }
            }
        }

        Ok(nodes)
    } else {
        Err(core_error!("lan discover is empty"))
    }
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_discoverable_set(
    app_state: tauri::State<'_, AppState>,
    discoverable: bool,
) -> CoreResult<()> {
    if let Some(ref discover) = *app_state.lan_provider.lock().await {
        discover.set_discoverable(discoverable);
        Ok(())
    } else {
        Err(core_error!("lan discover is empty"))
    }
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn lan_discoverable_get(app_state: tauri::State<'_, AppState>) -> CoreResult<bool> {
    if let Some(ref discover) = *app_state.lan_provider.lock().await {
        Ok(discover.discoverable())
    } else {
        Err(core_error!("lan discover is empty"))
    }
}
