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
use futures::{SinkExt, StreamExt};
use ring::aead::{BoundKey, OpeningKey, SealingKey, UnboundKey};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub struct HandshakeRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub visit_credentials: String,
    pub opening_key_bytes: Vec<u8>,
    pub opening_nonce_bytes: Vec<u8>,
    pub sealing_key_bytes: Vec<u8>,
    pub sealing_nonce_bytes: Vec<u8>,
}

// pub struct HandshakeResponse {}

pub async fn active_device_handshake(req: HandshakeRequest) -> CoreResult<()> {
    let mut opening_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        opening_nonce[i] = req.opening_nonce_bytes[i];
    }

    let mut sealing_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        sealing_nonce[i] = req.sealing_nonce_bytes[i];
    }

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
    )
    .await
}

pub async fn passive_device_handshake(
    active_device_id: String,
    passive_device_id: String,
    visit_credentials: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> CoreResult<()> {
    inner_handshake(
        active_device_id,
        passive_device_id,
        visit_credentials,
        opening_key,
        sealing_key,
    )
    .await
}

async fn inner_handshake(
    active_device_id: String,
    passive_device_id: String,
    visit_credentials: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> CoreResult<()> {
    let entry = RESERVE_STREAMS
        .remove(&(active_device_id.to_owned(), passive_device_id.to_owned()))
        .ok_or(core_error!(
            "no stream exists in RESERVE_STREAMS with key ({},{})",
            &active_device_id,
            &passive_device_id
        ))?;

    let mut stream = entry.1;

    let handshake_req = EndPointHandshakeRequest {
        active_device_id: active_device_id.to_owned(),
        passive_device_id: passive_device_id.to_owned(),
        visit_credentials: visit_credentials,
    };

    let handshake_resp: EndPointHandshakeResponse = stream_call(&mut stream, handshake_req).await?;

    let (exit_tx, exit_rx) = async_broadcast::broadcast(16);
    let (send_message_tx, send_message_rx) = tokio::sync::mpsc::channel(1);
    let (sink, stream) = stream.split();

    super::super::serve_reader(
        active_device_id.to_owned(),
        passive_device_id.to_owned(),
        exit_tx.clone(),
        exit_tx.new_receiver(),
        stream,
        opening_key,
        send_message_tx,
    );

    super::super::serve_writer(
        active_device_id,
        passive_device_id,
        exit_tx.clone(),
        exit_tx.new_receiver(),
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
