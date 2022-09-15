use crate::{
    api::{endpoint::handlers::connect::ConnectRequest, signaling::SignalingClientManager},
    error::{CoreError, CoreResult},
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME},
};
use either::Either;
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

pub async fn handle(config_path: &str, req: &KeyExchangeRequest) {
    let reply = match key_agreement(config_path, req).await {
        Ok((
            secret_buffer,
            active_device_id,
            passive_device_id,
            domain,
            visit_credentials,
            sealing_key,
            opening_key,
        )) => {
            if let Err(err) = build_endpoint(
                active_device_id,
                passive_device_id,
                domain,
                visit_credentials,
                opening_key,
                sealing_key,
            )
            .await
            {
                tracing::error!(?err, "build endpoint failed");
                Either::Right(KeyExchangeReplyError::Internal)
            } else {
                Either::Left(secret_buffer)
            }
        }
        Err(err) => {
            if let CoreError::KeyExchangeReplyError(err) = err {
                Either::Right(err)
            } else {
                tracing::error!(?err, "handle key agreement failed");
                Either::Right(KeyExchangeReplyError::Internal)
            }
        }
    };

    let mut client = match SignalingClientManager::get_client().await {
        Ok(client) => client,
        Err(err) => {
            tracing::error!(?err, "get signaling client failed in key exchange handler");
            return;
        }
    };

    if let Err(err) = client.key_exchange_reply(build_reply(req, reply)).await {
        tracing::error!(?req.active_device_id, ?req.passive_device_id, ?err, "reply key exchange request failed");
    }
}

async fn key_agreement(
    config_path: &str,
    req: &KeyExchangeRequest,
) -> CoreResult<(
    Vec<u8>,
    i64,
    i64,
    String,
    String,
    SealingKey<NonceValue>,
    OpeningKey<NonceValue>,
)> {
    let domain_config = crate::api::config::read_domain_config(config_path, &req.domain)?.ok_or(
        CoreError::KeyExchangeReplyError(KeyExchangeReplyError::InvalidArgs),
    )?;

    if req.secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(CoreError::KeyExchangeReplyError(
            KeyExchangeReplyError::InvalidArgs,
        ));
    }

    // generate secret opening key with salt
    let mut active_device_secret_opening_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        domain_config.device_password.as_bytes(),
        &req.password_salt,
        10000,
        &mut active_device_secret_opening_key,
    );

    let unbound_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &active_device_secret_opening_key)?;

    let mut active_device_secret_opening_nonce = [0u8; ring::aead::NONCE_LEN];
    active_device_secret_opening_nonce[..ring::aead::NONCE_LEN]
        .copy_from_slice(&req.secret_nonce[..ring::aead::NONCE_LEN]);

    let mut active_device_secret_opening_key = ring::aead::OpeningKey::new(
        unbound_key,
        NonceValue::new(active_device_secret_opening_nonce),
    );

    let mut active_device_secret_buffer = req.secret.to_owned();

    let active_device_secret_buffer = active_device_secret_opening_key
        .open_in_place(ring::aead::Aad::empty(), &mut active_device_secret_buffer)
        .map_err(|_| CoreError::KeyExchangeReplyError(KeyExchangeReplyError::InvalidPassword))?;

    let active_device_secret =
        KeyExchangeActiveDeviceSecret::decode(&*active_device_secret_buffer)?;

    if active_device_secret.active_exchange_nonce.len() != ring::aead::NONCE_LEN {
        return Err(CoreError::KeyExchangeReplyError(
            KeyExchangeReplyError::InvalidArgs,
        ));
    }

    // generate passive device key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_exchange_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )?;

    let passive_exchange_public_key = passive_exchange_private_key.compute_public_key()?;

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
    )?;

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(active_exchange_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)?;

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
    )?;

    let secret_buffer = active_exchange_reply_public_key.encrypt(
        &mut OsRng,
        rsa::PaddingScheme::PKCS1v15Encrypt,
        &passive_device_secret_buffer,
    )?;

    Ok((
        secret_buffer,
        req.active_device_id,
        req.passive_device_id,
        req.domain.to_owned(),
        active_device_secret.visit_credentials,
        sealing_key,
        opening_key,
    ))
}

async fn build_endpoint(
    active_device_id: i64,
    passive_device_id: i64,
    domain: String,
    visit_credentials: String,
    opening_key: OpeningKey<NonceValue>,
    sealing_key: SealingKey<NonceValue>,
) -> CoreResult<()> {
    crate::api::endpoint::handlers::connect::connect(ConnectRequest {
        active_device_id,
        passive_device_id,
        addr: domain,
    })
    .await?;

    // run in new future otherwise it will dead lock for waiting active device handshake
    TOKIO_RUNTIME.spawn(async move {
        if let Err(err) = crate::api::endpoint::handlers::handshake::passive_device_handshake(
            active_device_id,
            passive_device_id,
            visit_credentials,
            opening_key,
            sealing_key,
        )
        .await
        {
            tracing::error!(?err, "passive device handshake failed");
        }
    });

    Ok(())
}

fn build_reply(
    req: &KeyExchangeRequest,
    reply: Either<Vec<u8>, KeyExchangeReplyError>,
) -> KeyExchangeReplyRequest {
    let inner_key_exchange_result = reply.either(InnerKeyExchangeResult::Secret, |error| {
        InnerKeyExchangeResult::Error(error.into())
    });

    KeyExchangeReplyRequest {
        domain: req.domain.to_owned(),
        active_device_id: req.active_device_id,
        passive_device_id: req.passive_device_id,
        key_exchange_result: Some(KeyExchangeResult {
            inner_key_exchange_result: Some(inner_key_exchange_result),
        }),
    }
}
