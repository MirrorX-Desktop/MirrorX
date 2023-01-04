use super::AppState;
use mirrorx_core::{
    api::endpoint::message::{EndPointDirectoryRequest, EndPointMessage},
    component::fs::Directory,
    core_error,
    error::CoreResult,
};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct DirectoryResult {
    pub path: Vec<String>,
    pub sub_dirs: Vec<DirEntryResult>,
    pub files: Vec<FileEntryResult>,
}

#[derive(Serialize)]
pub struct DirEntryResult {
    pub path: Vec<String>,
    pub modified_time: i64,
    pub icon: Option<String>,
}

#[derive(Serialize)]
pub struct FileEntryResult {
    pub path: Vec<String>,
    pub modified_time: i64,
    pub size: u64,
    pub icon: Option<String>,
}

#[tauri::command]
#[tracing::instrument(skip(app_state))]
pub async fn file_manager_visit(
    app_state: tauri::State<'_, AppState>,
    remote_device_id: String,
    path: Option<Vec<String>>,
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

    let directory = directory_rx
        .recv()
        .await
        .ok_or_else(|| core_error!("request remote file failed"))?
        .result
        .map_err(|err| core_error!("{}", err))?;

    let mut sub_dirs = Vec::new();
    for ele in directory.sub_dirs {
        sub_dirs.push(DirEntryResult {
            path: ele.path,
            modified_time: ele.modified_time,
            icon: ele.icon.map(|v| base64::encode(&v)),
        });
    }

    let mut files = Vec::new();
    for ele in directory.files {
        files.push(FileEntryResult {
            path: ele.path,
            modified_time: ele.modified_time,
            size: ele.size,
            icon: ele.icon.map(|v| base64::encode(&v)),
        });
    }

    let result = DirectoryResult {
        path: directory.path,
        sub_dirs,
        files,
    };

    Ok(result)
}
