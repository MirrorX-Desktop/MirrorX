use crate::{
    api::endpoint::message::{EndPointSendFileReply, EndPointSendFileRequest},
    component::fs::transfer::create_file_transfer_session,
    core_error,
    error::CoreResult,
};

pub async fn handle_send_file_request(
    req: EndPointSendFileRequest,
) -> CoreResult<EndPointSendFileReply> {
    if req.remote_path.exists() {
        return Err(core_error!("file already exists"));
    }

    create_file_transfer_session(req.id, &req.remote_path).await?;

    Ok(EndPointSendFileReply {})
}
