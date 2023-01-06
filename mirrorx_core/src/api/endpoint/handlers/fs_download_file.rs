use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{
            EndPointDownloadFileReply, EndPointDownloadFileRequest, EndPointFileTransferTerminate,
            EndPointMessage,
        },
    },
    component::fs::transfer::read_file_block,
    core_error,
    error::CoreResult,
};
use std::{sync::Arc, time::Duration};

pub async fn handle_download_file_request(
    client: Arc<EndPointClient>,
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
        if let Err(err) = read_file_block(id.clone(), client.clone(), &req.path).await {
            tracing::error!(?err, "read file block failed");
            let _ = client
                .send(&EndPointMessage::FileTransferTerminate(
                    EndPointFileTransferTerminate { id: id.clone() },
                ))
                .await;
        }
    });

    Ok(EndPointDownloadFileReply { size })
}
