use super::AppState;
use mirrorx_core::{
    api::endpoint::message::{EndPointDirectoryRequest, EndPointMessage},
    core_error,
    error::CoreResult,
};
use rayon::prelude::*;
use serde::Serialize;
use std::{path::PathBuf, time::Duration};

#[derive(Serialize)]
pub struct DirectoryResult {
    pub path: PathBuf,
    pub entries: Vec<EntryResult>,
}

#[derive(Serialize)]
pub struct EntryResult {
    pub is_dir: bool,
    pub path: PathBuf,
    pub modified_time: i64,
    pub size: u64,
    pub icon: Option<String>,
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn file_manager_visit_remote(
    app_state: tauri::State<'_, AppState>,
    remote_device_id: String,
    path: Option<PathBuf>,
) -> CoreResult<DirectoryResult> {
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

    let directory = tokio::time::timeout(Duration::from_secs(30), directory_rx.recv())
        .await?
        .ok_or_else(|| core_error!("request remote file failed"))?
        .result
        .map_err(|err| core_error!("{}", err))?;

    let path = directory.path;
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::task::spawn_blocking(move || {
        let entries: Vec<EntryResult> = directory
            .entries
            .into_par_iter()
            .map(|entry| EntryResult {
                is_dir: entry.is_dir,
                path: entry.path,
                modified_time: entry.modified_time,
                size: entry.size,
                icon: entry.icon.map(|v| base64::encode(&v)),
            })
            .collect();

        let _ = tx.send(entries);
    });
    let entries = rx.await?;

    Ok(DirectoryResult { path, entries })
}

#[tauri::command]
#[tracing::instrument]
pub async fn file_manager_visit_local(path: Option<PathBuf>) -> CoreResult<DirectoryResult> {
    let directory = if let Some(path) = path {
        tracing::info!(?path, "require path");
        mirrorx_core::component::fs::read_directory(&path)
    } else {
        mirrorx_core::component::fs::read_root_directory()
    }?;

    let path = directory.path;
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::task::spawn_blocking(move || {
        let entries: Vec<EntryResult> = directory
            .entries
            .into_par_iter()
            .map(|entry| EntryResult {
                is_dir: entry.is_dir,
                path: entry.path,
                modified_time: entry.modified_time,
                size: entry.size,
                icon: entry.icon.map(|v| base64::encode(&v)),
            })
            .collect();

        let _ = tx.send(entries);
    });
    let entries = rx.await?;

    Ok(DirectoryResult { path, entries })
}
