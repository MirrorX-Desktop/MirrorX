use super::packet::{Packet, ReplyPacket, RequestPacket};
use crate::{
    instance::BINCODE_INSTANCE,
    provider::service::message::{
        reply::ReplyMessage, reply_error::ReplyError, request::RequestMessage,
    },
};
use bincode::Options;
use dashmap::DashMap;
use log::error;
use ring::agreement::EphemeralPrivateKey;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::{
    sync::atomic::{AtomicU8, Ordering},
    time::Duration,
};
use tokio::sync::{mpsc, RwLock};

pub struct Client {
    tx: mpsc::Sender<Vec<u8>>,
    call_tx_map: DashMap<u8, mpsc::Sender<ReplyPacket>>,
    device_id: RwLock<String>,
    call_id: AtomicU8,
    password_authorize_pub_key: DashMap<String, RsaPublicKey>,
    key_exchange_priv_key: DashMap<String, RsaPrivateKey>,
}

impl Client {
    pub fn new(tx: mpsc::Sender<Vec<u8>>) -> Self {
        Client {
            tx,
            call_tx_map: DashMap::new(),
            device_id: RwLock::new(String::new()),
            call_id: AtomicU8::new(1),
            password_authorize_pub_key: DashMap::new(),
            key_exchange_priv_key: DashMap::new(),
        }
    }

    pub fn device_id(&self) -> String {
        self.device_id.blocking_read().clone()
    }

    pub fn set_device_id(&self, device_id: String) {
        self.device_id.blocking_write().clone_from(&device_id)
    }

    pub async fn call(
        &self,
        request: RequestMessage,
        timeout: Duration,
    ) -> Result<ReplyMessage, ReplyError> {
        let call_id = self.next_call_id();

        let packet = Packet {
            request_packet: Some(RequestPacket {
                call_id,
                payload: request,
            }),
            reply_packet: None,
        };

        let mut rx = self.register_call(call_id);

        self.send(packet).await.map_err(|err| {
            error!("client call failed: {:?}", err);
            self.remove_call(call_id);
            ReplyError::Internal
        })?;

        match tokio::time::timeout(timeout, rx.recv()).await {
            Ok(res) => match res {
                Some(reply_packet) => reply_packet.payload,
                None => Err(ReplyError::Internal),
            },
            Err(_) => {
                self.remove_call(call_id);
                Err(ReplyError::Timeout)
            }
        }
    }

    pub fn reply_call(&self, call_id: u8, reply_packet: ReplyPacket) {
        self.remove_call(call_id).map(|tx| {
            if let Err(err) = tx.try_send(reply_packet) {
                error!(
                    "client[{:?}] reply call failed: {:?}",
                    self.device_id(),
                    err
                )
            }
        });
    }

    pub async fn reply_request(&self, reply_packet: ReplyPacket) -> Result<(), ReplyError> {
        let packet = Packet {
            request_packet: None,
            reply_packet: Some(reply_packet),
        };

        self.send(packet).await.map_err(|err| {
            error!("reply_request: {:?}", err);
            ReplyError::Internal
        })
    }

    pub fn store_verify_password_pub_key(&self, device_id: String, pub_key: RsaPublicKey) {
        self.password_authorize_pub_key.insert(device_id, pub_key);
    }

    pub fn remove_verify_password_pub_key(&self, device_id: &str) -> Option<RsaPublicKey> {
        self.password_authorize_pub_key
            .remove(device_id)
            .and_then(|entry| Some(entry.1))
    }

    pub fn store_verify_password_priv_key(&self, device_id: String, priv_key: RsaPrivateKey) {
        self.key_exchange_priv_key.insert(device_id, priv_key);
    }

    pub fn remove_verify_password_priv_key(&self, device_id: &str) -> Option<RsaPrivateKey> {
        self.key_exchange_priv_key
            .remove(device_id)
            .and_then(|entry| Some(entry.1))
    }

    async fn send(&self, packet: Packet) -> anyhow::Result<()> {
        let buf = BINCODE_INSTANCE.serialize(&packet)?;
        self.tx.send_timeout(buf, Duration::from_secs(1)).await?;
        Ok(())
    }

    fn next_call_id(&self) -> u8 {
        loop {
            let new_call_id = self.call_id.fetch_add(1, Ordering::AcqRel);
            if new_call_id == 0 {
                continue;
            }

            return new_call_id;
        }
    }

    fn register_call(&self, call_id: u8) -> mpsc::Receiver<ReplyPacket> {
        let (tx, rx) = mpsc::channel(1);
        self.call_tx_map.insert(call_id, tx);
        rx
    }

    fn remove_call(&self, call_id: u8) -> Option<mpsc::Sender<ReplyPacket>> {
        self.call_tx_map.remove(&call_id).map(|entry| entry.1)
    }
}
