use super::message::{
    ConnectRequest, ConnectResponse, ConnectionKeyExchangeActiveDeviceSecret,
    ConnectionKeyExchangePassiveDeviceSecret, ConnectionKeyExchangeRequest,
    ConnectionKeyExchangeResponse,
};
use crate::{
    error::MirrorXError,
    provider,
    utility::{nonce_value::NonceValue, serializer::BINCODE_SERIALIZER},
};
use anyhow::anyhow;
use bincode::Options;
use pbkdf2::password_hash::PasswordHasher;
use rand::RngCore;
use ring::aead::BoundKey;
use rsa::{rand_core::OsRng, BigUint, PublicKey};
use tracing::error;

pub async fn handle_connect_request(req: ConnectRequest) -> Result<ConnectResponse, MirrorXError> {
    Ok(ConnectResponse { allow: true })
}

pub async fn handle_connection_key_exchange_request(
    mut req: ConnectionKeyExchangeRequest,
) -> Result<ConnectionKeyExchangeResponse, MirrorXError> {
    let passive_device_id = provider::config::read_device_id()?;

    let password = provider::config::read_device_password()?;

    if req.secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(MirrorXError::CipherNonceInvalid);
    }

    // try to decrypt secret

    let salt_string = String::from_utf8(req.password_derive_salt)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let password_derive_salt = pbkdf2::password_hash::SaltString::new(&salt_string)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let derived_key = pbkdf2::Pbkdf2
        .hash_password(password.as_bytes(), &password_derive_salt)
        .map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: hash password failed");
            MirrorXError::Raw(err.to_string())
        })?
        .to_string();

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, derived_key.as_bytes())
        .map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: create unbound key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let mut active_device_secret_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        active_device_secret_nonce[i] = req.secret_nonce[i];
    }

    let mut active_device_secret_opening_key =
        ring::aead::OpeningKey::new(unbound_key, NonceValue::new(active_device_secret_nonce));

    active_device_secret_opening_key
        .open_in_place(ring::aead::Aad::empty(), &mut req.secret)
        .map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: open secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let active_device_secret = BINCODE_SERIALIZER
        .deserialize::<ConnectionKeyExchangeActiveDeviceSecret>(&req.secret)
        .map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: serialize secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    if active_device_secret.active_device_nonce.len() != ring::aead::NONCE_LEN {
        return Err(MirrorXError::Raw("handle_connection_key_exchange_request: active device provide invalid key exchange nonce".to_string()));
    }

    let active_device_response_public_key = rsa::RsaPublicKey::new(BigUint::from_bytes_le(&active_device_secret.response_public_key_n),BigUint::from_bytes_le(&active_device_secret.response_public_key_e)).map_err(|err|{
        error!(err=?err,"handle_connection_key_exchange_request: recover active device response public key failed");
        MirrorXError::Raw(err.to_string())
    })?;

    // generate key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|err| {
        error!(
            err=?err,
            "handle_connection_key_exchange_request: generate ephemeral private key failed",
        );
        MirrorXError::Raw(err.to_string())
    })?;

    let passive_device_public_key =
        passive_device_private_key
            .compute_public_key()
            .map_err(|err| {
                error!(
                    err=?err,
                    "handle_connection_key_exchange_request: compute public key failed",
                );
                MirrorXError::Raw(err.to_string())
            })?;

    let mut passive_device_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut passive_device_nonce);

    // key agreement

    let mut active_device_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        active_device_nonce[i] = active_device_secret.active_device_nonce[i];
    }

    let active_device_public_key = ring::agreement::UnparsedPublicKey::new(
        &ring::agreement::X25519,
        active_device_secret.active_device_public_key,
    );

    let (raw_sealing_key, raw_opening_key) = ring::agreement::agree_ephemeral(
        passive_device_private_key,
        &active_device_public_key,
        ring::error::Unspecified,
        |key_material| {
            let sealing_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &passive_device_nonce)
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
                &active_device_secret.active_device_nonce,
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
        error!(err=?err,"handle_connection_key_exchange_request: key agreement failed");
        MirrorXError::Raw(err.to_string())
    })?;

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key).map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: create agreemented sealing key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_device_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key).map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: create agreemented opening key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_device_nonce));

    // create endpoint

    let endpoint_active_device_id = req.active_device_id.clone();
    let endpoint_passive_device_id = passive_device_id.clone();

    tokio::spawn(async move {
        if let Err(err) = provider::endpoint::connect(
            endpoint_active_device_id.clone(),
            endpoint_passive_device_id,
            sealing_key,
            opening_key,
        )
        .await
        {
            error!(err=?err,active_device_id=?endpoint_active_device_id,"handle_connection_key_exchange_request: create endpoint failed");
        }
    });

    // encrypt response inner passive device secret

    let passive_device_secret = ConnectionKeyExchangePassiveDeviceSecret {
        passive_device_public_key: passive_device_public_key.as_ref(),
        passive_device_nonce: &passive_device_nonce,
    };

    let passive_secret_buffer = BINCODE_SERIALIZER
        .serialize(&passive_device_secret)
        .map_err(|err| {
            error!(err=?err,"handle_connection_key_exchange_request: serialize secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let exchange_data = active_device_response_public_key.encrypt(
        &mut OsRng,
        rsa::PaddingScheme::PKCS1v15Encrypt,
        &passive_secret_buffer,
    ).map_err(|err|{
        error!(err=?err,"handle_connection_key_exchange_request: encrypt response exchange data failed");
        MirrorXError::Raw(err.to_string())
    })?;

    Ok(ConnectionKeyExchangeResponse {
        passive_device_id,
        exchange_data,
    })
}
