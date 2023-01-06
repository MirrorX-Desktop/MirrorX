use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointFileTransferBlock, EndPointFileTransferTerminate, EndPointMessage},
    },
    core_error,
    error::CoreResult,
};
use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use std::{path::Path, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    sync::mpsc::{Sender, UnboundedReceiver, UnboundedSender},
};

pub static FILES: Lazy<Cache<String, UnboundedSender<Option<Vec<u8>>>>> =
    Lazy::new(|| CacheBuilder::new(64).build());

pub async fn create_file_transfer_session(id: String, path: &Path) -> CoreResult<()> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    FILES.insert(id.clone(), tx).await;

    if let Err(err) = write_local_file(id.clone(), path, rx).await {
        FILES.invalidate(&id).await;
        return Err(err);
    }

    Ok(())
}

pub async fn delete_transfer_session(id: &str) {
    FILES.invalidate(id).await
}

pub async fn read_file_block(
    id: String,
    client: Arc<EndPointClient>,
    path: &Path,
) -> CoreResult<()> {
    let path = path.to_path_buf();
    let (tx, mut rx) = tokio::sync::mpsc::channel(64);

    tokio::spawn(async move {
        match read_local_file(&path, tx).await {
            Ok(rx) => rx,
            Err(err) => {
                tracing::error!(?err, "read file block failed");
                let _ = client
                    .send(&EndPointMessage::FileTransferTerminate(
                        EndPointFileTransferTerminate { id: id.clone() },
                    ))
                    .await;
                return;
            }
        };

        loop {
            let message = match rx.recv().await {
                Some(buffer) => EndPointMessage::FileTransferBlock(EndPointFileTransferBlock {
                    id: id.clone(),
                    finish: buffer.is_none(),
                    data: buffer.unwrap_or_default(),
                }),
                None => EndPointMessage::FileTransferTerminate(EndPointFileTransferTerminate {
                    id: id.clone(),
                }),
            };

            if let Err(err) = client.send(&message).await {
                tracing::error!(?err, "send file message failed");
                break;
            }
        }
    });

    Ok(())
}

async fn read_local_file(path: &Path, tx: Sender<Option<Vec<u8>>>) -> CoreResult<()> {
    let file = tokio::fs::File::open(path).await?;
    let mut reader = BufReader::new(file);
    let path = path.to_path_buf();

    tokio::spawn(async move {
        let mut buffer = [0u8; 1024 * 8];

        loop {
            let size = match reader.read(&mut buffer).await {
                Ok(n) => n,
                Err(err) => {
                    tracing::error!(?err, ?path, "read file failed");
                    break;
                }
            };

            let content = if size > 0 {
                Some(buffer.as_slice()[0..size].to_vec())
            } else {
                None
            };

            if tx.send(content).await.is_err() {
                tracing::error!("send file content failed");
                break;
            }
        }
    });

    Ok(())
}

pub async fn write_file_block(block: EndPointFileTransferBlock) -> CoreResult<()> {
    if let Some(tx) = FILES.get(&block.id) {
        let content = if block.finish { None } else { Some(block.data) };

        tx.send(content)
            .map_err(|_| core_error!("file session not exists: {}", block.id))?;

        Ok(())
    } else {
        Err(core_error!("file session not exists: {}", block.id))
    }
}

async fn write_local_file(
    id: String,
    path: &Path,
    mut rx: UnboundedReceiver<Option<Vec<u8>>>,
) -> CoreResult<()> {
    let file = tokio::fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);

    tokio::spawn(async move {
        loop {
            let Some(buffer) = rx.recv().await else {
                tracing::info!("exit write file");
                break;
            };

            match buffer {
                Some(buffer) => {
                    if let Err(err) = writer.write_all(&buffer).await {
                        tracing::error!(?err, "write file has error occurred");
                        break;
                    }
                }
                None => {
                    if let Err(err) = writer.flush().await {
                        tracing::error!(?err, "write flush file failed");
                        break;
                    }
                }
            }
        }

        FILES.invalidate(&id).await;
    });

    Ok(())
}
