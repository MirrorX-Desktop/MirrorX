use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointDirectoryRequest, EndPointDirectoryResponse, EndPointMessage},
    },
    component::fs::{read_directory, read_root_directory},
};
use once_cell::sync::Lazy;
use std::{path::PathBuf, sync::Arc};

static UNIX_ROOT: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/"));
static WINDOWS_ROOT: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(r"\"));

pub async fn handle_directory_request(client: Arc<EndPointClient>, req: EndPointDirectoryRequest) {
    let dir = if req.path == UNIX_ROOT.as_path() || req.path == WINDOWS_ROOT.as_path() {
        read_root_directory()
    } else {
        read_directory(req.path)
    };

    if let Err(err) = client
        .send(&EndPointMessage::DirectoryResponse(
            EndPointDirectoryResponse {
                result: dir.map_err(|err| err.to_string()),
            },
        ))
        .await
    {
        tracing::error!(?err, "send directory response failed");
    }
}
