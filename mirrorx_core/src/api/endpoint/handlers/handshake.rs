use crate::{
    api::endpoint::{
        message::{EndPointHandshakeRequest, EndPointHandshakeResponse},
        RESERVE_STREAMS,
    },
    core_error,
    error::{CoreError, CoreResult},
    utility::{nonce_value::NonceValue, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use bytes::{Buf, Bytes};
use flutter_rust_bridge::{StreamSink, ZeroCopyBuffer};
use futures::{SinkExt, StreamExt};
use ring::aead::{BoundKey, OpeningKey, SealingKey, UnboundKey};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub struct HandshakeRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub visit_credentials: String,
    pub opening_key_bytes: Vec<u8>,
    pub opening_nonce_bytes: Vec<u8>,
    pub sealing_key_bytes: Vec<u8>,
    pub sealing_nonce_bytes: Vec<u8>,
}

#[derive(Clone)]
pub enum EndPointMediaMessage {
    Video(i64, i64, ZeroCopyBuffer<Vec<u8>>),
    Audio(i64, i64, ZeroCopyBuffer<Vec<u8>>),
}

pub async fn active_device_handshake(
    req: HandshakeRequest,
    stream: StreamSink<EndPointMediaMessage>,
) -> CoreResult<()> {
    let mut opening_nonce = [0u8; ring::aead::NONCE_LEN];
    opening_nonce[0..ring::aead::NONCE_LEN]
        .copy_from_slice(&req.opening_nonce_bytes[0..ring::aead::NONCE_LEN]);

    let mut sealing_nonce = [0u8; ring::aead::NONCE_LEN];
    sealing_nonce[0..ring::aead::NONCE_LEN]
        .copy_from_slice(&req.sealing_nonce_bytes[0..ring::aead::NONCE_LEN]);

    let unbound_sealing_key = UnboundKey::new(&ring::aead::AES_256_GCM, &req.sealing_key_bytes)?;
    let sealing_key = SealingKey::new(unbound_sealing_key, NonceValue::new(sealing_nonce));

    let unbound_opening_key = UnboundKey::new(&ring::aead::AES_256_GCM, &req.opening_key_bytes)?;
    let opening_key = OpeningKey::new(unbound_opening_key, NonceValue::new(opening_nonce));

    inner_handshake(
        req.active_device_id,
        req.passive_device_id,
        req.visit_credentials,
        opening_key,
        sealing_key,
        Some(stream),
    )
    .await
}

pub async fn passive_device_handshake(
    local_device_id: i64,
    remote_device_id: i64,
    visit_credentials: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> CoreResult<()> {
    inner_handshake(
        local_device_id,
        remote_device_id,
        visit_credentials,
        opening_key,
        sealing_key,
        None,
    )
    .await
}

async fn inner_handshake(
    local_device_id: i64,
    remote_device_id: i64,
    visit_credentials: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
    flutter_stream: Option<StreamSink<EndPointMediaMessage>>,
) -> CoreResult<()> {
    let entry = RESERVE_STREAMS
        .remove(&(local_device_id, remote_device_id))
        .ok_or(core_error!(
            "no stream exists in RESERVE_STREAMS with key ({},{})",
            &local_device_id,
            &remote_device_id
        ))?;

    let mut stream = entry.1;

    let handshake_req = EndPointHandshakeRequest {
        device_id: local_device_id,
        visit_credentials,
    };

    let handshake_resp: EndPointHandshakeResponse = stream_call(&mut stream, handshake_req).await?;
    if handshake_resp.remote_device_id != remote_device_id {
        return Err(core_error!(
            "signaling server matched incorrect stream pair"
        ));
    }

    let (exit_tx, exit_rx) = async_broadcast::broadcast(16);
    let (send_message_tx, send_message_rx) = tokio::sync::mpsc::channel(1);
    let (sink, stream) = stream.split();

    super::super::serve_reader(
        local_device_id,
        remote_device_id,
        exit_tx.clone(),
        exit_rx.clone(),
        stream,
        opening_key,
        send_message_tx,
        flutter_stream,
    );

    super::super::serve_writer(
        local_device_id,
        remote_device_id,
        exit_tx,
        exit_rx,
        sink,
        sealing_key,
        send_message_rx,
    );

    Ok(())
}

async fn stream_call<Request, Reply>(
    stream: &mut Framed<TcpStream, LengthDelimitedCodec>,
    req: Request,
) -> CoreResult<Reply>
where
    Request: serde::Serialize,
    Reply: serde::de::DeserializeOwned,
{
    let req_buffer = Bytes::from(BINCODE_SERIALIZER.serialize(&req)?);

    stream.send(req_buffer).await?;
    let resp_buffer = tokio::time::timeout(Duration::from_secs(60), stream.next())
        .await?
        .ok_or(core_error!("stream was closed"))?
        .map_err(|err| core_error!("stream read failed ({})", err))?;

    let resp: Reply = BINCODE_SERIALIZER.deserialize_from(resp_buffer.reader())?;
    Ok(resp)
}
