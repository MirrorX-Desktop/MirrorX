use crate::{
    api::{
        config::{entity::domain::Domain, LocalStorage},
        endpoint::{create_passive_endpoint_client, id::EndPointID},
    },
    utility::nonce_value::NonceValue,
};
use hmac::Hmac;
use prost::Message;
use rand::RngCore;
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use rsa::{rand_core::OsRng, BigUint, PublicKey};
use sha2::Sha256;
use signaling_proto::message::{
    key_exchange_result::InnerKeyExchangeResult, KeyExchangeActiveDeviceSecret,
    KeyExchangePassiveDeviceSecret, KeyExchangeReplyError, KeyExchangeReplyRequest,
    KeyExchangeRequest, KeyExchangeResult,
};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tonic::transport::{Channel, Uri};

pub async fn handle(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    domain_id: i64,
    req: &KeyExchangeRequest,
) {
    let Ok(storage)= LocalStorage::current() else {
        send_key_exchange_reply(client, req.active_device_id, req.passive_device_id, Err(KeyExchangeReplyError::Internal)).await;
        return;
    };

    let Ok(domain) = storage.domain().get_domain_by_id(domain_id) else {
        send_key_exchange_reply(client, req.active_device_id, req.passive_device_id, Err(KeyExchangeReplyError::Internal)).await;
        return;
    };

    let Ok(uri) = Uri::try_from(&domain.addr) else {
        send_key_exchange_reply(client, req.active_device_id, req.passive_device_id, Err(KeyExchangeReplyError::Internal)).await;
        return;
    };

    let Some(host) = uri.host().map(|host| host.to_string()) else {
        send_key_exchange_reply(client, req.active_device_id, req.passive_device_id, Err(KeyExchangeReplyError::Internal)).await;
        return;
    };

    let (resolve_tx, resolve_rx) = tokio::sync::oneshot::channel();
    tokio::task::spawn_blocking(move || {
        if let Ok(resolved_addrs) = format!("{}:{}", host, 29000)
            .to_socket_addrs()
            .map(|addrs| addrs.collect::<Vec<SocketAddr>>())
        {
            let _ = resolve_tx.send(resolved_addrs);
        }
    });

    let Ok(Ok(resolved_addrs)) = tokio::time::timeout(Duration::from_secs(10), resolve_rx).await else {
        send_key_exchange_reply(client, req.active_device_id, req.passive_device_id, Err(KeyExchangeReplyError::Internal)).await;
        return;
    };

    if resolved_addrs.is_empty() {
        send_key_exchange_reply(
            client,
            req.active_device_id,
            req.passive_device_id,
            Err(KeyExchangeReplyError::Internal),
        )
        .await;
        return;
    }

    let (active_device_id, passive_device_id, _, visit_credentials, sealing_key, opening_key) =
        match key_agreement(&domain, req).await {
            Ok((
                secret,
                active_device_id,
                passive_device_id,
                visit_credentials,
                sealing_key,
                opening_key,
            )) => {
                send_key_exchange_reply(
                    client,
                    req.active_device_id,
                    req.passive_device_id,
                    Ok(secret),
                )
                .await;

                (
                    active_device_id,
                    passive_device_id,
                    domain.name,
                    visit_credentials,
                    sealing_key,
                    opening_key,
                )
            }
            Err(err) => {
                tracing::error!(?err, "key agreement failed");

                send_key_exchange_reply(
                    client,
                    req.active_device_id,
                    req.passive_device_id,
                    Err(err),
                )
                .await;

                return;
            }
        };

    if let Err(err) = create_passive_endpoint_client(
        EndPointID::DeviceID {
            local_device_id: passive_device_id,
            remote_device_id: active_device_id,
        },
        Some((opening_key, sealing_key)),
        crate::api::endpoint::EndPointStream::ActiveTCP(resolved_addrs[0]),
        Some(visit_credentials),
    )
    .await
    {
        tracing::error!(?err, "create passive endpoint client failed");
    }
}

async fn key_agreement(
    domain: &Domain,
    req: &KeyExchangeRequest,
) -> Result<
    (
        Vec<u8>,
        i64,
        i64,
        String,
        SealingKey<NonceValue>,
        OpeningKey<NonceValue>,
    ),
    KeyExchangeReplyError,
> {
    if req.secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(KeyExchangeReplyError::InvalidArgs);
    }

    tracing::info!(?domain.password,"domain password");

    // generate secret opening key with salt
    let mut active_device_secret_opening_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        domain.password.as_bytes(),
        &req.password_salt,
        10000,
        &mut active_device_secret_opening_key,
    );

    let unbound_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &active_device_secret_opening_key)
            .map_err(|err| {
                tracing::error!(?err, "create unbound key failed");
                KeyExchangeReplyError::Internal
            })?;

    let mut active_device_secret_opening_nonce = [0u8; ring::aead::NONCE_LEN];
    active_device_secret_opening_nonce[..ring::aead::NONCE_LEN]
        .copy_from_slice(&req.secret_nonce[..ring::aead::NONCE_LEN]);

    let mut active_device_secret_opening_key = ring::aead::OpeningKey::new(
        unbound_key,
        NonceValue::new(active_device_secret_opening_nonce),
    );

    let mut active_device_secret_buffer = req.secret.to_owned();

    let active_device_secret_buffer = active_device_secret_opening_key
        .open_in_place(
            ring::aead::Aad::from(req.active_device_id.to_le_bytes()),
            &mut active_device_secret_buffer,
        )
        .map_err(|_| KeyExchangeReplyError::InvalidPassword)?;

    let active_device_secret = KeyExchangeActiveDeviceSecret::decode(&*active_device_secret_buffer)
        .map_err(|err| {
            tracing::error!(?err, "decode active device secret failed");
            KeyExchangeReplyError::Internal
        })?;

    if active_device_secret.active_exchange_nonce.len() != ring::aead::NONCE_LEN {
        return Err(KeyExchangeReplyError::InvalidArgs);
    }

    // generate passive device key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_exchange_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|err| {
        tracing::error!(?err, "generate ephemeral private key failed");
        KeyExchangeReplyError::Internal
    })?;

    let passive_exchange_public_key =
        passive_exchange_private_key
            .compute_public_key()
            .map_err(|err| {
                tracing::error!(?err, "compute exchange public key failed");
                KeyExchangeReplyError::Internal
            })?;

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

    let (raw_sealing_key, raw_opening_key) = ring::agreement::agree_ephemeral(
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
                &active_device_secret.active_exchange_nonce,
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
    )
    .map_err(|err| {
        tracing::error!(?err, "agree ephemeral failed");
        KeyExchangeReplyError::Internal
    })?;

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key).map_err(|err| {
            tracing::error!(?err, "create unbound sealing key failed");
            KeyExchangeReplyError::Internal
        })?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(active_exchange_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key).map_err(|err| {
            tracing::error!(?err, "create unbound opening failed");
            KeyExchangeReplyError::Internal
        })?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(passive_exchange_nonce));

    // build key exchange response

    let passive_device_secret = KeyExchangePassiveDeviceSecret {
        passive_exchange_public_key: passive_exchange_public_key.as_ref().to_vec(),
        passive_exchange_nonce: passive_exchange_nonce.to_vec(),
    };

    let passive_device_secret_buffer = passive_device_secret.encode_to_vec();

    let active_exchange_reply_public_key = rsa::RsaPublicKey::new(
        BigUint::from_bytes_le(&active_device_secret.exchange_reply_public_key_n),
        BigUint::from_bytes_le(&active_device_secret.exchange_reply_public_key_e),
    )
    .map_err(|err| {
        tracing::error!(?err, "recover exchange reply public key failed");
        KeyExchangeReplyError::Internal
    })?;

    let secret_buffer = active_exchange_reply_public_key
        .encrypt(
            &mut OsRng,
            rsa::PaddingScheme::PKCS1v15Encrypt,
            &passive_device_secret_buffer,
        )
        .map_err(|err| {
            tracing::error!(?err, "encrypt exchange reply data failed");
            KeyExchangeReplyError::Internal
        })?;

    Ok((
        secret_buffer,
        req.active_device_id,
        req.passive_device_id,
        active_device_secret.visit_credentials,
        sealing_key,
        opening_key,
    ))
}

async fn send_key_exchange_reply(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    active_device_id: i64,
    passive_device_id: i64,
    reply: Result<Vec<u8>, KeyExchangeReplyError>,
) {
    let inner_key_exchange_result = match reply {
        Ok(secret) => InnerKeyExchangeResult::Secret(secret),
        Err(err) => InnerKeyExchangeResult::Error(err.into()),
    };

    let reply = KeyExchangeReplyRequest {
        active_device_id,
        passive_device_id,
        key_exchange_result: Some(KeyExchangeResult {
            inner_key_exchange_result: Some(inner_key_exchange_result),
        }),
    };

    if let Err(err) = client.key_exchange_reply(reply).await {
        tracing::error!(
            ?active_device_id,
            ?passive_device_id,
            ?err,
            "reply key exchange request failed"
        );
    }
}
