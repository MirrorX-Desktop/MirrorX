use crate::utility::format_device_id;
use mirrorx_core::{
    api::{
        config::{Config, DomainConfig},
        signaling::SignalingClient,
    },
    utility,
};
use std::{collections::HashMap, path::PathBuf};
use tauri::async_runtime::Mutex;

#[derive(Default)]
pub struct UIState {
    config: Mutex<Option<Config>>,
    config_path: Mutex<PathBuf>,
    signaling_client: Mutex<SignalingClient>,
}

#[tauri::command]
pub async fn init_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, UIState>,
) -> Result<(), String> {
    tracing::info!("call init_config");

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
    let (publish_message_tx, mut publish_message_rx) = tokio::sync::mpsc::channel(8);

    let device_id = signaling_client
        .dial(&domain, &config_db_path, publish_message_tx)
        .await
        .map_err(|err| {
            tracing::error!(?domain, ?err, "init signaling client failed");
            "Signaling client initialize failed"
        })?;

    tokio::spawn(async move {
        loop {
            match publish_message_rx.recv().await {
                Some(publish_message) => {
                    if let Err(err) = window.emit("publish_message", publish_message) {
                        tracing::error!(?err, "window emit 'publish_message' event failed");
                    }
                }
                None => {
                    tracing::error!("publish message channel is closed");
                    return;
                }
            }
        }
    });

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

    Ok(())
}

#[tauri::command]
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
pub fn generate_random_password() -> String {
    mirrorx_core::utility::rand::generate_random_password()
}

#[tauri::command]
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
