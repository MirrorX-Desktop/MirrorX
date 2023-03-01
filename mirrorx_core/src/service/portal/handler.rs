use super::message::{
    ActiveEndpointKeyExchangeSecret, PassiveEndpointKeyExchangeSecret, PortalClientMessage,
    PortalError, VisitPassiveReply, VisitPassiveRequest,
};
use crate::{
    service::{
        config,
        endpoint::{create_endpoint_client, EndPointID},
    },
    utility::{
        bincode::{bincode_deserialize, bincode_serialize},
        nonce_value::NonceValue,
    },
};
use hmac::Hmac;
use rand::RngCore;
use ring::aead::{BoundKey, OpeningKey, SealingKey, UnboundKey};
use rsa::{rand_core::OsRng, BigUint, PublicKey};
use sha2::Sha256;
use std::sync::{atomic::AtomicI64, Arc};

#[allow(clippy::too_many_arguments)]
pub async fn handle_passive_visit_request(
    domain_id: Arc<AtomicI64>,
    storage: config::service::Service,
    req: VisitPassiveRequest,
) -> PortalClientMessage {
    let visit_credentials = uuid::Uuid::new_v4();
    let mut reply = VisitPassiveReply {
        active_device_id: req.active_visit_req.active_device_id,
        passive_device_id: req.active_visit_req.passive_device_id,
        visit_credentials: visit_credentials.to_string(),
        access_result: Err(PortalError::Internal),
    };

    if let Ok(domain) = storage
        .domain()
        .get_domain_by_id(domain_id.load(std::sync::atomic::Ordering::SeqCst))
    {
        match key_agreement(
            &domain.password,
            req.active_visit_req.active_device_id,
            req.active_visit_req.password_salt,
            req.active_visit_req.secret,
            req.active_visit_req.secret_nonce,
        )
        .await
        {
            Ok((secret, sealing_key, opening_key)) => {
                tokio::spawn(async move {
                    if let Err(err) = create_endpoint_client(
                        EndPointID::DeviceID {
                            local_device_id: req.active_visit_req.passive_device_id,
                            remote_device_id: req.active_visit_req.active_device_id,
                        },
                        Some((opening_key, sealing_key)),
                        crate::service::endpoint::EndPointStream::ActiveTCP(req.relay_addr),
                        Some(visit_credentials.as_bytes().to_vec()),
                    )
                    .await
                    {
                        tracing::error!(?err, "create passive endpoint client failed");
                    }
                });

                reply.access_result = Ok(secret)
            }
            Err(err) => reply.access_result = Err(err),
        };
    };

    PortalClientMessage::VisitPassiveReply(reply)
}

async fn key_agreement(
    domain_password: &str,
    active_device_id: i64,
    password_salt: Vec<u8>,
    mut secret: Vec<u8>,
    secret_nonce: Vec<u8>,
) -> Result<(Vec<u8>, SealingKey<NonceValue>, OpeningKey<NonceValue>), PortalError> {
    if secret_nonce.len() != ring::aead::NONCE_LEN {
        return Err(PortalError::InvalidRequest);
    }

    // generate secret opening key with salt
    let mut active_device_secret_opening_key = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(
        domain_password.as_bytes(),
        &password_salt,
        10000,
        &mut active_device_secret_opening_key,
    );

    let unbound_key = match ring::aead::UnboundKey::new(
        &ring::aead::AES_256_GCM,
        &active_device_secret_opening_key,
    ) {
        Ok(unbound_key) => unbound_key,
        Err(err) => {
            tracing::error!(?err, "create unbound key failed");
            return Err(PortalError::InvalidRequest);
        }
    };

    let mut active_device_secret_opening_nonce = [0u8; ring::aead::NONCE_LEN];
    active_device_secret_opening_nonce[..ring::aead::NONCE_LEN]
        .copy_from_slice(&secret_nonce[..ring::aead::NONCE_LEN]);

    let mut active_device_secret_opening_key = ring::aead::OpeningKey::new(
        unbound_key,
        NonceValue::new(active_device_secret_opening_nonce),
    );

    let active_device_secret_buffer = match active_device_secret_opening_key.open_in_place(
        ring::aead::Aad::from(active_device_id.to_le_bytes()),
        &mut secret,
    ) {
        Ok(buffer) => buffer,
        Err(_) => return Err(PortalError::RemoteRefuse),
    };

    let active_device_secret: ActiveEndpointKeyExchangeSecret =
        match bincode_deserialize(&*active_device_secret_buffer) {
            Ok(secret) => secret,
            Err(_) => {
                return Err(PortalError::InvalidRequest);
            }
        };

    if active_device_secret.active_exchange_nonce.len() != ring::aead::NONCE_LEN {
        return Err(PortalError::InvalidRequest);
    }

    // generate passive device key exchange pair and nonce

    let system_random_rng = ring::rand::SystemRandom::new();

    let passive_exchange_private_key = match ring::agreement::EphemeralPrivateKey::generate(
        &ring::agreement::X25519,
        &system_random_rng,
    ) {
        Ok(private_key) => private_key,
        Err(_) => return Err(PortalError::Internal),
    };

    let passive_exchange_public_key = match passive_exchange_private_key.compute_public_key() {
        Ok(public_key) => public_key,
        Err(err) => {
            tracing::error!(
                ?err,
                "compute public key from passive exchange private key failed"
            );
            return Err(PortalError::Internal);
        }
    };

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

    let agree_result = ring::agreement::agree_ephemeral(
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
                active_device_secret.active_exchange_nonce,
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
    );

    let (raw_sealing_key, raw_opening_key) = match agree_result {
        Ok(v) => v,
        Err(err) => {
            tracing::error!(?err, "agree ephemeral failed");
            return Err(PortalError::Internal);
        }
    };

    // derive opening and sealing key

    let unbound_sealing_key = match UnboundKey::new(&ring::aead::AES_256_GCM, &raw_sealing_key) {
        Ok(unbound_sealing_key) => unbound_sealing_key,
        Err(err) => {
            tracing::error!(?err, "create unbound sealing key failed");
            return Err(PortalError::Internal);
        }
    };

    let sealing_key = SealingKey::new(unbound_sealing_key, NonceValue::new(active_exchange_nonce));

    let unbound_opening_key = match UnboundKey::new(&ring::aead::AES_256_GCM, &raw_opening_key) {
        Ok(unbound_opening_key) => unbound_opening_key,
        Err(err) => {
            tracing::error!(?err, "create unbound opening failed");
            return Err(PortalError::Internal);
        }
    };

    let opening_key =
        ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(passive_exchange_nonce));

    // build key exchange response

    let passive_device_secret = PassiveEndpointKeyExchangeSecret {
        passive_exchange_public_key: passive_exchange_public_key.as_ref(),
        passive_exchange_nonce: &passive_exchange_nonce,
    };

    let passive_device_secret_buffer = match bincode_serialize(&passive_device_secret) {
        Ok(buffer) => buffer,
        Err(_) => return Err(PortalError::Internal),
    };

    let active_exchange_reply_public_key = match rsa::RsaPublicKey::new(
        BigUint::from_bytes_le(active_device_secret.exchange_reply_public_key_n),
        BigUint::from_bytes_le(active_device_secret.exchange_reply_public_key_e),
    ) {
        Ok(public_key) => public_key,
        Err(err) => {
            tracing::error!(?err, "recover exchange reply public key failed");
            return Err(PortalError::Internal);
        }
    };

    let secret_buffer = match active_exchange_reply_public_key.encrypt(
        &mut OsRng,
        rsa::Pkcs1v15Encrypt::default(),
        &passive_device_secret_buffer,
    ) {
        Ok(buffer) => buffer,
        Err(err) => {
            tracing::error!(?err, "encrypt exchange reply data failed");
            return Err(PortalError::Internal);
        }
    };

    Ok((secret_buffer, sealing_key, opening_key))
}
