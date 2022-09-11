use super::RESERVE_ENDPOINTS;
use crate::{error::CoreResult, utility::nonce_value::NonceValue};
use ring::aead::BoundKey;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::LengthDelimitedCodec;

pub struct ConnectRequest {
    pub active_device_id: String,
    pub passive_device_id: String,
    pub addr: String,
    pub opening_key_bytes: Vec<u8>,
    pub opening_nonce_bytes: Vec<u8>,
    pub sealing_key_bytes: Vec<u8>,
    pub sealing_nonce_bytes: Vec<u8>,
}

pub struct ConnectResponse {}

pub async fn connect(req: ConnectRequest) -> CoreResult<ConnectResponse> {
    let stream =
        tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(req.addr)).await??;

    stream.set_nodelay(true)?;

    let stream = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(32 * 1024 * 1024)
        .new_framed(stream);

    let mut opening_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        opening_nonce[i] = req.opening_nonce_bytes[i];
    }

    let mut sealing_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        sealing_nonce[i] = req.sealing_nonce_bytes[i];
    }

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &req.sealing_key_bytes)?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(sealing_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &req.opening_key_bytes)?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(opening_nonce));

    RESERVE_ENDPOINTS.insert(
        (
            req.active_device_id.to_owned(),
            req.passive_device_id.to_owned(),
        ),
        (stream, opening_key, sealing_key),
    );

    Ok(ConnectResponse {})
}
// (
//     SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
//     SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
// )
