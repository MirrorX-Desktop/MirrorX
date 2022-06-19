use crate::{
    error::anyhow::Result, socket::endpoint::endpoint::EndPoint, utility::nonce_value::NonceValue,
};
use dashmap::DashMap;
use ring::aead::{OpeningKey, SealingKey};
use std::sync::Arc;

pub async fn connect(
    local_device_id: String,
    remote_device_id: String,
    sealing_key: SealingKey<NonceValue>,
    opening_key: OpeningKey<NonceValue>,
) -> anyhow::Result<()> {
    let endpoint = EndPoint::connect(
        "192.168.0.101:28001",
        local_device_id,
        remote_device_id.to_owned(),
        opening_key,
        sealing_key,
    )
    .await?;

    // ENDPOINTS.insert(remote_device_id, Arc::new(endpoint));

    Ok(())
}
