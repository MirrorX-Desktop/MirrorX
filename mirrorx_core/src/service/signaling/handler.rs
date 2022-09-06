use super::message::{
    ConnectRequest, ConnectResponse, ConnectionKeyExchangeActiveDeviceSecret,
    ConnectionKeyExchangePassiveDeviceSecret, ConnectionKeyExchangeRequest,
    ConnectionKeyExchangeResponse,
};
use crate::{
    api, core_error,
    error::{CoreError, CoreResult},
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use hmac::Hmac;
use rand::RngCore;
use ring::aead::BoundKey;
use rsa::{rand_core::OsRng, BigUint, PublicKey};
use sha2::Sha256;

pub async fn handle_connect_request(req: ConnectRequest) -> CoreResult<ConnectResponse> {
    Ok(ConnectResponse { allow: true })
}

pub async fn handle_connection_key_exchange_request(
    mut req: ConnectionKeyExchangeRequest,
) -> CoreResult<ConnectionKeyExchangeResponse> {
    let passive_device_id = match api::config::read_device_id()? {
        Some(id) => id,
        None => return Err(core_error!("local device is None")),
    };

    let password = match api::config::read_device_password()? {
        Some(password) => password,
        None => return Err(core_error!("local device password is None")),
    };

    if req.secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(core_error!("active device secret nonce is invalid"));
    }

    // try to decrypt secret

    let mut derived_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        &req.password_derive_salt,
        pbkdf2::Params::default().rounds,
        &mut derived_key,
    );

    let unbound_key = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &derived_key)
        .map_err(|_| core_error!("create unbound key from request failed"))?;

    let mut active_device_secret_nonce = [0u8; ring::aead::NONCE_LEN];
    for i in 0..ring::aead::NONCE_LEN {
        active_device_secret_nonce[i] = req.secret_nonce[i];
    }

    let mut active_device_secret_opening_key =
        ring::aead::OpeningKey::new(unbound_key, NonceValue::new(active_device_secret_nonce));

    let active_device_secret_buf = active_device_secret_opening_key
        .open_in_place(ring::aead::Aad::empty(), &mut req.secret)
        .map_err(|_| core_error!("opening key exchange request message failed"))?;

    let active_device_secret = BINCODE_SERIALIZER
        .deserialize::<ConnectionKeyExchangeActiveDeviceSecret>(&active_device_secret_buf)?;

    if active_device_secret.active_device_nonce.len() != ring::aead::NONCE_LEN {
        return Err(core_error!("active device key exchange nonce is invalid"));
    }

    let active_device_response_public_key = rsa::RsaPublicKey::new(
        BigUint::from_bytes_le(&active_device_secret.response_public_key_n),
        BigUint::from_bytes_le(&active_device_secret.response_public_key_e),
    )
    .map_err(|err| core_error!("create response public key from request failed ({})", err))?;

    // generate key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_device_private_key = ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    )
    .map_err(|_| core_error!("generate key exchange private key failed"))?;

    let passive_device_public_key = passive_device_private_key
        .compute_public_key()
        .map_err(|_| core_error!("generate key exchange public key failed"))?;

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
    .map_err(|_| core_error!("key exchange agree ephemeral failed"))?;

    // derive opening and sealing key

    let unbound_sealing_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key)
            .map_err(|_| core_error!("generate unbound sealing key failed"))?;

    let sealing_key =
        ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(active_device_nonce));

    let unbound_opening_key =
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key)
            .map_err(|_| core_error!("generate unbound opening key failed"))?;

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(passive_device_nonce));

    // create endpoint
    let local_device_id = passive_device_id.clone();

    TOKIO_RUNTIME.spawn(async move {
        if let Err(err) = api::endpoint::connect(
            false,
            local_device_id,
            req.active_device_id.clone(),
            sealing_key,
            opening_key,
        )
        .await
        {
            tracing::error!(err=?err, active_device_id=?req.active_device_id.clone(), "create endpoint failed");
        }
    });

    // encrypt response inner passive device secret

    let passive_device_secret = ConnectionKeyExchangePassiveDeviceSecret {
        passive_device_public_key: passive_device_public_key.as_ref(),
        passive_device_nonce: &passive_device_nonce,
    };

    let passive_secret_buffer = BINCODE_SERIALIZER.serialize(&passive_device_secret)?;

    let exchange_data = active_device_response_public_key
        .encrypt(
            &mut OsRng,
            rsa::PaddingScheme::PKCS1v15Encrypt,
            &passive_secret_buffer,
        )
        .map_err(|err| {
            core_error!(
                "encrypt key exchange response message packet failed ({})",
                err
            )
        })?;

    Ok(ConnectionKeyExchangeResponse {
        passive_device_id,
        exchange_data,
    })
}
