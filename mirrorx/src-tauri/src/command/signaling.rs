use super::AppState;
use crate::window::create_desktop_window;
use mirrorx_core::{
    api::{
        endpoint::{
            create_desktop_active_endpoint_client, create_file_manager_active_endpoint_client,
            id::EndPointID, EndPointStream,
        },
        signaling::{http_message::Response, SignalingClient},
    },
    core_error,
    error::CoreResult,
};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use tauri::http::Uri;
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn signaling_connect(
    app_state: tauri::State<'_, AppState>,
    force: bool,
) -> CoreResult<()> {
    let mut current_signaling = app_state.signaling_client.lock().await;

    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let primary_domain = storage.domain().get_primary_domain()?;

    if let Some((current_domain_id, _)) = *current_signaling {
        if current_domain_id == primary_domain.id && !force {
            return Ok(());
        }
    }

    let addrs: Vec<SocketAddr> = if let Ok(ipv4_addr) = primary_domain.addr.parse::<Ipv4Addr>() {
        vec![(ipv4_addr, primary_domain.subscribe_port).into()]
    } else if let Ok(ipv6_addr) = primary_domain.addr.parse::<Ipv6Addr>() {
        vec![(ipv6_addr, primary_domain.subscribe_port).into()]
    } else if let Ok(url_addr) = primary_domain.addr.parse::<Uri>() {
        if let Some(host) = url_addr.host() {
            let host = host.to_string();
            let (tx, rx) = tokio::sync::oneshot::channel();
            tokio::task::spawn_blocking(move || {
                match (host, primary_domain.subscribe_port).to_socket_addrs() {
                    Ok(addrs) => {
                        let addrs: Vec<SocketAddr> = addrs.collect();
                        let _ = tx.send(Some(addrs));
                    }
                    Err(_) => {
                        let _ = tx.send(None);
                    }
                };
            });

            match rx.await {
                Ok(addrs) => match addrs {
                    Some(addrs) => addrs,
                    None => {
                        return Err(core_error!("resolve empty socket addr"));
                    }
                },
                Err(_) => {
                    return Err(core_error!(
                        "receive addr resolve result failed, this shouldn't happen"
                    ));
                }
            }
        } else {
            return Err(core_error!("invalid domain addr"));
        }
    } else {
        return Err(core_error!("invalid domain addr"));
    };

    let mut client = SignalingClient::new(primary_domain.addr)?;

    client
        .subscribe(
            addrs,
            primary_domain.device_id,
            &primary_domain.finger_print,
            storage.clone(),
        )
        .await?;

    *current_signaling = Some((primary_domain.id, client));

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, app_state, egui_plugin, password))]
pub async fn signaling_visit(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
    remote_device_id: String,
    password: String,
    visit_desktop: bool,
) -> CoreResult<()> {
    let window_label = if visit_desktop {
        format!("Desktop:{remote_device_id}")
    } else {
        format!("FileManager:{remote_device_id}")
    };

    let window_title = if visit_desktop {
        format!("MirrorX {remote_device_id}")
    } else {
        format!("MirrorX File Transfer {remote_device_id}")
    };

    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let Some((_,ref signaling_client)) = *app_state.signaling_client.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let remote_device_id_num = remote_device_id.replace('-', "").parse()?;
    let primary_domain = storage.domain().get_primary_domain()?;
    let local_device_id = primary_domain.device_id;
    let resp = signaling_client
        .visit(
            primary_domain.device_id,
            remote_device_id_num,
            password,
            visit_desktop,
        )
        .await?;

    let (endpoint_addr, visit_credentials, opening_key, sealing_key) = match resp {
        Response::Message(result) => match result {
            Ok(v) => v,
            Err(reason) => return Err(core_error!("Visit Failed ({:?})", reason)),
        },
        Response::Error(err) => return Err(core_error!("Visit Failed ({:?})", err)),
    };

    let endpoint_addr: SocketAddr = endpoint_addr
        .parse()
        .map_err(|_| core_error!("parse endpoint addr failed"))?;

    tracing::info!(?local_device_id, ?remote_device_id, "key exchange success");

    let endpoint_id = EndPointID::DeviceID {
        local_device_id,
        remote_device_id: remote_device_id_num,
    };

    if visit_desktop {
        let (client, render_frame_rx) = create_desktop_active_endpoint_client(
            endpoint_id,
            Some((opening_key, sealing_key)),
            EndPointStream::ActiveTCP(endpoint_addr),
            Some(visit_credentials),
        )
        .await?;

        if let Err(err) = egui_plugin.create_window(
            window_label,
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
            window_title,
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
            Some((opening_key, sealing_key)),
            EndPointStream::ActiveTCP(endpoint_addr),
            Some(visit_credentials),
        )
        .await?;

        app_state
            .files_endpoints
            .lock()
            .await
            .insert(remote_device_id.clone(), client)
            .await;

        let (tx, rx) = tokio::sync::oneshot::channel();

        let device_id = remote_device_id.clone();
        tokio::spawn(async move {
            if let Err(err) = tauri::WindowBuilder::new(
                &app_handle,
                window_label,
                tauri::WindowUrl::App(format!("/files?device_id={device_id}").into()),
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
                .invalidate(&remote_device_id)
                .await;
            tracing::error!(?err, "create file manager window failed");
            return Err(core_error!("create remote file manager window failed"));
        }
    }

    let _ = storage
        .history()
        .create(remote_device_id_num, &primary_domain.name);

    Ok(())
}
