use super::AppState;
use crate::window::create_desktop_window;
use mirrorx_core::{
    api::{
        endpoint::id::EndPointID,
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
#[tracing::instrument(skip(app_state, egui_plugin))]
pub async fn signaling_visit(
    app_state: tauri::State<'_, AppState>,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
    remote_device_id: String,
    password: String,
) -> CoreResult<()> {
    let window_label = format!("MirrorX {}", remote_device_id);
    let remote_device_id: i64 = remote_device_id.replace('-', "").parse()?;

    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let Some((_,ref signaling_client)) = *app_state.signaling_client.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let primary_domain = storage.domain().get_primary_domain()?;
    let local_device_id = primary_domain.device_id;
    let resp = signaling_client
        .visit(primary_domain.device_id, remote_device_id, password)
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

    if let Err(err) = egui_plugin.create_window(
        window_label.clone(),
        Box::new(move |cc| {
            if let Some(gl_context) = cc.gl.as_ref() {
                Box::new(create_desktop_window(
                    cc,
                    gl_context.clone(),
                    EndPointID::DeviceID {
                        local_device_id,
                        remote_device_id,
                    },
                    Some((opening_key, sealing_key)),
                    Some(visit_credentials),
                    endpoint_addr,
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

    let _ = storage
        .history()
        .create(remote_device_id, &primary_domain.name);

    Ok(())
}
