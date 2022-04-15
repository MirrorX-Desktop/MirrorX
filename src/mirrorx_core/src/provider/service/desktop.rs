use super::{
    message::{
        reply::{ConnectReply, KeyExchangeAndVerifyPasswordReply},
        reply_error::ReplyError,
        request::{ConnectRequest, KeyExchangeAndVerifyPasswordRequest},
    },
    network::client::Client,
};
use log::{error, info};
use ring::rand::SecureRandom;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;

pub struct DesktopService {}

impl DesktopService {
    pub fn new() -> Self {
        DesktopService {}
    }

    pub async fn connect(
        &self,
        client: Arc<Client>,
        req: ConnectRequest,
    ) -> Result<ConnectReply, ReplyError> {
        info!("handle connect, client: {}", client.device_id());

        let mut rng = rand::thread_rng();
        let priv_key = match RsaPrivateKey::new(&mut rng, 4096) {
            Ok(key) => key,
            Err(err) => {
                error!("generate rsa private key failed: {:?}", err);
                return Err(ReplyError::Internal);
            }
        };

        let pub_key = RsaPublicKey::from(&priv_key);
        let n = pub_key.n().to_bytes_le();
        let e = pub_key.e().to_bytes_le();

        client.store_verify_password_priv_key(req.offer_device_id.clone(), priv_key);

        Ok(ConnectReply {
            offer_device_id: req.offer_device_id,
            ask_device_id: req.ask_device_id,
            pub_key_n: n,
            pub_key_e: e,
        })
    }

    pub async fn key_exchange_and_verify_password(
        &self,
        client: Arc<Client>,
        req: KeyExchangeAndVerifyPasswordRequest,
    ) -> Result<KeyExchangeAndVerifyPasswordReply, ReplyError> {
        info!(
            "handle key_exchange_and_verify_password, client: {}",
            client.device_id()
        );

        // todo: check white list

        let password = crate::instance::CONFIG_PROVIDER_INSTANCE
            .get()
            .ok_or_else(|| ReplyError::Internal)
            .and_then(|provider| {
                match provider.read_device_password().map_err(|err| {
                    error!("read device password failed: {:?}", err);
                    ReplyError::Internal
                })? {
                    Some(password) => Ok(password),
                    None => return Err(ReplyError::Internal),
                }
            })?;

        let priv_key = match client.remove_verify_password_priv_key(&req.offer_device_id) {
            Some(key) => key,
            None => return Err(ReplyError::NotSatisfied),
        };

        let req_password =
            match priv_key.decrypt(PaddingScheme::PKCS1v15Encrypt, &req.password_secret) {
                Ok(password) => password,
                Err(err) => {
                    error!("decrypt password failed: {:?}", err);
                    return Err(ReplyError::Internal);
                }
            };

        if req_password != Vec::from(password) {
            return Err(ReplyError::PasswordIncorrect);
        }

        // gen key exchange
        let ephemeral_rng = ring::rand::SystemRandom::new();
        let local_private_key = ring::agreement::EphemeralPrivateKey::generate(
            &ring::agreement::X25519,
            &ephemeral_rng,
        )
        .map_err(|err| {
            error!("failed to generate ephemeral private key: {:?}", err);
            ReplyError::Internal
        })?;

        let local_public_key = local_private_key.compute_public_key().map_err(|err| {
            error!("failed to compute public key: {:?}", err);
            ReplyError::Internal
        })?;

        let exchange_pub_key = local_public_key.as_ref().to_vec();

        let mut exchange_salt = Vec::<u8>::with_capacity(32);
        ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
            error!("failed to generate exchange salt: {:?}", err);
            ReplyError::Internal
        })?;

        let remote_public_key = ring::agreement::UnparsedPublicKey::new(
            &ring::agreement::X25519,
            &req.exchange_pub_key,
        );

        let (send_key, recv_key) = ring::agreement::agree_ephemeral(
            local_private_key,
            &remote_public_key,
            ring::error::Unspecified,
            |key_material| {
                let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &req.exchange_salt)
                    .extract(key_material)
                    .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                    .and_then(|orm| {
                        let mut key = Vec::<u8>::with_capacity(32);
                        if let Err(err) = orm.fill(&mut key) {
                            error!("failed to fill key for send: {:?}", err);
                            return Err(ring::error::Unspecified);
                        };
                        Ok(key)
                    })?;

                let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
                    .extract(key_material)
                    .expand(&["".as_bytes()], &ring::aead::CHACHA20_POLY1305)
                    .and_then(|orm| {
                        let mut key = Vec::<u8>::with_capacity(32);
                        if let Err(err) = orm.fill(&mut key) {
                            error!("failed to fill key for recv: {:?}", err);
                            return Err(ring::error::Unspecified);
                        };
                        Ok(key)
                    })?;

                Ok((send_key, recv_key))
            },
        )
        .map_err(|err| {
            error!("failed to agree ephemeral key: {:?}", err);
            ReplyError::Internal
        })?;

        Ok(KeyExchangeAndVerifyPasswordReply {
            offer_device_id: req.offer_device_id,
            ask_device_id: req.ask_device_id,
            exchange_pub_key,
            exchange_salt,
        })
    }
}
