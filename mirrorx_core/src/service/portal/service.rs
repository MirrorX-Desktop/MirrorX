use super::{handler::handle_passive_visit_request, message::*};
use crate::{
    core_error,
    error::{CoreError, CoreResult},
    service::config,
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use hmac::Hmac;
use moka::future::{Cache, CacheBuilder};
use rand::RngCore;
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use rsa::{rand_core::OsRng, PublicKeyParts};
use sha2::Sha256;
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicI64},
        Arc,
    },
    time::Duration,
};
use tokio::{
    net::TcpStream,
    select,
    sync::mpsc::{Receiver, Sender},
    time::timeout,
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use uuid::Uuid;

pub struct Service {
    domain_id: Arc<AtomicI64>,
    addr: Option<String>,
    tx: Option<Sender<Bytes>>,
    pending_call: Cache<Uuid, Sender<PortalServerMessage>>,
    running: Arc<AtomicBool>,
    config: config::service::Service,
}

impl Service {
    pub fn new(config: config::service::Service) -> Self {
        Service {
            domain_id: Arc::new(AtomicI64::new(0)),
            addr: None,
            tx: None,
            pending_call: CacheBuilder::new(8)
                .time_to_live(Duration::from_secs(30))
                .build(),
            running: Arc::new(AtomicBool::new(false)),
            config,
        }
    }

    pub fn domain_id(&self) -> i64 {
        self.domain_id.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set_domain_id(&self, domain_id: i64) {
        self.domain_id
            .store(domain_id, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn connect<F>(
        &mut self,
        domain_id: i64,
        addr: String,
        visit_callback: F,
    ) -> CoreResult<()>
    where
        F: Send + Sync + Clone + 'static + Fn(i64, i64, bool) -> bool,
    {
        if self.is_running() {
            return Err(core_error!("repeat connect"));
        }

        let stream = timeout(Duration::from_secs(5), TcpStream::connect(&addr))
            .await
            .map_err(|_| CoreError::Timeout)??;

        stream.set_nodelay(true)?;

        let stream = LengthDelimitedCodec::builder()
            .little_endian()
            .length_field_length(2)
            .max_frame_length(1000)
            .new_framed(stream);

        let (tx, rx) = tokio::sync::mpsc::channel(8);

        self.domain_id
            .store(domain_id, std::sync::atomic::Ordering::SeqCst);
        self.addr = Some(addr);
        self.tx = Some(tx.clone());
        self.running.swap(true, std::sync::atomic::Ordering::SeqCst);
        self.serve_call(visit_callback, tx, rx, stream);

        Ok(())
    }

    pub async fn get_server_config(&mut self) -> CoreResult<ServerConfigReply> {
        let reply = self
            .call(
                &PortalClientMessage::ServerConfigRequest,
                Duration::from_secs(10),
            )
            .await?;

        let PortalServerMessage::ServerConfigReply(server_config) = reply else {
            return Err(CoreError::PortalCallError(PortalError::Internal));
        };

        Ok(server_config)
    }

    pub async fn client_register(
        &mut self,
        device_id: i64,
        device_finger_print: &str,
    ) -> CoreResult<ClientRegisterReply> {
        let reply = self
            .call(
                &PortalClientMessage::ClientRegisterRequest(ClientRegisterRequest {
                    device_id,
                    device_finger_print: device_finger_print.into(),
                }),
                Duration::from_secs(10),
            )
            .await?;

        let PortalServerMessage::ClientRegisterReply(result) = reply else {
            return Err(CoreError::PortalCallError(PortalError::Internal));
        };

        Ok(result)
    }

    pub async fn check_remote_device_is_online(&mut self, device_id: i64) -> CoreResult<bool> {
        let reply = self
            .call(
                &PortalClientMessage::CheckRemoteDeviceIsOnlineRequest(
                    CheckRemoteDeviceIsOnlineRequest { device_id },
                ),
                Duration::from_secs(10),
            )
            .await?;

        let PortalServerMessage::CheckRemoteDeviceIsOnlineReply(is_online) = reply else {
            return Err(CoreError::PortalCallError(PortalError::Internal));
        };

        Ok(is_online)
    }

    pub async fn visit(
        &mut self,
        local_device_id: i64,
        remote_device_id: i64,
        password: String,
        visit_desktop: bool,
    ) -> CoreResult<(
        SocketAddr,
        String,
        OpeningKey<NonceValue>,
        SealingKey<NonceValue>,
    )> {
        let secure_random = ring::rand::SystemRandom::new();

        // generate key pair for passive device key exchange reply
        let reply_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096)?;
        let reply_public_key = reply_private_key.to_public_key();

        // generate exchange key pair and nonce
        let active_exchange_private_key = ring::agreement::EphemeralPrivateKey::generate(
            &ring::agreement::X25519,
            &secure_random,
        )?;
        let active_exchange_public_key = active_exchange_private_key.compute_public_key()?;

        let mut active_exchange_nonce = [0u8; ring::aead::NONCE_LEN];
        OsRng.fill_bytes(&mut active_exchange_nonce);

        let mut visit_credentials_buffer = [0u8; 16];
        OsRng.fill_bytes(&mut visit_credentials_buffer);

        // generate and sealing active device key exchange secret
        let active_device_secret = ActiveEndpointKeyExchangeSecret {
            exchange_reply_public_key_n: &reply_public_key.n().to_bytes_le(),
            exchange_reply_public_key_e: &reply_public_key.e().to_bytes_le(),
            active_exchange_public_key: active_exchange_public_key.as_ref(),
            active_exchange_nonce: &active_exchange_nonce,
        };

        // generate secret sealing key with salt
        let mut active_device_secret_salt = [0u8; 16];
        OsRng.fill_bytes(&mut active_device_secret_salt);

        let mut active_device_secret_sealing_key = [0u8; 32];
        pbkdf2::pbkdf2::<Hmac<Sha256>>(
            password.as_bytes(),
            &active_device_secret_salt,
            10000,
            &mut active_device_secret_sealing_key,
        );

        let mut active_device_secret_buffer = bincode_serialize(&active_device_secret)?;

        let active_device_secret_sealing_unbound_key = ring::aead::UnboundKey::new(
            &ring::aead::AES_256_GCM,
            &active_device_secret_sealing_key,
        )?;

        let mut active_device_secret_sealing_nonce = [0u8; ring::aead::NONCE_LEN];
        OsRng.fill_bytes(&mut active_device_secret_sealing_nonce);

        let mut active_device_secret_sealing_key = ring::aead::SealingKey::new(
            active_device_secret_sealing_unbound_key,
            NonceValue::new(active_device_secret_sealing_nonce),
        );

        active_device_secret_sealing_key.seal_in_place_append_tag(
            ring::aead::Aad::from(local_device_id.to_le_bytes()),
            &mut active_device_secret_buffer,
        )?;

        // visit call
        let reply = self
            .call(
                &PortalClientMessage::ActiveVisitRequest(ActiveVisitRequest {
                    active_device_id: local_device_id,
                    passive_device_id: remote_device_id,
                    visit_desktop,
                    password_salt: active_device_secret_salt.to_vec(),
                    secret: active_device_secret_buffer,
                    secret_nonce: active_device_secret_sealing_nonce.to_vec(),
                }),
                Duration::from_secs(60),
            )
            .await?;

        let reply = match reply {
            PortalServerMessage::Error(err) => return Err(CoreError::PortalCallError(err)),
            PortalServerMessage::ActiveVisitReply(reply) => reply,
            _ => return Err(CoreError::PortalCallError(PortalError::RemoteInternal)),
        };

        let relay_addr = reply.relay_addr;
        let visit_credentials = reply.passive_reply.visit_credentials;
        let secret = match reply.passive_reply.access_result {
            Ok(secret) => secret,
            Err(err) => return Err(CoreError::PortalCallError(err)),
        };

        // compute open and seal key
        let passive_device_secret_buffer =
            reply_private_key.decrypt(rsa::Pkcs1v15Encrypt::default(), &secret)?;

        let passive_device_secret: PassiveEndpointKeyExchangeSecret =
            bincode_deserialize(&passive_device_secret_buffer)?;

        let passive_exchange_public_key = ring::agreement::UnparsedPublicKey::new(
            &ring::agreement::X25519,
            passive_device_secret.passive_exchange_public_key,
        );

        let (raw_sealing_key, raw_opening_key) = ring::agreement::agree_ephemeral(
            active_exchange_private_key,
            &passive_exchange_public_key,
            ring::error::Unspecified,
            |key_material| {
                let sealing_key =
                    ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &active_exchange_nonce)
                        .extract(key_material)
                        .expand(&["".as_bytes()], &ring::aead::AES_256_GCM)
                        .and_then(|orm| {
                            let mut key = Vec::<u8>::new();
                            key.resize(ring::aead::AES_256_GCM.key_len(), 0);
                            orm.fill(&mut key)?;
                            Ok(key)
                        })?;

                let opening_key = ring::hkdf::Salt::new(
                    ring::hkdf::HKDF_SHA512,
                    passive_device_secret.passive_exchange_nonce,
                )
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::AES_256_GCM)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::new();
                    key.resize(ring::aead::AES_256_GCM.key_len(), 0);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

                Ok((sealing_key, opening_key))
            },
        )?;

        let unbound_sealing_key =
            ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)?;

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(passive_device_secret.passive_exchange_nonce);
        let sealing_key = ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(nonce));

        let unbound_opening_key =
            ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)?;

        let mut nonce = [0u8; 12];
        nonce.copy_from_slice(&active_exchange_nonce);
        let opening_key = ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(nonce));

        Ok((relay_addr, visit_credentials, opening_key, sealing_key))
    }

    async fn call(
        &mut self,
        request: &PortalClientMessage,
        timeout_duration: Duration,
    ) -> CoreResult<PortalServerMessage> {
        if !self.is_running() {
            return Err(core_error!("portal service is disconnected"));
        }

        let Some(ref tx) = self.tx else {
            return Err(core_error!("portal service is disconnected"));
        };

        let request_id = Uuid::new_v4();
        let request_bytes = bincode_serialize(&(request_id, request))?;
        if tx
            .send_timeout(Bytes::from(request_bytes), Duration::from_secs(1))
            .await
            .is_err()
        {
            return Err(core_error!("portal service is disconnected"));
        }

        let (reply_tx, mut reply_rx) = tokio::sync::mpsc::channel(1);
        self.pending_call.insert(request_id, reply_tx).await;

        let res = timeout(timeout_duration, reply_rx.recv()).await;
        self.pending_call.invalidate(&request_id).await;

        let server_message = res
            .map_err(|_| CoreError::Timeout)?
            .ok_or(CoreError::Timeout)?;

        Ok(server_message)
    }

    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip_all)]
    fn serve_call<F>(
        &self,
        visit_callback: F,
        tx: Sender<Bytes>,
        mut rx: Receiver<Bytes>,
        mut stream: Framed<TcpStream, LengthDelimitedCodec>,
    ) where
        F: Send + Sync + Clone + 'static + Fn(i64, i64, bool) -> bool,
    {
        let running = self.running.clone();
        let domain_id = self.domain_id.clone();
        let config = self.config.clone();
        let pending_call = self.pending_call.clone();

        tokio::spawn(async move {
            loop {
                select! {
                    send_buffer = rx.recv() => match send_buffer {
                        Some(buffer) => {
                            if let Err(err) = stream.send(buffer).await {
                                tracing::error!(?err, "send buffer failed");
                                break;
                            }

                            continue;
                        },
                        None => {
                            tracing::error!("send channel closed");
                            break;
                        }
                    },
                    recv_buffer = stream.next() => match recv_buffer {
                        Some(buffer) => match buffer {
                            Ok(buffer) => {
                                let (request_id, server_message): (Uuid, PortalServerMessage) = match bincode_deserialize(buffer.as_ref()) {
                                    Ok(request_message) => request_message,
                                    Err(err) => {
                                        tracing::error!(?err, "deserialize portal server message failed");
                                        continue;
                                    }
                                };

                                match server_message {
                                    PortalServerMessage::VisitPassiveRequest(req) => {
                                        let visit_callback_clone = visit_callback.clone();
                                        let config_clone = config.clone();
                                        let tx_clone = tx.clone();
                                        let domain_id = domain_id.clone();
                                        tokio::spawn(async move {
                                            let client_message = if visit_callback_clone(
                                                req.active_visit_req.active_device_id,
                                                req.active_visit_req.passive_device_id,
                                                req.active_visit_req.visit_desktop,
                                            ) {
                                                handle_passive_visit_request(domain_id, config_clone, req).await
                                            } else {
                                                PortalClientMessage::Error(PortalError::RemoteRefuse)
                                            };

                                            let reply_bytes = match bincode_serialize(&(request_id, client_message)) {
                                                Ok(reply_bytes) => reply_bytes,
                                                Err(err) => {
                                                    tracing::error!(?err, "serialize portal client message failed");
                                                    return;
                                                }
                                            };

                                            let _ = tx_clone.send(Bytes::from(reply_bytes)).await;
                                        });
                                    },
                                    req => {
                                        if let Some(tx) = pending_call.get(&request_id) {
                                            let _ = tx.send(req).await;
                                            pending_call.invalidate(&request_id).await;
                                        }
                                    }
                                };
                            },
                            Err(err) => {
                                tracing::error!(?err, "read recv buffer failed");
                                break;
                            }
                        },
                        None => {
                            tracing::error!("recv buffer failed");
                            break;
                        }
                    }
                };
            }

            running.swap(false, std::sync::atomic::Ordering::SeqCst);
            tracing::info!("portal service handle loop exit");
        });
    }
}
