use crate::{
    error::MirrorXResult, socket::endpoint::endpoint::EndPoint, utility::nonce_value::NonceValue,
};
use dashmap::DashMap;
use ring::aead::{OpeningKey, SealingKey};
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

static ENDPOINTS: DashMap<String, Arc<EndPoint>> = DashMap::new();

pub async fn connect<A>(
    addr: A,
    active_device_id: String,
    passive_device_id: String,
    sealing_key: SealingKey<NonceValue>,
    opening_key: OpeningKey<NonceValue>,
) -> MirrorXResult<()>
where
    A: ToSocketAddrs,
{
    let endpoint = EndPoint::connect(
        addr,
        active_device_id,
        passive_device_id.to_owned(),
        opening_key,
        sealing_key,
    )
    .await?;

    ENDPOINTS.insert(passive_device_id, Arc::new(endpoint));

    Ok(())
}
