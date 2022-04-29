use super::message::client_to_client::{
    ClientToClientMessage, ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
    KeyExchangeAndVerifyPasswordRequest,
};
use crate::instance::STREAMER_INSTANCE;
use anyhow::bail;
use dashmap::DashMap;
use std::{any::Any, time::Duration};

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    cache: MemoryCache,
}

impl EndPoint {
    pub fn new(local_device_id: String, remote_device_id: String) -> Self {
        Self {
            local_device_id,
            remote_device_id,
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

    pub async fn desktop_connect(
        &self,
        req: ConnectRequest,
        timeout: Duration,
    ) -> anyhow::Result<ConnectReply> {
        STREAMER_INSTANCE
            .call_client(
                self.local_device_id.to_owned(),
                self.remote_device_id.to_owned(),
                ClientToClientMessage::ConnectRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ClientToClientMessage::ConnectReply(message) => Ok(message),
                _ => bail!("desktop_connect: mismatched reply type, got {:?}", resp),
            })
    }

    pub async fn desktop_key_exchange_and_verify_password(
        &self,
        req: KeyExchangeAndVerifyPasswordRequest,
        timeout: Duration,
    ) -> anyhow::Result<KeyExchangeAndVerifyPasswordReply> {
        STREAMER_INSTANCE
            .call_client(
                self.local_device_id.to_owned(),
                self.remote_device_id.to_owned(),
                ClientToClientMessage::KeyExchangeAndVerifyPasswordRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ClientToClientMessage::KeyExchangeAndVerifyPasswordReply(message) => Ok(message),
                _ => bail!(
                    "desktop_key_exchange_and_verify_password: mismatched reply type, got {:?}",
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
