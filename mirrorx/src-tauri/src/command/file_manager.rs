use super::AppState;
use mirrorx_core::{
    api::endpoint::message::{
        EndPointCallRequest, EndPointDownloadFileReply, EndPointDownloadFileRequest,
        EndPointFileTransferError, EndPointMessage, EndPointSendFileReply, EndPointSendFileRequest,
        EndPointVisitDirectoryRequest, EndPointVisitDirectoryResponse,
    },
    component::fs::transfer::{
        create_file_append_session, query_transferred_bytes_count, send_file_to_remote,
    },
    core_error,
    error::CoreResult,
};
use rayon::prelude::*;
use serde::Serialize;
use std::path::PathBuf;

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
    let client = app_state
        .files_endpoints
        .lock()
        .await
        .get(&remote_device_id)
        .ok_or_else(|| core_error!("remote file manager not exist"))?;

    let reply: EndPointVisitDirectoryResponse = client
        .call(EndPointCallRequest::VisitDirectoryRequest(
            EndPointVisitDirectoryRequest { path },
        ))
        .await?;

    let path = reply.dir.path;
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::task::spawn_blocking(move || {
        let entries: Vec<EntryResult> = reply
            .dir
            .entries
            .into_par_iter()
            .map(|entry| EntryResult {
                is_dir: entry.is_dir,
                path: entry.path,
                modified_time: entry.modified_time,
                size: entry.size,
                icon: entry.icon.map(base64::encode),
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
                icon: entry.icon.map(base64::encode),
            })
            .collect();

        let _ = tx.send(entries);
    });
    let entries = rx.await?;

    Ok(DirectoryResult { path, entries })
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn file_manager_send_file(
    app_state: tauri::State<'_, AppState>,
    remote_device_id: String,
    local_path: PathBuf,
    remote_path: PathBuf,
) -> CoreResult<(String, u64)> {
    if !local_path.is_file() {
        return Err(core_error!("local path is not a file"));
    }

    let Some(filename) = local_path.file_name() else {
         return Err(core_error!("local path get filename failed"));
    };

    let filename = filename
        .to_str()
        .ok_or_else(|| core_error!("convert filename failed"))?
        .to_string();

    let meta = local_path.metadata()?;
    let size = meta.len();

    let id = uuid::Uuid::new_v4().to_string();

    let client = app_state
        .files_endpoints
        .lock()
        .await
        .get(&remote_device_id)
        .ok_or_else(|| core_error!("remote file manager not exist"))?;

    let _: EndPointSendFileReply = client
        .call(EndPointCallRequest::SendFileRequest(
            EndPointSendFileRequest {
                id: id.clone(),
                filename,
                path: remote_path,
                size,
            },
        ))
        .await?;

    send_file_to_remote(id.clone(), client, &local_path).await?;

    Ok((id, size))
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn file_manager_download_file(
    app_state: tauri::State<'_, AppState>,
    remote_device_id: String,
    local_path: PathBuf,
    remote_path: PathBuf,
) -> CoreResult<(String, u64)> {
    if local_path.exists() {
        return Err(core_error!("local path is not a file"));
    }

    let id = uuid::Uuid::new_v4().to_string();

    let client = app_state
        .files_endpoints
        .lock()
        .await
        .get(&remote_device_id)
        .ok_or_else(|| core_error!("remote file manager not exist"))?;

    let reply: EndPointDownloadFileReply = client
        .call(EndPointCallRequest::DownloadFileRequest(
            EndPointDownloadFileRequest {
                id: id.clone(),
                path: remote_path,
            },
        ))
        .await?;

    if let Err(err) = create_file_append_session(id.clone(), &local_path).await {
        let _ = client
            .send(&EndPointMessage::FileTransferError(
                EndPointFileTransferError { id: id.clone() },
            ))
            .await;

        return Err(err);
    }

    Ok((id, reply.size))
}

#[tauri::command]
pub async fn file_manager_query_transferred_bytes_count(id: String) -> u64 {
    query_transferred_bytes_count(&id)
}
