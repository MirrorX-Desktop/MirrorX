use crate::{
    core_error,
    error::{CoreError, CoreResult},
    service::signaling::{
        client::{SignalingClient, CURRENT_SIGNALING_CLIENT},
        message::{
            ConnectRequest, ConnectionKeyExchangeActiveDeviceSecret,
            ConnectionKeyExchangePassiveDeviceSecret, ConnectionKeyExchangeRequest,
            HandshakeRequest, HeartBeatRequest,
        },
    },
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use hmac::Hmac;
use rand::{rngs::OsRng, RngCore};
use ring::aead::BoundKey;
use rsa::PublicKeyParts;
use sha2::Sha256;
use std::{sync::Arc, time::Duration};
use tokio::net::ToSocketAddrs;
use tracing::error;

pub async fn init<A>(addr: A) -> CoreResult<()>
where
    A: ToSocketAddrs,
{
    let client = SignalingClient::connect(addr).await?;
    CURRENT_SIGNALING_CLIENT.store(Some(Arc::new(client)));
    Ok(())
}

pub fn begin_heartbeat() {
    TOKIO_RUNTIME.spawn(async move {
        let mut failures = 0;
        let mut interval = tokio::time::interval(Duration::from_secs(20));
        loop {
            interval.tick().await;

            if let Some(client) = CURRENT_SIGNALING_CLIENT.load().as_ref() {
                match client
                    .heartbeat(
                        None,
                        HeartBeatRequest {
                            time_stamp: chrono::Utc::now().timestamp() as u32,
                        },
                    )
                    .await
                {
                    Ok(_) => failures = 0,
                    Err(_) => {
                        failures += 1;
                        if failures == 5 {
                            break;
                        }
                    }
                };
            } else {
                break;
            }
        }
        error!("too many failures while heart beat");
    });
}

pub async fn handshake() -> CoreResult<()> {
    let device_id = crate::api::config::read_device_id()?;
    let device_hash = match crate::api::config::read_device_hash()? {
        Some(v) => v,
        None => {
            let mut device_hash = [0u8; 512];
            OsRng.fill_bytes(&mut device_hash);
            hex::encode_upper(device_hash)
        }
    };

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(core_error!("signaling client not exists"))?
        .handshake(
            None,
            HandshakeRequest {
                device_id: device_id.clone(),
                device_hash: device_hash.clone(),
            },
        )
        .await?;

    crate::api::config::save_device_id(&resp.device_id)?;
    crate::api::config::save_device_hash(&device_hash)?;
    crate::api::config::save_device_id_expiration(&resp.expire)?;

    CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(core_error!("signaling client not exists"))?
        .set_device_id(resp.device_id);

    Ok(())
}

pub async fn connect(remote_device_id: String) -> CoreResult<bool> {
    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(core_error!("signaling client not exists"))?
        .connect_remote(Some(remote_device_id), ConnectRequest {})
        .await?;

    Ok(resp.allow)
}

pub async fn connection_key_exchange(
    remote_device_id: String,
    password: String,
) -> Result<(), CoreError> {
    let local_device_id = match crate::api::config::read_device_id()? {
        Some(id) => id,
        None => return Err(core_error!("local device id is None")),
    };

    // generate rsa key pair for remote device reply
    let response_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096)
        .map_err(|err| (core_error!("generate response private key failed ({})", err)))?;

    let response_public_key = rsa::RsaPublicKey::from(&response_private_key);

    // generate key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let active_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|_| core_error!("generate key exchange private key failed"))?;

    let active_device_public_key = active_device_private_key
        .compute_public_key()
        .map_err(|_| core_error!("generate key exchange public key failed"))?;

    let mut active_device_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_device_nonce);

    // derive aes-256-gcm key from password

    let mut password_derive_salt = [0u8; 16];
    OsRng.fill_bytes(&mut password_derive_salt);

    let mut derived_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        &password_derive_salt,
        pbkdf2::Params::default().rounds,
        &mut derived_key,
    );

    // build secret

    let mut secret_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut secret_nonce);

    let active_device_secret = ConnectionKeyExchangeActiveDeviceSecret {
        response_public_key_n: response_public_key.n().to_bytes_le(),
        response_public_key_e: response_public_key.e().to_bytes_le(),
        active_device_public_key: active_device_public_key.as_ref(),
        active_device_nonce: &active_device_nonce,
    };

    let mut active_device_secret_buffer = BINCODE_SERIALIZER.serialize(&active_device_secret)?;

    // sealing packet and call key-exchange

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &derived_key)
        .map_err(|_| core_error!("generate request message packet sealing key failed"))?;

    let mut sealing_key = ring::aead::SealingKey::new(unbound_key, NonceValue::new(secret_nonce));

    sealing_key
        .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut active_device_secret_buffer)
        .map_err(|_| core_error!("sealing request message packet failed"))?;

    let resp = CURRENT_SIGNALING_CLIENT
        .load()
        .as_ref()
        .ok_or(core_error!("current signaling client not exists"))?
        .connection_key_exchange(
            Some(remote_device_id.clone()),
            ConnectionKeyExchangeRequest {
                active_device_id: local_device_id.clone(),
                password_derive_salt: password_derive_salt.to_vec(),
                secret: active_device_secret_buffer,
                secret_nonce: secret_nonce.to_vec(),
            },
        )
        .await?;

    if resp.passive_device_id != remote_device_id {
        return Err(core_error!(
            "key exchang responsed device is not aimed device"
        ));
    }

    // handle key exchange response

    let passive_device_secret_buffer = response_private_key
        .decrypt(
            rsa::PaddingScheme::PKCS1v15Encrypt,
            resp.exchange_data.as_ref(),
        )
        .map_err(|err| core_error!("decrypt key exchange data failed ({})", err))?;

    let passive_device_secret = BINCODE_SERIALIZER
        .deserialize::<ConnectionKeyExchangePassiveDeviceSecret>(&passive_device_secret_buffer)?;

    if passive_device_secret.passive_device_nonce.len() != ring::aead::NONCE_LEN {
        return Err(core_error!(
            "passive device provide invalid key exchange nonce"
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
    .map_err(|_| core_error!("key exchange agree ephemeral failed"))?;

    let mut passive_device_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        passive_device_nonce[i] = passive_device_secret.passive_device_nonce[i];
    }

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)
            .map_err(|_| core_error!("generate unbound sealing key failed"))?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_device_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)
            .map_err(|_| core_error!("generate unbound opening key failed"))?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_device_nonce));

    crate::api::endpoint::connect(
        true,
        local_device_id,
        remote_device_id,
        sealing_key,
        opening_key,
    )
    .await?;

    Ok(())
}
