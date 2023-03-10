use crate::{
    component::fs::transfer::send_file_to_remote,
    core_error,
    error::CoreResult,
    service::endpoint::{
        message::{
            EndPointDownloadFileReply, EndPointDownloadFileRequest, EndPointFileTransferError,
            EndPointMessage,
        },
        service::Service,
    },
};
use std::{sync::Arc, time::Duration};

pub async fn handle_download_file_request(
    service: Arc<Service>,
    req: EndPointDownloadFileRequest,
) -> CoreResult<EndPointDownloadFileReply> {
    if !req.path.is_file() {
        return Err(core_error!("file not exists"));
    }

    let id = req.id.clone();
    let meta = req.path.metadata()?;
    let size = meta.len();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        if let Err(err) = send_file_to_remote(id.clone(), service.clone(), &req.path).await {
            tracing::error!(?err, "read file block failed");
            let _ = service
                .send(&EndPointMessage::FileTransferError(
                    EndPointFileTransferError { id: id.clone() },
                ))
                .await;
        }
    });

    Ok(EndPointDownloadFileReply { size })
}
