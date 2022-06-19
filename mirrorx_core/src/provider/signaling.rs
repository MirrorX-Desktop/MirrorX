use crate::{
    error::{anyhow::Result, MirrorXError},
    socket::signaling::{
        client::{SignalingClient, CURRENT_SIGNALING_CLIENT},
        message::{
            ConnectRequest, ConnectionKeyExchangeActiveDeviceSecret,
            ConnectionKeyExchangePassiveDeviceSecret, ConnectionKeyExchangeRequest,
            HandshakeRequest, HeartBeatRequest,
        },
    },
    utility::{nonce_value::NonceValue, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use hmac::Mac;
use pbkdf2::password_hash::PasswordHasher;
use rand::{rngs::OsRng, RngCore};
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use rsa::PublicKeyParts;
use sha2::Sha512;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tracing::error;

pub async fn init<A>(addr: A) -> anyhow::Result<()>
where
    A: ToSocketAddrs,
{
    let client = SignalingClient::connect(addr).await?;
    CURRENT_SIGNALING_CLIENT.store(Some(Arc::new(client)));
    Ok(())
}

pub async fn heartbeat() -> anyhow::Result<()> {
    let _ = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ProviderNotInitialized)?
        .heartbeat(HeartBeatRequest {
            time_stamp: chrono::Utc::now().timestamp() as u32,
        })
        .await?;

    Ok(())
}

pub async fn handshake() -> anyhow::Result<()> {
    let device_id = crate::provider::config::read_device_id()?;
    let unique_id = crate::provider::config::read_unique_id()?;
    let device_native_id = machine_uid::get().map_err(|err| MirrorXError::Raw(err.to_string()))?;

    let device_token = if device_id.is_some() && unique_id.is_some() {
        Some((device_id.unwrap(), unique_id.unwrap()))
    } else {
        None
    };

    let mut salt = [0u8; 256];
    OsRng.fill_bytes(&mut salt);

    let mut mac = hmac::Hmac::<Sha512>::new_from_slice(&salt).map_err(|err| {
        error!(err=?err, "handshake: init hmac failed");
        MirrorXError::Raw(err.to_string())
    })?;

    mac.update(device_native_id.as_ref());
    let device_native_id_salt = mac.finalize().into_bytes().to_vec();

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ProviderNotInitialized)?
        .handshake(HandshakeRequest {
            device_token,
            device_native_id,
            device_native_id_salt,
        })
        .await?;

    crate::provider::config::save_device_id(&resp.device_id)?;
    crate::provider::config::save_device_id_expiration(&resp.device_id_expiration)?;
    crate::provider::config::save_unique_id(&resp.unique_id)?;

    Ok(())
}

pub async fn connect(remote_device_id: String) -> anyhow::Result<bool> {
    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ProviderNotInitialized)?
        .connect_remote(ConnectRequest { remote_device_id })
        .await?;

    Ok(resp.allow)
}

pub async fn connection_key_exchange(
    active_device_id: String,
    passive_device_id: String,
    password: String,
) -> anyhow::Result<(OpeningKey<NonceValue>, SealingKey<NonceValue>)> {
    // generate rsa key pair for remote device reply
    let response_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096).map_err(|err| {
        error!(err=?err,"connection_key_exchange: generate response private key failed");
        MirrorXError::Raw(err.to_string())
    })?;

    let response_public_key = rsa::RsaPublicKey::from(&response_private_key);

    // generate key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let active_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|err| {
        error!(
            err=?err,
            "connection_key_exchange: generate ephemeral private key failed",
        );
        MirrorXError::Raw(err.to_string())
    })?;

    let active_device_public_key =
        active_device_private_key
            .compute_public_key()
            .map_err(|err| {
                error!(
                    err=?err,
                    "connection_key_exchange: compute public key failed",
                );
                MirrorXError::Raw(err.to_string())
            })?;

    let mut active_device_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_device_nonce);

    // derive aes-256-gcm key from password

    let password_derive_salt = pbkdf2::password_hash::SaltString::generate(&mut OsRng);

    let derived_key = pbkdf2::Pbkdf2
        .hash_password(password.as_bytes(), &password_derive_salt)
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: hash password failed");
            MirrorXError::Raw(err.to_string())
        })?
        .to_string();

    // build secret

    let mut secret_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut secret_nonce);

    let active_device_secret = ConnectionKeyExchangeActiveDeviceSecret {
        response_public_key_n: response_public_key.n().to_bytes_le(),
        response_public_key_e: response_public_key.e().to_bytes_le(),
        active_device_public_key: active_device_public_key.as_ref(),
        active_device_nonce: &active_device_nonce,
    };

    let mut active_device_secret_buffer = BINCODE_SERIALIZER
        .serialize(&active_device_secret)
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: serialize secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    // sealing packet and call key-exchange

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, derived_key.as_bytes())
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: create unbound key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let mut sealing_key = ring::aead::SealingKey::new(unbound_key, NonceValue::new(secret_nonce));

    sealing_key
        .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut active_device_secret_buffer)
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: seal secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ProviderNotInitialized)?
        .connection_key_exchange(ConnectionKeyExchangeRequest {
            active_device_id,
            password_derive_salt: password_derive_salt.as_bytes().to_vec(),
            secret: active_device_secret_buffer,
            secret_nonce: secret_nonce.to_vec(),
        })
        .await?;

    if resp.passive_device_id != passive_device_id {
        return Err(MirrorXError::Raw(
            "connection_key_exchange: key exchang responsed device is not aimed device".to_string(),
        ));
    }

    // handle key exchange response

    let passive_device_secret_buffer = response_private_key
        .decrypt(
            rsa::PaddingScheme::PKCS1v15Encrypt,
            resp.exchange_data.as_ref(),
        )
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: decrypt response exchange data failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let passive_device_secret = BINCODE_SERIALIZER
        .deserialize::<ConnectionKeyExchangePassiveDeviceSecret>(&passive_device_secret_buffer)
        .map_err(|err| {
            error!(err=?err,"connection_key_exchange: deserialize secret failed");
            MirrorXError::Raw(err.to_string())
        })?;

    if passive_device_secret.passive_device_nonce.len() != ring::aead::NONCE_LEN {
        return Err(MirrorXError::Raw(
            "connection_key_exchange: passive device provide invalid key exchange nonce"
                .to_string(),
        ));
    }

    let passive_device_public_key = ring::agreement::UnparsedPublicKey::new(
        &ring::agreement::X25519,
        passive_device_secret.passive_device_public_key,
    );

    let (raw_sealing_key, raw_opening_key) = ring::agreement::agree_ephemeral(
        active_device_private_key,
        &passive_device_public_key,
        ring::error::Unspecified,
        |key_material| {
            let sealing_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &active_device_nonce)
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
                &passive_device_secret.passive_device_nonce,
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
        error!(err=?err,"connection_key_exchange: key agreement failed");
        MirrorXError::Raw(err.to_string())
    })?;

    let mut passive_device_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        passive_device_nonce[i] = passive_device_secret.passive_device_nonce[i];
    }

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key).map_err(|err| {
            error!(err=?err,"connection_key_exchange: create agreement sealing key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_device_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key).map_err(|err| {
            error!(err=?err,"connection_key_exchange: create agreement opening key failed");
            MirrorXError::Raw(err.to_string())
        })?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_device_nonce));

    Ok((opening_key, sealing_key))
}
