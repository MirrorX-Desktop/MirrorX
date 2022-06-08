use crate::{provider::socket::SocketProvider, utility::serializer::BINCODE_SERIALIZER};

use super::{
    message::client_to_client::{
        ClientToClientMessage, ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
        KeyExchangeAndVerifyPasswordRequest, StartMediaTransmissionReply,
        StartMediaTransmissionRequest,
    },
    packet::Packet,
};
use anyhow::bail;
use bincode::Options;
use dashmap::DashMap;
use ring::aead::{BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey};
use std::{any::Any, time::Duration};
use tokio::sync::Mutex;

pub struct EndPoint {
    local_device_id: String,
    remote_device_id: String,
    opening_key: Mutex<Option<OpeningKey<NonceValue>>>,
    sealing_key: Mutex<Option<SealingKey<NonceValue>>>,
    cache: MemoryCache,
}

impl EndPoint {
    pub fn new(local_device_id: String, remote_device_id: String) -> Self {
        Self {
            local_device_id,
            remote_device_id,
            opening_key: Mutex::new(None),
            sealing_key: Mutex::new(None),
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
        let mut key = self.opening_key.lock().await;
        *key = Some(opening_key);
    }

    pub async fn set_sealing_key(&self, key: UnboundKey, initial_nonce: u64) {
        let sealing_key =
            ring::aead::SealingKey::<NonceValue>::new(key, NonceValue::new(initial_nonce));
        let mut key = self.sealing_key.lock().await;
        *key = Some(sealing_key);
    }

    pub async fn secure_seal(&self, message: ClientToClientMessage) -> anyhow::Result<()> {
        let mut buf = BINCODE_SERIALIZER.serialize(&message)?;
        let mut sealing_key = self.sealing_key.lock().await;
        match sealing_key
            .as_mut()
            .and_then(|key| Some(key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut buf)))
        {
            Some(res) => {
                if let Err(err) = res {
                    bail!("secure_send: sealing message failed: {}", err);
                }
            }
            None => bail!("secure_send: sealing key is not set"),
        };

        SocketProvider::current()?
            .send(Packet::ClientToClient(
                0,
                self.local_device_id.clone(),
                self.remote_device_id.clone(),
                true,
                buf,
            ))
            .await
    }

    pub async fn secure_open(&self, buf: &mut [u8]) -> anyhow::Result<()> {
        match self
            .opening_key
            .lock()
            .await
            .as_mut()
            .and_then(|key| Some(key.open_in_place(ring::aead::Aad::empty(), buf)))
        {
            Some(res) => match res {
                Ok(_) => Ok(()),
                Err(err) => bail!("secure_open: opening message failed: {}", err),
            },
            None => bail!("secure_open: opening key is not set"),
        }
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

    pub async fn desktop_start_media_transmission(
        &self,
        req: StartMediaTransmissionRequest,
        timeout: Duration,
    ) -> anyhow::Result<StartMediaTransmissionReply> {
        SocketProvider::current()?
            .call_client(
                self.local_device_id.to_owned(),
                self.remote_device_id.to_owned(),
                ClientToClientMessage::StartMediaTransmissionRequest(req),
                timeout,
            )
            .await
            .and_then(|resp| match resp {
                ClientToClientMessage::Error => {
                    bail!("desktop_start_media_transmission: remote error")
                }
                ClientToClientMessage::StartMediaTransmissionReply(message) => Ok(message),
                _ => bail!(
                    "desktop_start_media_transmission: mismatched reply type, got {}",
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
