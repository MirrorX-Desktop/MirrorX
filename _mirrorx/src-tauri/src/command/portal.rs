use crate::{window::create_desktop_window, ConfigService, FileTransferCache, PortalService};
use mirrorx_core::{
    core_error,
    error::CoreResult,
    service::{
        endpoint::{self, EndPointID, EndPointStreamType},
        portal,
    },
};
use tauri::{AppHandle, State};
use tauri_egui::EguiPluginHandle;

#[tauri::command]
#[tracing::instrument(skip(app_handle, portal, config))]
pub async fn portal_switch(
    app_handle: AppHandle,
    portal: State<'_, PortalService>,
    config: State<'_, ConfigService>,
    force: bool,
) -> CoreResult<()> {
    let primary_domain = config.domain().get_primary_domain()?;

    let mut portal = portal.0.lock().await;
    if portal.domain_id() == primary_domain.id && !force {
        return Ok(());
    }

    let mut new_portal = portal::service::Service::new(config.inner().clone());

    new_portal
        .connect(primary_domain.id, primary_domain.addr, |_, _, _| -> bool {
            true
        })
        .await?;

    let reply = new_portal.get_server_config().await?;

    let server_require_version = semver::Version::parse(&reply.min_client_version)
        .map_err(|_| core_error!("parse portal server version requirement failed"))?;

    let client_version = app_handle
        .config()
        .package
        .version
        .clone()
        .unwrap_or(String::from("0.0.1"));

    let client_version = semver::Version::parse(&client_version)
        .map_err(|_| core_error!("parse client version failed"))?;

    if client_version < server_require_version {
        return Err(core_error!(
            "your clint version is lower than portal server requirement"
        ));
    }

    let update_device_id = primary_domain.device_id == 0;

    let reply = new_portal
        .client_register(primary_domain.device_id, &primary_domain.finger_print)
        .await?;

    if update_device_id {
        config
            .domain()
            .set_domain_device_id(primary_domain.id, reply.device_id)?;
        new_portal.set_domain_id(reply.device_id);
    }

    *portal = new_portal;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
#[tracing::instrument(skip(
    app_handle,
    egui_plugin,
    portal,
    config,
    file_transfer_cache,
    password
))]
pub async fn portal_visit(
    app_handle: tauri::AppHandle,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
    portal: tauri::State<'_, PortalService>,
    config: tauri::State<'_, ConfigService>,
    file_transfer_cache: tauri::State<'_, FileTransferCache>,
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

    let remote_device_id_num = remote_device_id.replace('-', "").parse()?;
    let primary_domain = config.domain().get_primary_domain()?;
    let local_device_id = primary_domain.device_id;
    let (endpoint_addr, visit_credentials, opening_key, sealing_key) = portal
        .0
        .lock()
        .await
        .visit(
            primary_domain.device_id,
            remote_device_id_num,
            password,
            visit_desktop,
        )
        .await?;

    tracing::info!(?local_device_id, ?remote_device_id, "key exchange success");

    let endpoint_id = EndPointID::DeviceID {
        local_device_id,
        remote_device_id: remote_device_id_num,
    };

    if visit_desktop {
        let endpoint_service = endpoint::Service::new(
            endpoint_id,
            EndPointStreamType::ActiveTCP(endpoint_addr),
            Some((opening_key, sealing_key)),
            Some(visit_credentials),
        )
        .await?;

        if let Err(err) = egui_plugin.create_window(
            window_label,
            Box::new(move |cc| {
                if let Some(gl_context) = cc.gl.clone() {
                    Box::new(create_desktop_window(
                        cc,
                        gl_context,
                        endpoint_id,
                        endpoint_service,
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
        let endpoint_service = endpoint::Service::new(
            endpoint_id,
            EndPointStreamType::ActiveTCP(endpoint_addr),
            Some((opening_key, sealing_key)),
            Some(visit_credentials),
        )
        .await?;

        file_transfer_cache
            .0
            .insert(remote_device_id.clone(), endpoint_service)
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
            file_transfer_cache.0.invalidate(&remote_device_id).await;
            tracing::error!(?err, "create file manager window failed");
            return Err(core_error!("create remote file manager window failed"));
        }
    }

    let _ = config
        .history()
        .create(remote_device_id_num, &primary_domain.name);

    Ok(())
}
