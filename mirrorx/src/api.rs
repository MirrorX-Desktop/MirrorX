use crate::{event::PopupDialogInputRemotePasswordEvent, utility::format_device_id};
use mirrorx_core::api::{
    config::{Config, DomainConfig},
    signaling::{PublishMessage, ResourceType, SignalingClient, VisitReplyRequest},
};
use std::{collections::HashMap, path::PathBuf};
use tauri::async_runtime::Mutex;
use tauri_egui::{
    eframe::{glow::HasContext, HardwareAcceleration},
    EguiPluginHandle,
};
use tokio::sync::mpsc::Receiver;

#[derive(Default)]
pub struct UIState {
    config: Mutex<Option<Config>>,
    config_path: Mutex<PathBuf>,
    signaling_client: Mutex<SignalingClient>,
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, state))]
pub async fn init_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, UIState>,
) -> Result<(), String> {
    let app_dir = app_handle
        .path_resolver()
        .app_dir()
        .ok_or("read app dir from path resolver failed")?;

    tracing::info!(?app_dir, "app dir");

    if let Err(err) = std::fs::create_dir_all(app_dir.clone()) {
        tracing::error!(?err, "create config dir failed");
        return Err("create config db directory failed".into());
    }

    let db_filepath = app_dir.join("mirrorx.db");

    match mirrorx_core::api::config::read(&db_filepath) {
        Ok(config) => {
            let config = match config {
                Some(config) => config,
                None => {
                    let mut domain_configs = HashMap::new();
                    domain_configs.insert(
                        String::from("MirrorX.cloud"),
                        DomainConfig {
                            addr: String::from("tcp://192.168.0.101:28000"),
                            device_id: 0,
                            device_finger_print:
                                mirrorx_core::utility::rand::generate_device_finger_print(),
                            device_password: mirrorx_core::utility::rand::generate_random_password(
                            ),
                        },
                    );

                    let default_config = Config {
                        primary_domain: String::from("MirrorX.cloud"),
                        domain_configs,
                    };

                    if let Err(err) = mirrorx_core::api::config::save(&db_filepath, &default_config)
                    {
                        tracing::error!(?err, "save config failed");
                        return Err("save newly initialize config failed".into());
                    }

                    default_config
                }
            };

            *state.config.lock().await = Some(config);
            *state.config_path.lock().await = db_filepath;
            Ok(())
        }
        Err(err) => {
            tracing::error!(?err, "read config db failed");
            Err("open config db failed".into())
        }
    }
}

#[tauri::command]
#[tracing::instrument(skip(state, window))]
pub async fn init_signaling_client(
    domain: String,
    state: tauri::State<'_, UIState>,
    window: tauri::Window,
) -> Result<(), String> {
    let mut signaling_client = state.signaling_client.lock().await;
    if signaling_client.domain() == domain {
        return Ok(());
    }

    let config_db_path = state.config_path.lock().await;
    let (publish_message_tx, publish_message_rx) = tokio::sync::mpsc::channel(8);

    let device_id = signaling_client
        .dial(&domain, &config_db_path, publish_message_tx)
        .await
        .map_err(|err| {
            tracing::error!(?domain, ?err, "init signaling client failed");
            "Signaling client initialize failed"
        })?;

    let mut guard = state.config.lock().await;

    let config = guard.as_mut().ok_or("get primary domain failed")?;

    let mut domain_config = config
        .domain_configs
        .get_mut(&domain)
        .ok_or("current domain doesn't have config")?;

    if domain_config.device_id != device_id {
        domain_config.device_id = device_id;

        if let Err(err) = mirrorx_core::api::config::save(&config_db_path, config) {
            tracing::error!(?err, "save config failed");
            return Err("save config failed".into());
        }
    }

    start_signaling_publish_event_handle(publish_message_rx, window);

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_config_primary_domain(state: tauri::State<'_, UIState>) -> Result<String, String> {
    let primary_domain = state
        .config
        .lock()
        .await
        .as_ref()
        .ok_or("get primary domain failed")?
        .primary_domain
        .to_owned();

    Ok(primary_domain)
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_config_device_id(
    domain: String,
    state: tauri::State<'_, UIState>,
) -> Result<String, String> {
    let device_id = state
        .config
        .lock()
        .await
        .as_ref()
        .ok_or("get primary domain failed")?
        .domain_configs
        .get(&domain)
        .ok_or("current domain doesn't have config")?
        .device_id;

    Ok(format_device_id(device_id))
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn get_config_device_password(
    domain: String,
    state: tauri::State<'_, UIState>,
) -> Result<String, String> {
    let password = state
        .config
        .lock()
        .await
        .as_ref()
        .ok_or("get primary domain failed")?
        .domain_configs
        .get(&domain)
        .ok_or("current domain doesn't have config")?
        .device_password
        .to_owned();

    Ok(password)
}

#[tauri::command]
#[tracing::instrument]
pub fn generate_random_password() -> String {
    mirrorx_core::utility::rand::generate_random_password()
}

#[tauri::command]
#[tracing::instrument(skip(password, state))]
pub async fn set_config_device_password(
    domain: String,
    password: String,
    state: tauri::State<'_, UIState>,
) -> Result<(), String> {
    let mut guard = state.config.lock().await;

    let config = guard.as_mut().ok_or("get primary domain failed")?;

    let mut domain_config = config
        .domain_configs
        .get_mut(&domain)
        .ok_or("current domain doesn't have config")?;

    domain_config.device_password = password;

    let config_db_path = state.config_path.lock().await;

    if let Err(err) = mirrorx_core::api::config::save(&config_db_path, config) {
        tracing::error!(?err, "save config failed");
        return Err("save config failed".into());
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(state, window))]
pub async fn signaling_visit_request(
    domain: String,
    remote_device_id: String,
    state: tauri::State<'_, UIState>,
    window: tauri::Window,
) -> Result<(), String> {
    let remote_device_id: i64 = remote_device_id
        .replace('-', "")
        .parse()
        .map_err(|_| "invalid device_id format")?;

    let local_device_id = state
        .config
        .lock()
        .await
        .as_ref()
        .ok_or("current config is empty")?
        .domain_configs
        .get(&domain)
        .ok_or("domain config is empty")?
        .device_id;

    let signaling_client = state.signaling_client.lock().await;
    let resp = signaling_client
        .visit(mirrorx_core::api::signaling::VisitRequest {
            local_device_id,
            remote_device_id,
            resource_type: ResourceType::Desktop,
        })
        .await
        .map_err(|err| {
            tracing::error!(?err, "signaling client visit request failed");
            "Visit request failed, remote device is offline or accept timeout"
        })?;

    if resp.allow {
        if let Err(err) = window.emit(
            "popup_dialog_input_remote_password",
            PopupDialogInputRemotePasswordEvent {
                active_device_id: format_device_id(local_device_id),
                passive_device_id: format_device_id(remote_device_id),
            },
        ) {
            tracing::error!(
                ?err,
                "window emit 'pop_dialog_input_remote_password' event failed"
            );
        }

        Ok(())
    } else {
        Err("Remote device reject your visit request".into())
    }
}

#[tauri::command]
#[tracing::instrument(skip(state))]
pub async fn signaling_reply_visit_request(
    allow: bool,
    active_device_id: String,
    passive_device_id: String,
    state: tauri::State<'_, UIState>,
) -> Result<(), String> {
    let active_device_id: i64 = active_device_id
        .replace('-', "")
        .parse()
        .map_err(|_| "invalid device_id format")?;

    let passive_device_id: i64 = passive_device_id
        .replace('-', "")
        .parse()
        .map_err(|_| "invalid device_id format")?;

    let signaling_client = state.signaling_client.lock().await;
    if let Err(err) = signaling_client
        .visit_reply(mirrorx_core::api::signaling::VisitReplyRequest {
            active_device_id,
            passive_device_id,
            allow,
        })
        .await
    {
        tracing::error!(
            ?active_device_id,
            ?passive_device_id,
            ?allow,
            ?err,
            "signaling client reply visit request failed"
        );
        return Err(
            "Reply visit request failed, maybe remote device is offline or reply timeout".into(),
        );
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, state, egui_plugin))]
pub async fn signaling_key_exchange(
    local_device_id: String,
    remote_device_id: String,
    password: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, UIState>,
    egui_plugin: tauri::State<'_, EguiPluginHandle>,
) -> Result<(), String> {
    let local_device_id = local_device_id
        .replace('-', "")
        .parse()
        .map_err(|_| "invalid device_id format")?;

    let window_label = format!("MirrorX {}", remote_device_id);

    let remote_device_id: i64 = remote_device_id
        .replace('-', "")
        .parse()
        .map_err(|_| "invalid device_id format")?;

    let signaling_client = state.signaling_client.lock().await;
    let resp = signaling_client
        .key_exchange(mirrorx_core::api::signaling::KeyExchangeRequest {
            local_device_id,
            remote_device_id,
            password,
        })
        .await
        .map_err(|err| {
            tracing::error!(?err, "signaling client key exchange failed");
            String::from("Key exchange failed, please try again later")
        })?;

    tracing::info!("key exchange success");

    if let Err(err) = egui_plugin.create_window(
        window_label.clone(),
        Box::new(move |cc| {
            // cc.egui_ctx.set_debug_on_hover(true);
            if let Some(gl_context) = cc.gl.as_ref() {
                set_fonts(&cc.egui_ctx);
                Box::new(crate::window::desktop::DesktopWindow::new(
                    local_device_id,
                    remote_device_id,
                    resp.opening_key_bytes,
                    resp.opening_nonce_bytes,
                    resp.sealing_key_bytes,
                    resp.sealing_nonce_bytes,
                    resp.visit_credentials,
                    gl_context.clone(),
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
        return Err("create remote desktop window failed".into());
    }

    Ok(())
}

fn start_signaling_publish_event_handle(
    mut publish_message_rx: Receiver<PublishMessage>,
    window: tauri::Window,
) {
    tokio::spawn(async move {
        loop {
            match publish_message_rx.recv().await {
                Some(publish_message) => match publish_message {
                    mirrorx_core::api::signaling::PublishMessage::VisitRequest {
                        active_device_id,
                        passive_device_id,
                        resource_type,
                    } => {
                        if let Err(err) = window.emit(
                            "popup_dialog_visit_request",
                            crate::event::PopupDialogVisitRequestEvent {
                                active_device_id: format_device_id(active_device_id),
                                passive_device_id: format_device_id(passive_device_id),
                                resource_type: if let ResourceType::Desktop = resource_type {
                                    "desktop"
                                } else {
                                    "files"
                                }
                                .into(),
                            },
                        ) {
                            tracing::error!(?err, "window emit 'pop_dialog_visit_request' failed");
                        }
                    }
                },
                None => {
                    tracing::error!("publish message channel is closed");
                    return;
                }
            }
        }
    });
}

fn set_fonts(ctx: &tauri_egui::egui::Context) {
    let mut fonts = tauri_egui::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "NotoSans".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSans-Regular.ttf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansJP".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansJP-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansKR".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansKR-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansSC".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansSC-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansTC".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansTC-Regular.otf"
        )),
    );
    fonts.font_data.insert(
        "NotoSansMono".to_owned(),
        tauri_egui::egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansMono-Regular.ttf"
        )),
    );

    let mut proportional_fonts = vec![
        "NotoSans".to_owned(),
        "NotoSansSC".to_owned(),
        "NotoSansTC".to_owned(),
        "NotoSansJP".to_owned(),
        "NotoSansKR".to_owned(),
    ];

    let old_fonts = fonts
        .families
        .entry(tauri_egui::egui::FontFamily::Proportional)
        .or_default();

    proportional_fonts.append(old_fonts);

    fonts.families.insert(
        tauri_egui::egui::FontFamily::Proportional,
        proportional_fonts.clone(),
    );

    let mut mono_fonts = vec!["NotoSansMono".to_owned()];

    let old_fonts = fonts
        .families
        .entry(tauri_egui::egui::FontFamily::Monospace)
        .or_default();

    mono_fonts.append(old_fonts);

    fonts
        .families
        .insert(tauri_egui::egui::FontFamily::Monospace, mono_fonts.clone());

    // cc.egui_ctx.set_debug_on_hover(true);
    // cc.egui_ctx.request_repaint_after(Duration::from_secs(1));

    ctx.set_fonts(fonts);
}
