use crate::provider::socket::SocketProvider;

use super::message::client_to_client::{
    ClientToClientMessage, ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
    KeyExchangeAndVerifyPasswordRequest,
};
use anyhow::bail;
use dashmap::DashMap;
use ring::aead::{BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use std::{any::Any, time::Duration};
use tokio::sync::RwLock;

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    opening_key: RwLock<Option<OpeningKey<NonceValue>>>,
    sealing_key: RwLock<Option<SealingKey<NonceValue>>>,
    cache: MemoryCache,
}

impl EndPoint {
    pub fn new(local_device_id: String, remote_device_id: String) -> Self {
        Self {
            local_device_id,
            remote_device_id,
            opening_key: RwLock::new(None),
            sealing_key: RwLock::new(None),
            cache: MemoryCache::new(),
        }
    }

    #[must_use]
    pub fn remote_device_id(&self) -> &str {
        self.remote_device_id.as_ref()
    }

    #[must_use]
    pub fn local_device_id(&self) -> &str {
        self.local_device_id.as_ref()
    }

    #[must_use]
    pub fn cache(&self) -> &MemoryCache {
        &self.cache
    }

    pub async fn set_opening_key(&self, key: UnboundKey, initial_nonce: u64) {
        let opening_key =
            ring::aead::OpeningKey::<NonceValue>::new(key, NonceValue::new(initial_nonce));
        let mut key = self.opening_key.write().await;
        *key = Some(opening_key);
    }

    pub async fn set_sealing_key(&self, key: UnboundKey, initial_nonce: u64) {
        let sealing_key =
            ring::aead::SealingKey::<NonceValue>::new(key, NonceValue::new(initial_nonce));
        let mut key = self.sealing_key.write().await;
        *key = Some(sealing_key);
    }

    pub async fn desktop_connect(
        &self,
        req: ConnectRequest,
        timeout: Duration,
    ) -> anyhow::Result<ConnectReply> {
        SocketProvider::current()?
            .call_client(
                self.local_device_id.to_owned(),
                self.remote_device_id.to_owned(),
                ClientToClientMessage::ConnectRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ClientToClientMessage::Error => bail!("desktop_connect: remote error"),
                ClientToClientMessage::ConnectReply(message) => Ok(message),
                _ => bail!("desktop_connect: mismatched reply type, got {}", resp),
            })
    }

    pub async fn desktop_key_exchange_and_verify_password(
        &self,
        req: KeyExchangeAndVerifyPasswordRequest,
        timeout: Duration,
    ) -> anyhow::Result<KeyExchangeAndVerifyPasswordReply> {
        SocketProvider::current()?
            .call_client(
                self.local_device_id.to_owned(),
                self.remote_device_id.to_owned(),
                ClientToClientMessage::KeyExchangeAndVerifyPasswordRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ClientToClientMessage::Error => {
                    bail!("desktop_key_exchange_and_verify_password: remote error")
                }
                ClientToClientMessage::KeyExchangeAndVerifyPasswordReply(message) => Ok(message),
                _ => bail!(
                    "desktop_key_exchange_and_verify_password: mismatched reply type, got {}",
                    resp
                ),
            })
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum CacheKey {
    PasswordVerifyPublicKey,
    PasswordVerifyPrivateKey,
}

pub struct MemoryCache {
    values: DashMap<CacheKey, Box<dyn Any + Send + Sync>>,
}

impl MemoryCache {
    fn new() -> Self {
        Self {
            values: DashMap::new(),
        }
    }

    pub fn set<T>(&self, key: CacheKey, value: T)
    where
        T: Any + Send + Sync,
    {
        self.values.insert(key, Box::new(value));
    }

    pub fn take<T>(&self, key: CacheKey) -> Option<T>
    where
        T: Any + Send + Sync,
    {
        match self.values.remove(&key) {
            Some(entry) => match entry.1.downcast::<T>() {
                Ok(v) => Some(*v),
                Err(_) => None,
            },
            None => None,
        }
    }
}

struct NonceValue {
    n: u128,
}

impl NonceValue {
    fn new(n: u64) -> Self {
        Self { n: n as u128 }
    }
}

impl NonceSequence for NonceValue {
    fn advance(&mut self) -> Result<ring::aead::Nonce, ring::error::Unspecified> {
        self.n += 1;
        let m = self.n & 0xFFFFFFFFFFFF;
        Nonce::try_assume_unique_for_key(&m.to_le_bytes()[..12])
    }
}
