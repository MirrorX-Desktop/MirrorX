use crate::{
    api::endpoint::message::{EndPointSendFileReply, EndPointSendFileRequest},
    component::fs::transfer::create_file_append_session,
    core_error,
    error::CoreResult,
};

pub async fn handle_send_file_request(
    req: EndPointSendFileRequest,
) -> CoreResult<EndPointSendFileReply> {
    let path = req.path.join(req.filename);

    if path.exists() {
        return Err(core_error!("file already exists"));
    }

    create_file_append_session(req.id, &path).await?;

    Ok(EndPointSendFileReply {})
}
