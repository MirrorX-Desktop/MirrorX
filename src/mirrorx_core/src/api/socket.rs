use crate::provider::config::ConfigProvider;
use crate::provider::endpoint::EndPointProvider;
use crate::socket::endpoint::CacheKey;
use crate::socket::endpoint::EndPoint;
use crate::socket::message::client_to_client::ConnectRequest;
use crate::socket::message::client_to_client::KeyExchangeAndVerifyPasswordRequest;
use crate::socket::message::client_to_client::StartMediaTransmissionReply;
use crate::socket::message::client_to_client::StartMediaTransmissionRequest;
use anyhow::anyhow;
use log::trace;
use once_cell::sync::Lazy;
use rand::thread_rng;
use ring::rand::SecureRandom;
use rsa::BigUint;
use rsa::PublicKey;
use rsa::RsaPublicKey;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub async fn desktop_connect(remote_device_id: String) -> anyhow::Result<()> {
    static CONNECT_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    CONNECT_MUTEX.lock().await;

    let endpoint = match EndPointProvider::current()?.get(&remote_device_id) {
        Some(endpoint) => endpoint,
        None => {
            let local_device_id = ConfigProvider::current()?
                .read_device_id()?
                .ok_or(anyhow::anyhow!("desktop_connect: local device id not set"))?;

            let endpoint = Arc::new(EndPoint::new(local_device_id, remote_device_id.clone()));
            EndPointProvider::current()?.insert(remote_device_id, endpoint.clone());
            endpoint
        }
    };

    let resp = endpoint
        .desktop_connect(ConnectRequest {}, Duration::from_secs(20))
        .await?;

    let n = BigUint::from_bytes_le(&resp.pub_key_n);
    let e = BigUint::from_bytes_le(&resp.pub_key_e);

    let public_key = RsaPublicKey::new(n, e)?;

    endpoint
        .cache()
        .set(CacheKey::PasswordVerifyPublicKey, public_key);

    Ok(())
}

pub async fn desktop_key_exchange_and_password_verify(
    remote_device_id: String,
    password: String,
) -> anyhow::Result<bool> {
    let endpoint = EndPointProvider::current()?
        .get(&remote_device_id)
        .ok_or(anyhow!(
            "desktop_key_exchange_and_password_verify: remote device '{}' already connected",
            &remote_device_id
        ))?;

    let remote_device_pub_key = endpoint
        .cache()
        .take::<RsaPublicKey>(CacheKey::PasswordVerifyPublicKey)
        .ok_or(anyhow!(
            "desktop_key_exchange_and_password_verify: verify password public key not exists"
        ))?;

    let mut rng = thread_rng();
    let password_secret = remote_device_pub_key
        .encrypt(
            &mut rng,
            rsa::PaddingScheme::PKCS1v15Encrypt,
            &password.as_bytes(),
        )
        .map_err(|err| {
            anyhow!(
                "desktop_key_exchange_and_password_verify: encrypt password failed: {}",
                err
            )
        })?;

    let ephemeral_rng = ring::rand::SystemRandom::new();
    let local_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &ephemeral_rng)
            .map_err(|err| {
                anyhow!(
            "desktop_key_exchange_and_password_verify: generate ephemeral private key failed: {}",
            err
        )
            })?;

    let local_public_key = local_private_key.compute_public_key().map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: compute public key failed: {}",
            err
        )
    })?;

    let exchange_pub_key = local_public_key.as_ref().to_vec();

    let mut exchange_salt = Vec::<u8>::new();
    exchange_salt.resize(32, 0);
    ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: generate exchange salt failed: {}",
            err
        )
    })?;

    let resp = endpoint
        .desktop_key_exchange_and_verify_password(
            KeyExchangeAndVerifyPasswordRequest {
                password_secret,
                exchange_pub_key,
                exchange_salt: exchange_salt.clone(),
            },
            Duration::from_secs(20),
        )
        .await?;

    if !resp.password_correct {
        EndPointProvider::current()?.remove(&remote_device_id);
        return Ok(false);
    }

    let remote_public_key =
        ring::agreement::UnparsedPublicKey::new(&ring::agreement::X25519, &resp.exchange_pub_key);

    let (sealing_key, opening_key) = ring::agreement::agree_ephemeral(
        local_private_key,
        &remote_public_key,
        ring::error::Unspecified,
        |key_material| {
            let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::new();
                    key.resize(ring::aead::CHACHA20_POLY1305.key_len(), 0);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &resp.exchange_salt)
                .extract(key_material)
                .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                .and_then(|orm| {
                    let mut key = Vec::<u8>::new();
                    key.resize(ring::aead::CHACHA20_POLY1305.key_len(), 0);
                    orm.fill(&mut key)?;
                    Ok(key)
                })?;

            Ok((send_key, recv_key))
        },
    )
    .map_err(|err| {
        anyhow!(
            "desktop_key_exchange_and_password_verify: agree ephemeral key failed: {:?}",
            err
        )
    })?;

    // initial endpoint opening(recv) key
    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &opening_key).map_err(
            |err| {
                anyhow::anyhow!(
                    "key_exchange_and_verify_password: create unbounded key for opening failed: {}",
                    err
                )
            },
        )?;

    let opening_initial_nonce =
        unsafe { u64::from_le_bytes(*(exchange_salt[..8].as_ptr() as *const [u8; 8])) };

    endpoint
        .set_opening_key(unbound_opening_key, opening_initial_nonce)
        .await;

    // initial endpoint sealing(send) key
    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &sealing_key).map_err(
            |err| {
                anyhow::anyhow!(
                    "key_exchange_and_verify_password: create unbounded key for sealing failed: {}",
                    err
                )
            },
        )?;

    let sealing_initial_nonce =
        unsafe { u64::from_le_bytes(*(resp.exchange_salt[..8].as_ptr() as *const [u8; 8])) };

    endpoint
        .set_sealing_key(unbound_sealing_key, sealing_initial_nonce)
        .await;

    trace!("key exchange and password verify success");

    trace!("sealing key: {:X?}", sealing_key);
    trace!("opening key: {:X?}", opening_key);

    trace!("opening initial nonce: {}", opening_initial_nonce);
    trace!("sealing initial nonce: {}", sealing_initial_nonce);

    Ok(true)
}

pub async fn desktop_start_media_transmission(
    remote_device_id: String,
) -> anyhow::Result<StartMediaTransmissionReply> {
    let endpoint = EndPointProvider::current()?
        .get(&remote_device_id)
        .ok_or(anyhow!(
            "desktop_start_media_transmission: remote device '{}' already connected",
            &remote_device_id
        ))?;

    let resp = endpoint
        .desktop_start_media_transmission(StartMediaTransmissionRequest {}, Duration::from_secs(10))
        .await?;

    Ok(resp)
}
