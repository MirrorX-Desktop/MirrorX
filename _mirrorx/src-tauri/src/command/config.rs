use crate::{ConfigService, PortalService};
use mirrorx_core::{
    core_error,
    error::CoreResult,
    service::{
        config::entity::{domain::Domain, history::Record, kv::Theme},
        portal,
    },
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, CustomMenuItem, Manager, State, SystemTrayMenu, SystemTrayMenuItem};

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_domain_get(config: State<'_, ConfigService>) -> CoreResult<Domain> {
    config.domain().get_primary_domain()
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_domain_get_by_name(
    config: State<'_, ConfigService>,
    name: String,
) -> CoreResult<Domain> {
    config.domain().get_domain_by_name(name)
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_domain_get_id_and_names(
    config: State<'_, ConfigService>,
) -> CoreResult<Vec<(i64, String)>> {
    config.domain().get_domain_id_and_names()
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, config))]
pub async fn config_domain_create(
    app_handle: AppHandle,
    config: State<'_, ConfigService>,
    addr: String,
    port: u16,
    is_primary: bool,
    remarks: String,
) -> CoreResult<()> {
    let addr = format!("{addr}:{port}");
    let mut portal = portal::service::Service::new(config.inner().clone());
    portal
        .connect(0, addr.clone(), |_, _, _| -> bool { false })
        .await?;
    let reply = portal.get_server_config().await?;

    let name = reply.name;
    if config.domain().domain_exist(&name)? {
        return Err(core_error!("domain is exists"));
    }

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

    let finger_print = mirrorx_core::utility::rand::generate_device_finger_print();
    let reply = portal.client_register(0, &finger_print).await?;

    config.domain().add_domain(Domain {
        id: 0,
        name,
        addr,
        is_primary,
        device_id: reply.device_id,
        password: mirrorx_core::utility::rand::generate_random_password(),
        finger_print,
        remarks,
    })?;

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_domain_delete(config: State<'_, ConfigService>, id: i64) -> CoreResult<()> {
    let domain = config.domain().get_domain_by_id(id)?;
    config.domain().delete_domain(id)?;
    config.history().delete_domain_related(&domain.name)?;

    Ok(())
}

#[derive(Serialize)]
pub struct ConfigDomainListResponse {
    pub total: u32,
    pub domains: Vec<Domain>,
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_domain_list(
    config: State<'_, ConfigService>,
    page: u32,
    limit: u32,
) -> CoreResult<ConfigDomainListResponse> {
    let (total, domains) = config.domain().get_domains(page, limit)?;

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
#[tracing::instrument(skip(portal, config))]
pub async fn config_domain_update(
    portal: State<'_, PortalService>,
    config: State<'_, ConfigService>,
    req: ConfigDomainUpdateRequest,
) -> CoreResult<()> {
    match req.update_type {
        ConfigDomainUpdateType::SetPrimary => {
            let client = portal.0.lock().await;
            if client.domain_id() == req.id {
                return Ok(());
            }

            config.domain().set_domain_is_primary(req.id)?;
        }
        ConfigDomainUpdateType::Password(new_password) => config
            .domain()
            .set_domain_device_password(req.id, &new_password)?,
        ConfigDomainUpdateType::Remarks(new_remarks) => {
            config.domain().set_domain_remarks(req.id, &new_remarks)?
        }
    }

    Ok(())
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_language_get(config: State<'_, ConfigService>) -> CoreResult<String> {
    Ok(config.kv().get_language()?.unwrap_or_default())
}

#[derive(Serialize, Clone)]
struct UpdateLanguageEvent {
    pub language: String,
}

#[tauri::command]
#[tracing::instrument(skip(app_handle, config))]
pub async fn config_language_set(
    app_handle: AppHandle,
    config: State<'_, ConfigService>,
    language: String,
) -> CoreResult<()> {
    config.kv().set_language(&language)?;

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
        let Some(window) = app_handle.get_window("main") else {
            return Ok(());
        };

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
#[tracing::instrument(skip(config))]
pub async fn config_theme_get(config: State<'_, ConfigService>) -> CoreResult<Option<Theme>> {
    config.kv().get_theme()
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_theme_set(config: State<'_, ConfigService>, theme: Theme) -> CoreResult<()> {
    config.kv().set_theme(theme)
}

#[tauri::command]
#[tracing::instrument(skip(config))]
pub async fn config_history_get(
    config: State<'_, ConfigService>,
    time_range: Option<(i64, i64)>,
) -> CoreResult<Vec<Record>> {
    tracing::info!(?time_range, "query");
    let records = config.history().query(time_range)?;
    Ok(records)
}
