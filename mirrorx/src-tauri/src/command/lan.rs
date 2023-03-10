use crate::{window::create_desktop_window, FileTransferCache, LANService};
use mirrorx_core::{
    core_error,
    error::CoreResult,
    service::{
        endpoint::{self, EndPointID, EndPointStreamType},
        lan::service::Node,
    },
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tauri::{AppHandle, State};
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(app_handle, egui_plugin, file_transfer_cache))]
pub async fn lan_connect(
    app_handle: AppHandle,
    egui_plugin: State<'_, EguiPluginHandle>,
    file_transfer_cache: State<'_, FileTransferCache>,
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

    let endpoint_id = EndPointID::IP {
        local_ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        remote_ip,
    };

    if visit_desktop {
        let endpoint_service = endpoint::Service::new(
            endpoint_id,
            EndPointStreamType::ActiveTCP(remote_addr),
            None,
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
                        endpoint_service,
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
        let endpoint_service = endpoint::Service::new(
            endpoint_id,
            EndPointStreamType::ActiveTCP(remote_addr),
            None,
            None,
        )
        .await?;

        file_transfer_cache
            .0
            .insert(remote_ip.to_string(), endpoint_service)
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
            file_transfer_cache
                .0
                .invalidate(&remote_ip.to_string())
                .await;
            tracing::error!(?err, "create file manager window failed");
            return Err(core_error!("create remote file manager window failed"));
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(lan_service))]
pub async fn lan_nodes_list(lan_service: State<'_, LANService>) -> CoreResult<Vec<Node>> {
    Ok(lan_service.nodes().await)
}

#[tauri::command]
#[tracing::instrument(skip(lan_service))]
pub async fn lan_nodes_search(
    lan_service: State<'_, LANService>,
    keyword: String,
) -> CoreResult<Vec<Node>> {
    let mut nodes = lan_service.nodes().await;
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
}

#[tauri::command]
#[tracing::instrument(skip(lan_service))]
pub async fn lan_discoverable_set(
    lan_service: tauri::State<'_, LANService>,
    discoverable: bool,
) -> CoreResult<()> {
    lan_service.set_discoverable(discoverable);
    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(lan_service))]
pub async fn lan_discoverable_get(lan_service: tauri::State<'_, LANService>) -> CoreResult<bool> {
    Ok(lan_service.discoverable())
}
