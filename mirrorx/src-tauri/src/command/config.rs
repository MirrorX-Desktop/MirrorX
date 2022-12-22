use crate::command::AppState;
use mirrorx_core::{
    api::{
        config::{
            entity::{domain::Domain, kv::Theme},
            LocalStorage,
        },
        signaling::http_message::Response,
    },
    core_error,
    error::CoreResult,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tauri::{
    http::Uri, AppHandle, CustomMenuItem, Manager, State, SystemTrayMenu, SystemTrayMenuItem,
    Window,
};

#[tauri::command]
#[tracing::instrument(skip(app_handle, app_state))]
pub async fn config_init(
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> CoreResult<()> {
    let config_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .ok_or(core_error!("read app dir from path resolver failed"))?;

    tracing::info!(path = ?config_dir, "config dir");
    std::fs::create_dir_all(config_dir.clone())?;

    let storage = LocalStorage::new(&config_dir.join("mirrorx.db"))?;
    let domain_count = storage.domain().get_domain_count()?;

    let mut storage_guard = app_state.storage.lock().await;
    *storage_guard = Some(storage);
    drop(storage_guard);

    if domain_count == 0 {
        config_domain_create(
            app_state,
            String::from("http://mirrorx.cloud:28000"),
            true,
            String::default(),
        )
        .await?;
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_get(app_state: State<'_, AppState>) -> CoreResult<Domain> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.domain().get_primary_domain()
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_get_id_and_names(
    app_state: State<'_, AppState>,
) -> CoreResult<Vec<(i64, String)>> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.domain().get_domain_id_and_names()
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_create(
    app_state: State<'_, AppState>,
    addr: String,
    is_primary: bool,
    remarks: String,
) -> CoreResult<()> {
    let uri = addr
        .parse::<SocketAddr>()
        .map(|addr| {
            Uri::builder()
                .scheme("http")
                .authority(addr.to_string())
                .path_and_query("")
                .build()
                .map_err(|_| core_error!("invalid addr format"))
        })
        .unwrap_or_else(|_| Uri::try_from(addr).map_err(|_| core_error!("invalid uri format")))?;

    let client = mirrorx_core::api::signaling::SignalingClient::new(uri.to_string())?;
    let response = match client.identity().await? {
        Response::Message(resp) => resp,
        Response::Error(err) => return Err(core_error!("http error: {:?}", err)),
    };

    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let domain = response.domain;
    let signaling_port = response.signaling_port;
    let subscribe_port = response.subscribe_port;
    if storage.domain().domain_exist(&domain)? {
        return Err(core_error!("domain is exists"));
    }

    let finger_print = mirrorx_core::utility::rand::generate_device_finger_print();
    let response = match client.domain_register(0, &finger_print).await? {
        Response::Message(resp) => resp,
        Response::Error(err) => return Err(core_error!("http error: {:?}", err)),
    };

    storage.domain().add_domain(Domain {
        id: 0,
        name: domain,
        addr: uri.to_string(),
        signaling_port,
        subscribe_port,
        is_primary,
        device_id: response.device_id,
        password: mirrorx_core::utility::rand::generate_random_password(),
        finger_print,
        remarks,
    })?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_delete(id: i64, app_state: State<'_, AppState>) -> CoreResult<()> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.domain().delete_domain(id)
}

#[derive(Serialize)]
pub struct ConfigDomainListResponse {
    pub total: u32,
    pub domains: Vec<Domain>,
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_list(
    page: u32,
    limit: u32,
    app_state: tauri::State<'_, AppState>,
) -> CoreResult<ConfigDomainListResponse> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    let (total, domains) = storage.domain().get_domains(page, limit)?;

    Ok(ConfigDomainListResponse { total, domains })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDomainUpdateRequest {
    pub id: i64,
    pub update_type: ConfigDomainUpdateType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDomainUpdateType {
    SetPrimary,
    Password(String),
    Remarks(String),
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_domain_update(
    app_state: tauri::State<'_, AppState>,
    req: ConfigDomainUpdateRequest,
) -> CoreResult<()> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    match req.update_type {
        ConfigDomainUpdateType::SetPrimary => {
            let current_signaling = app_state.signaling_client.lock().await;
            if let Some((domain_id, _)) = *current_signaling {
                if domain_id == req.id {
                    return Ok(());
                }
            }

            storage.domain().set_domain_is_primary(req.id)?;
        }
        ConfigDomainUpdateType::Password(new_password) => storage
            .domain()
            .set_domain_device_password(req.id, &new_password)?,
        ConfigDomainUpdateType::Remarks(new_remarks) => {
            storage.domain().set_domain_remarks(req.id, &new_remarks)?
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_language_get(app_state: State<'_, AppState>) -> CoreResult<String> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    Ok(storage.kv().get_language()?.unwrap_or_default())
}

#[derive(Serialize, Clone)]
struct UpdateLanguageEvent {
    pub language: String,
}

#[tauri::command]
#[tracing::instrument(skip(app_state, app_handle, window))]
pub async fn config_language_set(
    app_state: State<'_, AppState>,
    app_handle: AppHandle,
    window: Window,
    language: String,
) -> CoreResult<()> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.kv().set_language(&language)?;

    app_handle
        .emit_all(
            "update_language",
            UpdateLanguageEvent {
                language: language.clone(),
            },
        )
        .map_err(|err| {
            tracing::error!(?err, "emit event 'update_language' failed");
            core_error!("emit event 'update_language' failed")
        })?;

    // update menu language

    let (quit_text, show_text, hide_text, about_text) = match language.as_str() {
        "en" => ("Quit", "Show", "Hide", "About"),
        "zh" => ("退出", "显示", "隐藏", "关于"),
        _ => return Ok(()),
    };

    let quit = CustomMenuItem::new("quit", quit_text);
    let show = CustomMenuItem::new("show", show_text);
    let hide = CustomMenuItem::new("hide", hide_text);
    let about = CustomMenuItem::new("about", about_text);

    let tray_menu = if cfg!(target_os = "macos") {
        SystemTrayMenu::new()
            .add_item(hide)
            .add_item(show)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(quit)
    } else {
        SystemTrayMenu::new()
            .add_item(hide)
            .add_item(show)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(about)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(quit)
    };

    if let Err(err) = app_handle.tray_handle().set_menu(tray_menu) {
        tracing::error!(?err, "set new tray menu failed");
    }

    #[cfg(target_os = "macos")]
    {
        let about_text = match language.as_str() {
            "en" => "About",
            "zh" => "关于",
            _ => return Ok(()),
        };

        if let Err(err) = window
            .menu_handle()
            .get_item("about")
            .set_title(format!("{about_text} MirrorX"))
        {
            tracing::error!(menu = "about", ?err, "set os menu failed");
        }

        if let Err(err) = window.menu_handle().get_item("quit").set_title(quit_text) {
            tracing::error!(menu = "quit", ?err, "set os menu failed");
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_theme_get(app_state: State<'_, AppState>) -> CoreResult<Option<Theme>> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.kv().get_theme()
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn config_theme_set(app_state: State<'_, AppState>, theme: Theme) -> CoreResult<()> {
    let Some(ref storage) = *app_state.storage.lock().await else {
        return Err(core_error!("storage not initialize"));
    };

    storage.kv().set_theme(theme)?;

    Ok(())
}
