use super::RECV_MESSAGE_TIMEOUT;
use crate::{
    api::endpoint::{
        id::EndPointID,
        message::{EndPointHandshakeRequest, EndPointHandshakeResponse},
    },
    core_error,
    error::{CoreError, CoreResult},
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use ring::aead::{OpeningKey, SealingKey};
use std::ops::Deref;
use tokio::{
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub async fn serve_tcp(
    stream: TcpStream,
    endpoint_id: EndPointID,
    sealing_key: Option<SealingKey<NonceValue>>,
    opening_key: Option<OpeningKey<NonceValue>>,
    mut visit_credentials: Option<Vec<u8>>,
) -> CoreResult<(Sender<Vec<u8>>, Receiver<Bytes>)> {
    let mut framed = Framed::new(
        stream,
        LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(32 * 1024 * 1024)
            .new_codec(),
    );

    if let Some(visit_credentials) = visit_credentials.take() {
        serve_handshake(&mut framed, visit_credentials, endpoint_id).await?;
    }

    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let (sink, stream) = framed.split();
    serve_tcp_write(endpoint_id, rx, sealing_key, sink);
    let rx = serve_tcp_read(endpoint_id, opening_key, stream)?;
    Ok((tx, rx))
}

async fn serve_handshake(
    stream: &mut Framed<TcpStream, LengthDelimitedCodec>,
    visit_credentials: Vec<u8>,
    endpoint_id: EndPointID,
) -> CoreResult<()> {
    let EndPointID::DeviceID { local_device_id, remote_device_id } = endpoint_id else {
        return Err(core_error!("lan connection needn't device id"));
    };

    let handshake_request_buffer = bincode_serialize(&EndPointHandshakeRequest {
        visit_credentials,
        device_id: local_device_id,
    })?;

    stream
        .send(Bytes::from(handshake_request_buffer))
        .await
        .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)?;

    let handshake_response_buffer = tokio::time::timeout(RECV_MESSAGE_TIMEOUT, stream.next())
        .await
        .map_err(|_| CoreError::Timeout)?
        .ok_or(CoreError::OutgoingMessageChannelDisconnect)??;

    let resp: EndPointHandshakeResponse = bincode_deserialize(handshake_response_buffer.deref())?;

    if resp.remote_device_id != remote_device_id {
        return Err(core_error!("endpoints server build mismatch tunnel"));
    }

    Ok(())
}

fn serve_tcp_read(
    endpoint_id: EndPointID,
    mut opening_key: Option<OpeningKey<NonceValue>>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
) -> CoreResult<tokio::sync::mpsc::Receiver<Bytes>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            let mut buffer = match stream.next().await {
                Some(packet) => match packet {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(?endpoint_id, ?err, "read stream failed");
                        break;
                    }
                },
                None => {
                    tracing::error!(?endpoint_id, "read stream is closed");
                    break;
                }
            };

            let buffer_len = if let Some(ref mut opening_key) = opening_key {
                match opening_key.open_in_place(ring::aead::Aad::empty(), buffer.as_mut()) {
                    Ok(output) => output.len(),
                    Err(err) => {
                        tracing::error!(?err, "open endpoint message packet failed");
                        break;
                    }
                }
            } else {
                buffer.len()
            };

            buffer.truncate(buffer_len);

            if tx.send(buffer.freeze()).await.is_err() {
                tracing::error!(?endpoint_id, "output channel closed");
                break;
            }
        }

        tracing::info!(?endpoint_id, "tcp read loop exit");
    });

    Ok(rx)
}

fn serve_tcp_write(
    endpoint_id: EndPointID,
    mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    mut sealing_key: Option<SealingKey<NonceValue>>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
) {
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(mut buffer) => {
                    if let Some(ref mut sealing_key) = sealing_key {
                        if let Err(err) = sealing_key
                            .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buffer)
                        {
                            tracing::error!(?err, "seal endpoint message packet failed");
                            break;
                        }
                    }

                    if sink.send(Bytes::from(buffer)).await.is_err() {
                        tracing::error!(?endpoint_id, "tcp write failed");
                        break;
                    }
                }
                None => {
                    tracing::error!(?endpoint_id, "input channel closed");
                    break;
                }
            }
        }

        tracing::info!(?endpoint_id, "tcp write loop exit");
    });
}
