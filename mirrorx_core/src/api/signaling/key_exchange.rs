use super::SignalingClientManager;
use crate::{
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
};
use hmac::Hmac;
use prost::Message;
use rand::RngCore;
use ring::aead::BoundKey;
use rsa::{rand_core::OsRng, PublicKeyParts};
use sha2::Sha256;
use signaling_proto::{KeyExchangeActiveDeviceSecret, KeyExchangePassiveDeviceSecret};

pub struct KeyExchangeRequest {
    pub local_device_id: String,
    pub remote_device_id: String,
    pub password: String,
}

pub struct KeyExchangeResponse {
    pub opening_key_bytes: Vec<u8>,
    pub opening_nonce_bytes: Vec<u8>,
    pub sealing_key_bytes: Vec<u8>,
    pub sealing_nonce_bytes: Vec<u8>,
}

pub async fn key_exchange(req: KeyExchangeRequest) -> CoreResult<KeyExchangeResponse> {
    let secure_random = ring::rand::SystemRandom::new();

    // generate key pair for passive device key exchange reply
    let reply_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096)?;
    let reply_public_key = reply_private_key.to_public_key();

    // generate exchange key pair and nonce
    let active_exchange_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &secure_random)?;
    let active_exchange_public_key = active_exchange_private_key.compute_public_key()?;

    let mut active_exchange_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_exchange_nonce);

    // generate and sealing active device key exchange secret
    let active_device_secret = KeyExchangeActiveDeviceSecret {
        exchange_reply_public_key_n: reply_public_key.n().to_bytes_le(),
        exchange_reply_public_key_e: reply_public_key.e().to_bytes_le(),
        active_exchange_public_key: active_exchange_public_key.as_ref().to_owned(),
        active_exchange_nonce: active_exchange_nonce.to_vec(),
    };

    // generate secret sealing key with salt
    let mut active_device_secret_salt = [0u8; 16];
    OsRng.fill_bytes(&mut active_device_secret_salt);

    let mut active_device_secret_sealing_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        req.password.as_bytes(),
        &active_device_secret_salt,
        10000,
        &mut active_device_secret_sealing_key,
    );

    let mut active_device_secret_buffer = active_device_secret.encode_to_vec();

    let active_device_secret_sealing_unbound_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &active_device_secret_sealing_key)?;

    let mut active_device_secret_sealing_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_device_secret_sealing_nonce);

    let mut active_device_secret_sealing_key = ring::aead::SealingKey::new(
        active_device_secret_sealing_unbound_key,
        NonceValue::new(active_device_secret_sealing_nonce),
    );

    active_device_secret_sealing_key.seal_in_place_append_tag(
        ring::aead::Aad::from(req.local_device_id.as_bytes()),
        &mut active_device_secret_buffer,
    )?;

    let resp = SignalingClientManager::get_client()
        .await?
        .key_exchange(signaling_proto::KeyExchangeRequest {
            active_device_id: req.local_device_id.to_owned(),
            passive_device_id: req.remote_device_id.to_owned(),
            password_salt: active_device_secret_salt.to_vec(),
            secret: active_device_secret_buffer,
            secret_nonce: active_device_secret_sealing_nonce.to_vec(),
        })
        .await?;

    // acquire key exchange
    let key_exchange_response = resp.get_ref();
    if key_exchange_response.active_device_id != req.local_device_id
        || key_exchange_response.passive_device_id != req.remote_device_id
    {
        return Err(core_error!("mismatched key exchange response"));
    }

    let passive_device_secret_buffer = reply_private_key.decrypt(
        rsa::PaddingScheme::PKCS1v15Encrypt,
        &key_exchange_response.secret,
    )?;

    let passive_device_secret =
        KeyExchangePassiveDeviceSecret::decode(passive_device_secret_buffer.as_ref())?;

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
                &passive_device_secret.passive_exchange_nonce,
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

    // let mut passive_exchange_nonce = [0u8; ring::aead::NONCE_LEN];
    // for i in 0..ring::aead::NONCE_LEN {
    //     passive_exchange_nonce[i] = passive_device_secret.passive_exchange_nonce[i];
    // }

    // create opening and sealing key

    // let unbound_sealing_key =
    //     ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)?;

    // let sealing_key =
    //     ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_exchange_nonce));

    // let unbound_opening_key =
    //     ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)?;

    // let opening_key =
    //     ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_exchange_nonce));

    Ok(KeyExchangeResponse {
        opening_key_bytes: raw_sealing_key,
        opening_nonce_bytes: passive_device_secret.passive_exchange_nonce,
        sealing_key_bytes: raw_opening_key,
        sealing_nonce_bytes: active_exchange_nonce.to_vec(),
    })
}
