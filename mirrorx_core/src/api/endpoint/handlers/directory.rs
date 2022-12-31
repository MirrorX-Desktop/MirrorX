use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointDirectoryRequest, EndPointDirectoryResponse, EndPointMessage},
    },
    component::fs::{read_directory, read_root_directory},
};
use std::sync::Arc;

pub async fn handle_directory_request(client: Arc<EndPointClient>, req: EndPointDirectoryRequest) {
    let dir = if let Some(path) = req.path {
        read_directory(path)
    } else {
        read_root_directory()
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
