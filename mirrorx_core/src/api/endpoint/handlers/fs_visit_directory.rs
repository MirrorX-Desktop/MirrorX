use crate::{
    api::endpoint::message::{EndPointVisitDirectoryRequest, EndPointVisitDirectoryResponse},
    component::fs::{read_directory, read_root_directory},
    error::CoreResult,
};

pub async fn handle_visit_directory_request(
    req: EndPointVisitDirectoryRequest,
) -> CoreResult<EndPointVisitDirectoryResponse> {
    let dir = if let Some(path) = req.path {
        tracing::info!(?path, "require path");
        read_directory(&path)
    } else {
        read_root_directory()
    }?;

    Ok(EndPointVisitDirectoryResponse { dir })
}
