use super::AppState;
use mirrorx_core::{
    api::endpoint::message::{EndPointDirectoryRequest, EndPointMessage},
    component::fs::Directory,
    core_error,
    error::CoreResult,
};
use std::path::PathBuf;

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn file_manager_visit(
    app_state: tauri::State<'_, AppState>,
    remote_device_id: String,
    path: Option<PathBuf>,
) -> CoreResult<Directory> {
    let mut v = app_state
        .files_endpoints
        .get_mut(&remote_device_id)
        .ok_or_else(|| core_error!("remote file manager not exist"))?;

    let (client, directory_rx) = v.value_mut();

    client
        .send(&EndPointMessage::DirectoryRequest(
            EndPointDirectoryRequest { path },
        ))
        .await?;

    directory_rx
        .recv()
        .await
        .ok_or_else(|| core_error!("request remote file failed"))?
        .result
        .map_err(|err| core_error!("{}", err))
}
