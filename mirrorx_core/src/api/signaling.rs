use crate::{
    core_error,
    error::{CoreError, CoreResult},
    proto::signaling::{
        signaling_client::SignalingClient, ConnectDeviceRequest, HeartbeatRequest,
        KeyExchangeActiveDeviceSecret, KeyExchangePassiveDeviceSecret, KeyExchangeRequest,
        RegisterRequest,
    },
    service::signaling::message::HeartBeatRequest,
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use arc_swap::ArcSwapOption;
use bincode::Options;
use flutter_rust_bridge::support::lazy_static;
use hmac::Hmac;
use once_cell::sync::Lazy;
use prost::Message;
use rand::{rngs::OsRng, RngCore};
use ring::{aead::BoundKey, pbkdf2::PBKDF2_HMAC_SHA256};
use rsa::PublicKeyParts;
use sha2::Sha256;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tonic::{
    codegen::CompressionEncoding,
    transport::{Channel, Uri},
};

static CURRENT_SIGNALING_CLIENT: Lazy<Mutex<Option<SignalingClient<Channel>>>> =
    Lazy::new(|| Mutex::new(None));

pub async fn signaling_connect(uri: Uri) -> CoreResult<()> {
    let channel = Channel::builder(uri)
        .tcp_nodelay(true)
        .keep_alive_timeout(Duration::from_secs(60))
        .rate_limit(5, Duration::from_secs(1))
        .connect_timeout(Duration::from_secs(10))
        .keep_alive_while_idle(true)
        .initial_connection_window_size(256 * 1024 * 1024)
        .initial_stream_window_size(32 * 1024 * 1024);

    let client = SignalingClient::connect(channel).await?;
    let client = client
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    let mut current_client = CURRENT_SIGNALING_CLIENT.lock().await;
    *current_client = Some(client);

    Ok(())
}

pub async fn disconnect() {
    let mut current_client = CURRENT_SIGNALING_CLIENT.lock().await;
    *current_client = None;
}

pub fn begin_heartbeat() {
    TOKIO_RUNTIME.spawn(async move {
        let mut failures = 0;
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;

            let mut current_client = CURRENT_SIGNALING_CLIENT.lock().await;
            if let Some(client) = current_client.as_mut() {
                if let Ok(_) = client
                    .heartbeat(HeartbeatRequest {
                        timestamp: chrono::Utc::now().timestamp() as u32,
                    })
                    .await
                {
                    failures = 0;
                } else {
                    failures += 1;
                    if failures == 5 {
                        break;
                    }
                }
            }
        }

        tracing::error!("too many failures while heart beat, disconnect");

        TOKIO_RUNTIME.spawn(async { disconnect().await });
    });
}

pub async fn handshake() -> CoreResult<()> {
    let mut client = CURRENT_SIGNALING_CLIENT.lock().await;
    let client = client
        .as_mut()
        .ok_or(core_error!("signaling client not exists"))?;

    let device_id = crate::api::config::read_device_id()?;
    let device_hash = match crate::api::config::read_device_hash()? {
        Some(v) => v,
        None => {
            let mut device_hash = [0u8; 512];
            OsRng.fill_bytes(&mut device_hash);
            hex::encode_upper(device_hash)
        }
    };

    let resp = client
        .register(RegisterRequest {
            device_id,
            device_fingerprint: device_hash.clone(),
            device_public_key: String::from(""),
        })
        .await?;

    let resp = resp.get_ref();

    crate::api::config::save_device_id(&resp.device_id)?;
    crate::api::config::save_device_hash(&device_hash)?;
    crate::api::config::save_device_id_expiration(&resp.device_id_expire)?;

    Ok(())
}

pub async fn connect_remote_device(remote_device_id: String) -> CoreResult<bool> {
    let mut client = CURRENT_SIGNALING_CLIENT.lock().await;
    let client = client
        .as_mut()
        .ok_or(core_error!("signaling client not exists"))?;

    let resp = client
        .connect_device(ConnectDeviceRequest { remote_device_id })
        .await?;

    Ok(resp.get_ref().allow)
}

pub async fn connection_key_exchange(
    remote_device_id: String,
    password: String,
) -> CoreResult<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> {
    let mut client = CURRENT_SIGNALING_CLIENT.lock().await;
    let client = client
        .as_mut()
        .ok_or(core_error!("signaling client not exists"))?;

    let local_device_id = match crate::api::config::read_device_id()? {
        Some(id) => id,
        None => return Err(core_error!("local device id is None")),
    };

    // generate rsa key pair for remote device reply
    // let response_private_key = rsa::RsaPrivateKey::new(&mut OsRng, 4096)
    //     .map_err(|err| (core_error!("generate response private key failed ({})", err)))?;

    // let response_public_key = rsa::RsaPublicKey::from(&response_private_key);

    // // generate key exchange pair and nonce

    // let system_random_rng = ring::rand::SystemRandom::new();

    // let active_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
    //     &ring::agreement::X25519,
    //     &system_random_rng,
    // )
    // .map_err(|_| core_error!("generate key exchange private key failed"))?;

    // let active_device_public_key = active_device_private_key
    //     .compute_public_key()
    //     .map_err(|_| core_error!("generate key exchange public key failed"))?;

    // let mut active_device_nonce = [0u8; ring::aead::NONCE_LEN];
    // OsRng.fill_bytes(&mut active_device_nonce);

    // // derive aes-256-gcm key from password

    // let mut password_derive_salt = [0u8; 16];
    // OsRng.fill_bytes(&mut password_derive_salt);

    // let mut derived_key = [0u8; 32];
    // pbkdf2::pbkdf2::<Hmac<Sha256>>(
    //     password.as_bytes(),
    //     &password_derive_salt,
    //     pbkdf2::Params::default().rounds,
    //     &mut derived_key,
    // );

    // // build secret

    // let mut secret_nonce = [0u8; ring::aead::NONCE_LEN];
    // OsRng.fill_bytes(&mut secret_nonce);

    // let active_device_secret = ConnectionKeyExchangeActiveDeviceSecret {
    //     response_public_key_n: response_public_key.n().to_bytes_le(),
    //     response_public_key_e: response_public_key.e().to_bytes_le(),
    //     active_device_public_key: active_device_public_key.as_ref(),
    //     active_device_nonce: &active_device_nonce,
    // };

    // let mut active_device_secret_buffer = BINCODE_SERIALIZER.serialize(&active_device_secret)?;

    // // sealing packet and call key-exchange

    // let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &derived_key)
    //     .map_err(|_| core_error!("generate request message packet sealing key failed"))?;

    // let mut sealing_key = ring::aead::SealingKey::new(unbound_key, NonceValue::new(secret_nonce));

    // sealing_key
    //     .seal_in_place_append_tag(ring::aead::Aad::empty(), &mut active_device_secret_buffer)
    //     .map_err(|_| core_error!("sealing request message packet failed"))?;

    // let resp = CURRENT_SIGNALING_CLIENT
    //     .load()
    //     .as_ref()
    //     .ok_or(core_error!("current signaling client not exists"))?
    //     .connection_key_exchange(
    //         Some(remote_device_id.clone()),
    //         ConnectionKeyExchangeRequest {
    //             active_device_id: local_device_id.clone(),
    //             password_derive_salt: password_derive_salt.to_vec(),
    //             secret: active_device_secret_buffer,
    //             secret_nonce: secret_nonce.to_vec(),
    //         },
    //     )
    //     .await?;

    // if resp.passive_device_id != remote_device_id {
    //     return Err(core_error!(
    //         "key exchang responsed device is not aimed device"
    //     ));
    // }

    // // handle key exchange response

    // let passive_device_secret_buffer = response_private_key
    //     .decrypt(
    //         rsa::PaddingScheme::PKCS1v15Encrypt,
    //         resp.exchange_data.as_ref(),
    //     )
    //     .map_err(|err| core_error!("decrypt key exchange data failed ({})", err))?;

    // let passive_device_secret = BINCODE_SERIALIZER
    //     .deserialize::<ConnectionKeyExchangePassiveDeviceSecret>(&passive_device_secret_buffer)?;

    // if passive_device_secret.passive_device_nonce.len() != ring::aead::NONCE_LEN {
    //     return Err(core_error!(
    //         "passive device provide invalid key exchange nonce"
    //     ));
    // }

    // let passive_device_public_key = ring::agreement::UnparsedPublicKey::new(
    //     &ring::agreement::X25519,
    //     passive_device_secret.passive_device_public_key,
    // );

    // let (raw_sealing_key, raw_opening_key) = ring::agreement::agree_ephemeral(
    //     active_device_private_key,
    //     &passive_device_public_key,
    //     ring::error::Unspecified,
    //     |key_material| {
    //         let sealing_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &active_device_nonce)
    //             .extract(key_material)
    //             .expand(&["".as_bytes()], &ring::aead::AES_256_GCM)
    //             .and_then(|orm| {
    //                 let mut key = Vec::<u8>::new();
    //                 key.resize(ring::aead::AES_256_GCM.key_len(), 0);
    //                 orm.fill(&mut key)?;
    //                 Ok(key)
    //             })?;

    //         let opening_key = ring::hkdf::Salt::new(
    //             ring::hkdf::HKDF_SHA512,
    //             &passive_device_secret.passive_device_nonce,
    //         )
    //         .extract(key_material)
    //         .expand(&["".as_bytes()], &ring::aead::AES_256_GCM)
    //         .and_then(|orm| {
    //             let mut key = Vec::<u8>::new();
    //             key.resize(ring::aead::AES_256_GCM.key_len(), 0);
    //             orm.fill(&mut key)?;
    //             Ok(key)
    //         })?;

    //         Ok((sealing_key, opening_key))
    //     },
    // )
    // .map_err(|_| core_error!("key exchange agree ephemeral failed"))?;

    // let mut passive_device_nonce = [0u8; ring::aead::NONCE_LEN];
    // for i in 0..ring::aead::NONCE_LEN {
    //     passive_device_nonce[i] = passive_device_secret.passive_device_nonce[i];
    // }

    // // derive opening and sealing key

    // let unbound_sealing_key =
    //     ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)
    //         .map_err(|_| core_error!("generate unbound sealing key failed"))?;

    // let sealing_key =
    //     ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(passive_device_nonce));

    // let unbound_opening_key =
    //     ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)
    //         .map_err(|_| core_error!("generate unbound opening key failed"))?;

    // let opening_key =
    //     ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(active_device_nonce));

    // crate::api::endpoint::connect(
    //     true,
    //     local_device_id,
    //     remote_device_id,
    //     sealing_key,
    //     opening_key,
    // )
    // .await?;

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

    // derive passive device password
    let mut active_device_secret_salt = [0u8; 16];
    OsRng.fill_bytes(&mut active_device_secret_salt);

    let mut active_device_secret_derived_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        &active_device_secret_salt,
        10000,
        &mut active_device_secret_derived_key,
    );

    // generate and sealing active device key exchange secret
    let active_device_secret = KeyExchangeActiveDeviceSecret {
        exchange_reply_public_key_n: reply_public_key.n().to_bytes_le(),
        exchange_reply_public_key_e: reply_public_key.e().to_bytes_le(),
        active_exchange_public_key: active_exchange_public_key.as_ref().to_owned(),
        active_exchange_nonce: active_exchange_nonce.to_vec(),
    };

    let mut active_device_secret_buffer = active_device_secret.encode_to_vec();

    let active_device_secret_sealing_unbound_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &active_device_secret_derived_key)?;

    let mut active_device_secret_sealing_nonce = [0u8; ring::aead::NONCE_LEN];
    OsRng.fill_bytes(&mut active_device_secret_sealing_nonce);

    let mut active_device_secret_sealing_key = ring::aead::SealingKey::new(
        active_device_secret_sealing_unbound_key,
        NonceValue::new(active_device_secret_sealing_nonce),
    );

    active_device_secret_sealing_key.seal_in_place_append_tag(
        ring::aead::Aad::from(local_device_id.as_bytes()),
        &mut active_device_secret_buffer,
    )?;

    let resp = client
        .key_exchange(KeyExchangeRequest {
            active_device_id: local_device_id.to_owned(),
            passive_device_id: remote_device_id.to_owned(),
            secret_salt: active_device_secret_salt.to_vec(),
            secret: active_device_secret_buffer,
        })
        .await?;

    // acquire key exchange
    let key_exchange_response = resp.get_ref();
    if key_exchange_response.active_device_id != local_device_id
        || key_exchange_response.passive_device_id != remote_device_id
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

    Ok((
        raw_sealing_key,
        passive_device_secret.passive_exchange_nonce,
        raw_opening_key,
        active_exchange_nonce.to_vec(),
    ))
}
