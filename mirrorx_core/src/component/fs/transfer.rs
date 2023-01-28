use crate::{
    api::endpoint::{
        client::EndPointClient,
        message::{EndPointFileTransferBlock, EndPointFileTransferError, EndPointMessage},
    },
    error::CoreResult,
};
use moka::future::{Cache, CacheBuilder};
use once_cell::sync::Lazy;
use std::{path::Path, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

pub static APPEND_FILES: Lazy<Cache<String, UnboundedSender<Option<Vec<u8>>>>> = Lazy::new(|| {
    CacheBuilder::new(64)
        .time_to_idle(Duration::from_secs(3 * 60))
        .build()
});

pub static BYTES_TRANSFERRED_CACHE: Lazy<Cache<String, u64>> = Lazy::new(|| {
    CacheBuilder::new(64)
        .time_to_idle(Duration::from_secs(3 * 60))
        .build()
});

pub async fn create_file_append_session(id: String, path: &Path) -> CoreResult<()> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    APPEND_FILES.insert(id.clone(), tx).await;

    if let Err(err) = save_file_from_remote(id.clone(), path, rx).await {
        APPEND_FILES.invalidate(&id).await;
        return Err(err);
    }

    Ok(())
}

pub async fn delete_file_append_session(id: &str) {
    APPEND_FILES.invalidate(id).await
}

pub async fn append_file_block(client: Arc<EndPointClient>, block: EndPointFileTransferBlock) {
    if let Some(tx) = APPEND_FILES.get(&block.id) {
        match tx.send(block.data) {
            Ok(_) => return,
            Err(_) => {
                tracing::error!(id = block.id, "append file block channel failed");
            }
        }
    } else {
        tracing::error!(id = block.id, "file session not exists");
    }

    let _ = client
        .send(&EndPointMessage::FileTransferError(
            EndPointFileTransferError { id: block.id },
        ))
        .await;
}

async fn save_file_from_remote(
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

                    update_transferred_bytes_count(&id, buffer.len() as _).await;
                }
                None => {
                    break;
                }
            }
        }

        let _ = writer.flush().await;

        APPEND_FILES.invalidate(&id).await;
    });

    Ok(())
}

pub async fn send_file_to_remote(
    id: String,
    client: Arc<EndPointClient>,
    path: &Path,
) -> CoreResult<()> {
    let file = tokio::fs::File::open(path).await?;
    let mut reader = BufReader::new(file);

    tokio::spawn(async move {
        let mut buffer = [0u8; 1024 * 64];

        loop {
            let (message, n) = match reader.read(&mut buffer).await {
                Ok(n) => {
                    let content = if n > 0 {
                        Some(buffer.as_slice()[0..n].to_vec())
                    } else {
                        None
                    };

                    (
                        EndPointMessage::FileTransferBlock(EndPointFileTransferBlock {
                            id: id.clone(),
                            data: content,
                        }),
                        n,
                    )
                }
                Err(err) => {
                    tracing::error!(?err, "read file failed");
                    (
                        EndPointMessage::FileTransferError(EndPointFileTransferError {
                            id: id.clone(),
                        }),
                        0,
                    )
                }
            };

            if let Err(err) = client.send(&message).await {
                tracing::error!(?err, "send file message failed");
                break;
            }

            update_transferred_bytes_count(&id, n as _).await;

            match message {
                EndPointMessage::FileTransferBlock(message) if message.data.is_none() => break,
                EndPointMessage::FileTransferError(_) => break,
                _ => {}
            }
        }
    });

    Ok(())
}

pub fn query_transferred_bytes_count(id: &str) -> u64 {
    BYTES_TRANSFERRED_CACHE.get(id).unwrap_or_default()
}

async fn update_transferred_bytes_count(id: &str, delta: u64) {
    let transferred = BYTES_TRANSFERRED_CACHE.get(id).unwrap_or_default() + delta;
    BYTES_TRANSFERRED_CACHE
        .insert(id.to_string(), transferred)
        .await;
}
