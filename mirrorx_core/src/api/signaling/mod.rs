pub mod http_message;
pub mod subscribe_message;

use self::{
    http_message::{
        IdentityResponse, RegisterRequest, RegisterResponse, Response, VisitRequest, VisitResponse,
    },
    subscribe_message::{
        ActiveEndpointKeyExchangeSecret, ClientMessage, PassiveEndpointKeyExchangeSecret,
        ServerMessage, Subscription, VisitFailureReason,
    },
};
use super::{
    config::LocalStorage,
    endpoint::{create_passive_endpoint_client, id::EndPointID},
};
use crate::{
    core_error,
    error::CoreResult,
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
        rand::generate_random_ping_value,
    },
};
use base64::engine::general_purpose::STANDARD as base64_standard;
use base64::Engine;
use bytes::Bytes;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use hmac::Hmac;
use rand::RngCore;
use reqwest::IntoUrl;
use ring::aead::{BoundKey, OpeningKey, SealingKey, UnboundKey};
use rsa::{rand_core::OsRng, BigUint, PublicKey, PublicKeyParts};
use sha2::Sha256;
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use url::Url;

pub struct SignalingClient {
    url: Url,
    http_client: reqwest::Client,
    subscribe_tx: Option<tokio::sync::mpsc::Sender<Bytes>>,
}

impl SignalingClient {
    pub fn new<U: IntoUrl>(domain: U) -> CoreResult<Self> {
        let url = domain.into_url()?;

        let http_client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self {
            url,
            http_client,
            subscribe_tx: None,
        })
    }

    #[tracing::instrument(skip(self))]
    pub async fn identity(&self) -> CoreResult<Response<IdentityResponse>> {
        let url = self.url.join("/api/identity")?;
        let resp = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Response<IdentityResponse>>()
            .await?;

        Ok(resp)
    }

    #[tracing::instrument(skip(self))]
    pub async fn domain_register(
        &self,
        device_id: i64,
        device_finger_print: &str,
    ) -> CoreResult<Response<RegisterResponse>> {
        let url = self.url.join("/api/domain/register")?;
        let resp = self
            .http_client
            .post(url)
            .json(&RegisterRequest {
                device_id,
                device_finger_print: device_finger_print.to_string(),
            })
            .send()
            .await?
            .json::<Response<RegisterResponse>>()
            .await?;

        Ok(resp)
    }

    #[allow(clippy::type_complexity)]
    #[tracing::instrument(skip(self))]
    pub async fn visit(
        &self,
        local_device_id: i64,
        remote_device_id: i64,
        password: String,
        visit_desktop: bool,
    ) -> CoreResult<
        Response<
            Result<
                (
                    String,
                    Vec<u8>,
                    OpeningKey<NonceValue>,
                    SealingKey<NonceValue>,
                ),
                VisitFailureReason,
            >,
        >,
    > {
        let url = self.url.join("/api/visit")?;

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

        let resp = self
            .http_client
            .post(url)
            .json(&VisitRequest {
                active_device_id: local_device_id,
                passive_device_id: remote_device_id,
                visit_desktop,
                password_salt: base64_standard.encode(active_device_secret_salt),
                secret: base64_standard.encode(active_device_secret_buffer),
                secret_nonce: base64_standard.encode(active_device_secret_sealing_nonce),
            })
            .timeout(Duration::from_secs(60))
            .send()
            .await?
            .json::<Response<VisitResponse>>()
            .await?;

        match resp {
            Response::Message(resp) => {
                let secret = match resp.result {
                    Ok(secret) => base64_standard.decode(secret)?,
                    Err(reason) => return Ok(Response::Message(Err(reason))),
                };

                let visit_credentials = base64_standard.decode(resp.visit_credentials)?;

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
                let sealing_key =
                    ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(nonce));

                let unbound_opening_key =
                    ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)?;

                let mut nonce = [0u8; 12];
                nonce.copy_from_slice(&active_exchange_nonce);
                let opening_key =
                    ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(nonce));

                Ok(Response::Message(Ok((
                    resp.endpoint_addr,
                    visit_credentials,
                    opening_key,
                    sealing_key,
                ))))
            }
            Response::Error(err) => Ok(Response::Error(err)),
        }
    }

    // see https://github.com/rust-lang/rust-clippy/pull/9496, which was merged but not release
    #[allow(clippy::never_loop)]
    pub async fn subscribe(
        &mut self,
        addrs: Vec<SocketAddr>,
        device_id: i64,
        device_finger_print: &str,
        storage: LocalStorage,
    ) -> CoreResult<()> {
        let subscription_bytes = Bytes::from(bincode_serialize(&Subscription {
            device_id,
            device_finger_print: device_finger_print.to_string(),
        })?);

        for addr in addrs {
            let Ok(Ok(stream)) = tokio::time::timeout(
                Duration::from_secs(10),
                tokio::net::TcpStream::connect(addr),
            )
            .await else {
                continue;
            };

            let mut framed_stream = Framed::new(
                stream,
                LengthDelimitedCodec::builder()
                    .length_field_length(2)
                    .little_endian()
                    .new_codec(),
            );

            framed_stream.send(subscription_bytes.clone()).await?;

            let (sink, stream) = framed_stream.split();
            let (tx, rx) = tokio::sync::mpsc::channel(1);

            tokio::spawn(serve_connection(rx, sink, stream, storage.clone()));

            self.subscribe_tx = Some(tx);

            return Ok(());
        }

        Err(core_error!("non addr usable"))
    }
}

async fn serve_connection(
    mut rx: tokio::sync::mpsc::Receiver<Bytes>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    storage: LocalStorage,
) {
    let mut ticker = tokio::time::interval(Duration::from_secs(60));
    let mut last_ping = None;
    let mut last_ping_value = 0;

    loop {
        let buffer = tokio::select! {
            _ = ticker.tick() => {
                if last_ping.is_some() {
                    return;
                }

                let value = generate_random_ping_value();
                if let Ok(buffer) = bincode_serialize(&ClientMessage::Ping(value)){
                    if (sink.send(Bytes::from(buffer)).await).is_ok() {
                        last_ping = Some(std::time::Instant::now());
                        last_ping_value = value;
                        continue;
                    }
                }

                return;
            }
            buffer = rx.recv() => {
                if let Some(buffer) = buffer {
                    let _ = sink.send(buffer).await;
                    continue;
                } else {
                    return;
                }
            },
            buffer = stream.next() => {
                let Some(Ok(buffer)) = buffer else {
                    return;
                };

                buffer
            }
        };

        let Ok(server_message) = bincode_deserialize::<ServerMessage>(&buffer) else {
            return;
        };

        match server_message {
            ServerMessage::Pong(value) => {
                if value != last_ping_value {
                    return;
                }

                if let Some(instant) = last_ping.take() {
                    if instant.elapsed().as_secs() > 60 {
                        return;
                    }
                }
            }
            ServerMessage::VisitRequest {
                active_device_id,
                passive_device_id,
                visit_desktop: _,
                endpoint_addr,
                password_salt,
                secret,
                secret_nonce,
                passive_visit_credentials,
            } => {
                let storage = storage.clone();
                let (tx, rx) = tokio::sync::oneshot::channel();
                tokio::spawn(async move {
                    let result = serve_visit_request(
                        storage,
                        active_device_id,
                        passive_device_id,
                        endpoint_addr,
                        // visit_desktop,
                        password_salt,
                        secret,
                        secret_nonce,
                        passive_visit_credentials,
                    )
                    .await;

                    let response = ClientMessage::VisitResponse {
                        active_device_id,
                        passive_device_id,
                        result,
                    };

                    let buffer = match bincode_serialize(&response) {
                        Ok(buffer) => buffer,
                        Err(err) => {
                            tracing::error!(?err, "serialize visit response failed");
                            return;
                        }
                    };

                    let _ = tx.send(buffer);
                });

                match rx.await {
                    Ok(buffer) => {
                        if let Err(err) = sink.send(Bytes::from(buffer)).await {
                            tracing::error!(?err, "reply visit failed");
                        }
                    }
                    Err(err) => {
                        tracing::error!(
                            ?err,
                            "receive key exchange result failed, this shouldn't happen!"
                        );
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn serve_visit_request(
    storage: LocalStorage,
    active_device_id: i64,
    passive_device_id: i64,
    endpoint_addr: String,
    password_salt: Vec<u8>,
    secret: Vec<u8>,
    secret_nonce: Vec<u8>,
    passive_visit_credentials: Vec<u8>,
) -> Result<Vec<u8>, VisitFailureReason> {
    let Ok(domain) = storage.domain().get_primary_domain() else {
        return Err(VisitFailureReason::InternalError);
    };

    let Ok(endpoint_addr) = endpoint_addr.parse::<SocketAddr>() else {
        return Err(VisitFailureReason::InternalError);
    };

    let (secret, sealing_key, opening_key) = match key_agreement(
        &domain.password,
        active_device_id,
        password_salt,
        secret,
        secret_nonce,
    )
    .await
    {
        Ok(v) => v,
        Err(err) => {
            return Err(err);
        }
    };

    tokio::spawn(async move {
        if let Err(err) = create_passive_endpoint_client(
            EndPointID::DeviceID {
                local_device_id: passive_device_id,
                remote_device_id: active_device_id,
            },
            Some((opening_key, sealing_key)),
            crate::api::endpoint::EndPointStream::ActiveTCP(endpoint_addr),
            Some(passive_visit_credentials),
        )
        .await
        {
            tracing::error!(?err, "create passive endpoint client failed");
        }
    });

    Ok(secret)
}

async fn key_agreement(
    domain_password: &str,
    active_device_id: i64,
    password_salt: Vec<u8>,
    mut secret: Vec<u8>,
    secret_nonce: Vec<u8>,
) -> Result<(Vec<u8>, SealingKey<NonceValue>, OpeningKey<NonceValue>), VisitFailureReason> {
    if secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(VisitFailureReason::InternalError);
    }

    // generate secret opening key with salt
    let mut active_device_secret_opening_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        domain_password.as_bytes(),
        &password_salt,
        10000,
        &mut active_device_secret_opening_key,
    );

    let unbound_key = match ring::aead::UnboundKey::new(
        &ring::aead::AES_256_GCM,
        &active_device_secret_opening_key,
    ) {
        Ok(unbound_key) => unbound_key,
        Err(err) => {
            tracing::error!(?err, "create unbound key failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    let mut active_device_secret_opening_nonce = [0u8; ring::aead::NONCE_LEN];
    active_device_secret_opening_nonce[..ring::aead::NONCE_LEN]
        .copy_from_slice(&secret_nonce[..ring::aead::NONCE_LEN]);

    let mut active_device_secret_opening_key = ring::aead::OpeningKey::new(
        unbound_key,
        NonceValue::new(active_device_secret_opening_nonce),
    );

    let active_device_secret_buffer = match active_device_secret_opening_key.open_in_place(
        ring::aead::Aad::from(active_device_id.to_le_bytes()),
        &mut secret,
    ) {
        Ok(buffer) => buffer,
        Err(_) => return Err(VisitFailureReason::InvalidPassword),
    };

    let active_device_secret =
        match bincode_deserialize::<ActiveEndpointKeyExchangeSecret>(&*active_device_secret_buffer)
        {
            Ok(secret) => secret,
            Err(_) => {
                return Err(VisitFailureReason::InvalidArgs);
            }
        };

    if active_device_secret.active_exchange_nonce.len() != ring::aead::NONCE_LEN {
        return Err(VisitFailureReason::InvalidArgs);
    }

    // generate passive device key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_exchange_private_key = match ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    ) {
        Ok(private_key) => private_key,
        Err(_) => return Err(VisitFailureReason::InternalError),
    };

    let passive_exchange_public_key = match passive_exchange_private_key.compute_public_key() {
        Ok(public_key) => public_key,
        Err(err) => {
            tracing::error!(
                ?err,
                "compute public key from passive exchange private key failed"
            );
            return Err(VisitFailureReason::InternalError);
        }
    };

    let mut passive_exchange_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut passive_exchange_nonce);

    // key agreement

    let mut active_exchange_nonce = [0u8; ring::aead::NONCE_LEN];
    active_exchange_nonce[..ring::aead::NONCE_LEN]
        .copy_from_slice(&active_device_secret.active_exchange_nonce[..ring::aead::NONCE_LEN]);

    let active_exchange_public_key = ring::agreement::UnparsedPublicKey::new(
        &ring::agreement::X25519,
        active_device_secret.active_exchange_public_key,
    );

    let agree_result = ring::agreement::agree_ephemeral(
        passive_exchange_private_key,
        &active_exchange_public_key,
        ring::error::Unspecified,
        |key_material| {
            let sealing_key =
                ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &passive_exchange_nonce)
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
                active_device_secret.active_exchange_nonce,
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
    );

    let (raw_sealing_key, raw_opening_key) = match agree_result {
        Ok(v) => v,
        Err(err) => {
            tracing::error!(?err, "agree ephemeral failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    // derive opening and sealing key

    let unbound_sealing_key = match UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key) {
        Ok(unbound_sealing_key) => unbound_sealing_key,
        Err(err) => {
            tracing::error!(?err, "create unbound sealing key failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    let sealing_key = SealingKey::new(unbound_sealing_key, NonceValue::new(active_exchange_nonce));

    let unbound_opening_key = match UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key) {
        Ok(unbound_opening_key) => unbound_opening_key,
        Err(err) => {
            tracing::error!(?err, "create unbound opening failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(passive_exchange_nonce));

    // build key exchange response

    let passive_device_secret = PassiveEndpointKeyExchangeSecret {
        passive_exchange_public_key: passive_exchange_public_key.as_ref(),
        passive_exchange_nonce: &passive_exchange_nonce,
    };

    let passive_device_secret_buffer = match bincode_serialize(&passive_device_secret) {
        Ok(buffer) => buffer,
        Err(_) => return Err(VisitFailureReason::InternalError),
    };

    let active_exchange_reply_public_key = match rsa::RsaPublicKey::new(
        BigUint::from_bytes_le(active_device_secret.exchange_reply_public_key_n),
        BigUint::from_bytes_le(active_device_secret.exchange_reply_public_key_e),
    ) {
        Ok(public_key) => public_key,
        Err(err) => {
            tracing::error!(?err, "recover exchange reply public key failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    let secret_buffer = match active_exchange_reply_public_key.encrypt(
        &mut OsRng,
        rsa::Pkcs1v15Encrypt::default(),
        &passive_device_secret_buffer,
    ) {
        Ok(buffer) => buffer,
        Err(err) => {
            tracing::error!(?err, "encrypt exchange reply data failed");
            return Err(VisitFailureReason::InternalError);
        }
    };

    Ok((secret_buffer, sealing_key, opening_key))
}
