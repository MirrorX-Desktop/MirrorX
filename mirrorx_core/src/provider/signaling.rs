use crate::{
    error::MirrorXError,
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
use anyhow::anyhow;
use bincode::Options;
use hmac::Mac;
use pbkdf2::password_hash::PasswordHasher;
use rand::{rngs::OsRng, RngCore};
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use rsa::PublicKeyParts;
use sha2::Sha512;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;

pub async fn init<A>(addr: A) -> Result<(), MirrorXError>
where
    A: ToSocketAddrs,
{
    let client = SignalingClient::connect(addr).await?;
    CURRENT_SIGNALING_CLIENT.store(Some(Arc::new(client)));
    Ok(())
}

pub async fn heartbeat() -> Result<(), MirrorXError> {
    let _ = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ComponentUninitialized)?
        .heartbeat(HeartBeatRequest {
            time_stamp: chrono::Utc::now().timestamp() as u32,
        })
        .await?;

    Ok(())
}

pub async fn handshake() -> Result<(), MirrorXError> {
    let device_id = crate::provider::config::read_device_id()?;
    let unique_id = crate::provider::config::read_unique_id()?;
    let device_native_id =
        machine_uid::get().map_err(|err| MirrorXError::Other(anyhow!(err.to_string())))?;

    let device_token = if device_id.is_some() && unique_id.is_some() {
        Some((device_id.unwrap(), unique_id.unwrap()))
    } else {
        None
    };

    let mut salt = [0u8; 256];
    OsRng.fill_bytes(&mut salt);

    let mut mac = hmac::Hmac::<Sha512>::new_from_slice(&salt)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    mac.update(device_native_id.as_ref());
    let device_native_id_salt = mac.finalize().into_bytes().to_vec();

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ComponentUninitialized)?
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

pub async fn connect(remote_device_id: String) -> Result<bool, MirrorXError> {
    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ComponentUninitialized)?
        .connect_remote(ConnectRequest { remote_device_id })
        .await?;

    Ok(resp.allow)
}

pub async fn connection_key_exchange(
    remote_device_id: String,
    password: String,
) -> Result<(), MirrorXError> {
    let local_device_id = match crate::provider::config::read_device_id()? {
        Some(id) => id,
        None => return Err(MirrorXError::LocalDeviceIDInvalid),
    };

    // generate rsa key pair for remote device reply
    let response_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let response_public_key = rsa::RsaPublicKey::from(&response_private_key);

    // generate key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let active_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let active_device_public_key = active_device_private_key
        .compute_public_key()
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let mut active_device_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_device_nonce);

    // derive aes-256-gcm key from password

    let password_derive_salt = pbkdf2::password_hash::SaltString::generate(&mut OsRng);

    let derived_key = pbkdf2::Pbkdf2
        .hash_password(password.as_bytes(), &password_derive_salt)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?
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
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    // sealing packet and call key-exchange

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, derived_key.as_bytes())
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let mut sealing_key = ring::aead::SealingKey::new(unbound_key, NonceValue::new(secret_nonce));

    sealing_key
        .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut active_device_secret_buffer)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(MirrorXError::ComponentUninitialized)?
        .connection_key_exchange(ConnectionKeyExchangeRequest {
            active_device_id: local_device_id.clone(),
            password_derive_salt: password_derive_salt.as_bytes().to_vec(),
            secret: active_device_secret_buffer,
            secret_nonce: secret_nonce.to_vec(),
        })
        .await?;

    if resp.passive_device_id != remote_device_id {
        return Err(MirrorXError::Other(anyhow!(
            "connection_key_exchange: key exchang responsed device is not aimed device"
        )));
    }

    // handle key exchange response

    let passive_device_secret_buffer = response_private_key
        .decrypt(
            rsa::PaddingScheme::PKCS1v15Encrypt,
            resp.exchange_data.as_ref(),
        )
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let passive_device_secret = BINCODE_SERIALIZER
        .deserialize::<ConnectionKeyExchangePassiveDeviceSecret>(&passive_device_secret_buffer)
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    if passive_device_secret.passive_device_nonce.len() != ring::aead::NONCE_LEN {
        return Err(MirrorXError::Other(anyhow!(
            "connection_key_exchange: passive device provide invalid key exchange nonce"
        )));
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
    .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let mut passive_device_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        passive_device_nonce[i] = passive_device_secret.passive_device_nonce[i];
    }

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)
            .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_device_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)
            .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_device_nonce));

    crate::provider::endpoint::connect(local_device_id, remote_device_id, sealing_key, opening_key)
        .await?;

    Ok(())
}
