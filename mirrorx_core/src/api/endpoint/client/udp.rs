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
use std::{net::SocketAddr, ops::Deref};
use tokio::{net::UdpSocket, sync::mpsc::Sender};
use tokio_util::{codec::LengthDelimitedCodec, udp::UdpFramed};

pub async fn serve_udp(
    socket: UdpSocket,
    endpoint_id: EndPointID,
    sealing_key: Option<SealingKey<NonceValue>>,
    opening_key: Option<OpeningKey<NonceValue>>,
    mut visit_credentials: Option<Vec<u8>>,
) -> CoreResult<(Sender<Vec<u8>>, tokio::sync::mpsc::Receiver<Bytes>)> {
    let remote_addr = socket.peer_addr()?;
    let mut framed = UdpFramed::new(
        socket,
        LengthDelimitedCodec::builder()
            .little_endian()
            .max_frame_length(32 * 1024 * 1024)
            .new_codec(),
    );

    if let Some(visit_credentials) = visit_credentials.take() {
        serve_udp_handshake(remote_addr, &mut framed, visit_credentials, endpoint_id).await?;
    }

    let (tx, rx) = tokio::sync::mpsc::channel(32);
    let (sink, stream) = framed.split();
    serve_udp_write(remote_addr, rx, sealing_key, sink);
    let rx = serve_udp_read(remote_addr, opening_key, stream)?;
    Ok((tx, rx))
}

async fn serve_udp_handshake(
    remote_addr: SocketAddr,
    stream: &mut UdpFramed<LengthDelimitedCodec>,
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
        .send((Bytes::from(handshake_request_buffer), remote_addr))
        .await
        .map_err(|_| CoreError::OutgoingMessageChannelDisconnect)?;

    // should we try receive 3 or more times because udp is connect less?
    let (handshake_response_buffer, response_remote_addr) =
        tokio::time::timeout(RECV_MESSAGE_TIMEOUT, stream.next())
            .await
            .map_err(|_| CoreError::Timeout)?
            .ok_or(CoreError::OutgoingMessageChannelDisconnect)??;

    if response_remote_addr != remote_addr {
        return Err(core_error!("unexpected handshake reply addr"));
    }

    let resp: EndPointHandshakeResponse = bincode_deserialize(handshake_response_buffer.deref())?;

    if resp.remote_device_id != remote_device_id {
        return Err(core_error!("endpoints server build mismatch tunnel"));
    }

    Ok(())
}

fn serve_udp_read(
    remote_addr: SocketAddr,
    mut opening_key: Option<OpeningKey<NonceValue>>,
    mut stream: SplitStream<UdpFramed<LengthDelimitedCodec>>,
) -> CoreResult<tokio::sync::mpsc::Receiver<Bytes>> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        loop {
            let mut buffer = match stream.next().await {
                Some(packet) => match packet {
                    Ok((buffer, addr)) => {
                        if addr != remote_addr {
                            continue;
                        }

                        buffer
                    }
                    Err(err) => {
                        tracing::error!(?remote_addr, ?err, "read stream failed");
                        break;
                    }
                },
                None => {
                    tracing::error!(?remote_addr, "read stream is closed");
                    break;
                }
            };

            if let Some(ref mut opening_key) = opening_key {
                if let Err(err) =
                    opening_key.open_in_place(ring::aead::Aad::empty(), buffer.as_mut())
                {
                    tracing::error!(?err, "open endpoint message packet failed");
                    break;
                }
            }

            if tx.send(buffer.freeze()).await.is_err() {
                tracing::error!(?remote_addr, "output channel closed");
                break;
            }
        }

        tracing::info!(?remote_addr, "tcp read loop exit");
    });

    Ok(rx)
}

fn serve_udp_write(
    remote_addr: SocketAddr,
    mut rx: tokio::sync::mpsc::Receiver<Vec<u8>>,
    mut sealing_key: Option<SealingKey<NonceValue>>,
    mut sink: SplitSink<UdpFramed<LengthDelimitedCodec>, (Bytes, SocketAddr)>,
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

                    if sink.send((Bytes::from(buffer), remote_addr)).await.is_err() {
                        tracing::error!(?remote_addr, "tcp write failed");
                        break;
                    }
                }
                None => {
                    tracing::error!(?remote_addr, "input channel closed");
                    break;
                }
            }
        }

        tracing::info!(?remote_addr, "tcp write loop exit");
    });
}
