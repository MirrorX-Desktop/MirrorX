use super::UIState;
use mirrorx_core::{
    api::config::{entity::domain::Domain, LocalStorage},
    core_error,
    error::CoreResult,
};

#[tauri::command]
#[tracing::instrument(skip(app_handle, state))]
pub async fn init_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, UIState>,
) -> CoreResult<()> {
    let app_dir = app_handle
        .path_resolver()
        .app_dir()
        .ok_or(core_error!("read app dir from path resolver failed"))?;

    tracing::info!(?app_dir, "app dir");

    std::fs::create_dir_all(app_dir.clone())?;

    let db_path = app_dir.join("mirrorx.db");
    let storage = LocalStorage::make_current(db_path)?;
    let domain_count = storage.domain().get_domain_count()?;

    let primary_domain = if domain_count == 0 {
        storage.domain().add_domain(Domain {
            id: 0,
            name: "MirrorX.cloud".into(),
            addr: "tcp://192.168.0.101:28000".into(),
            is_primary: true,
            device_id: 0,
            password: mirrorx_core::utility::rand::generate_random_password(),
            finger_print: mirrorx_core::utility::rand::generate_device_finger_print(),
            remarks: "".into(),
        })?
    } else {
        storage.domain().get_primary_domain()?
    };

    *state.domain.lock().await = Some(primary_domain);

    Ok(())
}
