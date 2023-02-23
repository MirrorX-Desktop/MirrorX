use crate::{
    api::endpoint::message::{EndPointVisitDirectoryReply, EndPointVisitDirectoryRequest},
    component::fs::{read_directory, read_root_directory},
    error::CoreResult,
};

pub async fn handle_visit_directory_request(
    req: EndPointVisitDirectoryRequest,
) -> CoreResult<EndPointVisitDirectoryReply> {
    let dir = if let Some(path) = req.path {
        tracing::info!(?path, "require path");
        read_directory(&path)
    } else {
        read_root_directory()
    }?;

    Ok(EndPointVisitDirectoryReply { dir })
}
