use super::api_error::APIError;
use crate::provider::service::message::request::KeyExchangeAndVerifyPasswordRequest;
use crate::provider::{
    runtime::RuntimeProvider,
    service::{
        message::request::{ConnectRequest, RegisterIdRequest},
        service::ServiceProvider,
    },
};
use log::{error, info, warn};
use rand::thread_rng;
use ring::rand::SecureRandom;
use rsa::BigUint;
use rsa::PublicKey;
use rsa::RsaPublicKey;
use std::time::Duration;

pub fn init() -> anyhow::Result<()> {
    let provider = ServiceProvider::new("192.168.0.101:45555")?;

    if let Err(_) = crate::instance::SERVICE_PROVIDER_INSTANCE.set(provider) {
        warn!("service already initialized");
    }

    begin_heart_beat()?;

    Ok(())
}

fn begin_heart_beat() -> anyhow::Result<()> {
    let runtime_provider = crate::instance::RUNTIME_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| anyhow::anyhow!("runtime provider is not initialized"))?;

    crate::instance::SERVICE_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| anyhow::anyhow!("service provider is not initialized"))?;

    runtime_provider.spawn(async move {
        match crate::instance::SERVICE_PROVIDER_INSTANCE
            .get()
            .ok_or_else(|| anyhow::anyhow!("service provider is not initialized"))
        {
            Ok(service_provider) => service_provider.begin_heart_beat().await,
            Err(err) => error!("{}", err),
        }
    });

    Ok(())
}

pub fn device_register_id() -> anyhow::Result<(), APIError> {
    provider_do(|service, runtime| {
        runtime.block_on(async move {
            let res = service
                .device_register_id(
                    RegisterIdRequest {
                        device_id: super::config::read_device_id()?,
                    },
                    Duration::from_secs(10),
                )
                .await;

            if let Ok(resp) = res {
                super::config::save_device_id(&resp.device_id)?;
                super::config::save_device_id_expiration(resp.expire_at)?;
                Ok(())
            } else {
                Err(APIError::ServiceReplyMismatched)
            }
        })
    })
}

pub fn desktop_connect(ask_device_id: String) -> anyhow::Result<(), APIError> {
    provider_do(move |service, runtime| {
        let offer_device_id = match super::config::read_device_id()? {
            Some(device_id) => device_id,
            None => return Err(APIError::ConfigDeviceIdNotFound),
        };

        let ask_device_id = ask_device_id.to_owned();

        runtime.block_on(async move {
            let res = service
                .desktop_connect(
                    ConnectRequest {
                        offer_device_id,
                        ask_device_id: ask_device_id.clone(),
                    },
                    Duration::from_secs(20),
                )
                .await;

            if let Ok(resp) = res {
                let n = BigUint::from_bytes_le(&resp.pub_key_n);
                let e = BigUint::from_bytes_le(&resp.pub_key_e);

                let public_key = RsaPublicKey::new(n, e).map_err(|err| {
                    error!("failed to create public key: {:?}", err);
                    APIError::ServiceReplyInvalid
                })?;

                service.store_verify_password_pub_key(ask_device_id, public_key);

                Ok(())
            } else {
                Err(APIError::ServiceReplyMismatched)
            }
        })
    })
}

pub fn desktop_key_exchange_and_password_verify(
    ask_device_id: String,
    password: String,
) -> anyhow::Result<(), APIError> {
    provider_do(|service, runtime| {
        let offer_device_id = match super::config::read_device_id()? {
            Some(device_id) => device_id,
            None => return Err(APIError::ConfigDeviceIdNotFound),
        };

        let ask_device_id = ask_device_id.to_owned();

        let ask_device_pub_key = match service.remove_verify_password_pub_key(&ask_device_id) {
            Some(pub_key) => pub_key,
            None => return Err(APIError::ServiceReplyInvalid),
        };

        let mut rng = thread_rng();
        let password_secret = ask_device_pub_key
            .encrypt(
                &mut rng,
                rsa::PaddingScheme::PKCS1v15Encrypt,
                &password.as_bytes(),
            )
            .map_err(|err| {
                error!("failed to encrypt password: {:?}", err);
                APIError::ServiceInternal
            })?;

        let ephemeral_rng = ring::rand::SystemRandom::new();
        let local_private_key = ring::agreement::EphemeralPrivateKey::generate(
            &ring::agreement::X25519,
            &ephemeral_rng,
        )
        .map_err(|err| {
            error!("failed to generate ephemeral private key: {:?}", err);
            APIError::ServiceInternal
        })?;

        let local_public_key = local_private_key.compute_public_key().map_err(|err| {
            error!("failed to compute public key: {:?}", err);
            APIError::ServiceInternal
        })?;

        let exchange_pub_key = local_public_key.as_ref().to_vec();

        let mut exchange_salt = Vec::<u8>::with_capacity(32);
        ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
            error!("failed to generate exchange salt: {:?}", err);
            APIError::ServiceInternal
        })?;

        runtime.block_on(async move {
            let resp = match service
                .desktop_key_exchange_and_verify_password(
                    KeyExchangeAndVerifyPasswordRequest {
                        offer_device_id,
                        ask_device_id,
                        password_secret,
                        exchange_pub_key,
                        exchange_salt: exchange_salt.clone(),
                    },
                    Duration::from_secs(20),
                )
                .await
            {
                Ok(resp) => resp,
                Err(err) => {
                    error!("failed to get connection authorization: {:?}", err);
                    return Err(APIError::ServiceReplyMismatched);
                }
            };

            let remote_public_key = ring::agreement::UnparsedPublicKey::new(
                &ring::agreement::X25519,
                &resp.exchange_pub_key,
            );

            let (send_key, recv_key) = ring::agreement::agree_ephemeral(
                local_private_key,
                &remote_public_key,
                ring::error::Unspecified,
                |key_material| {
                    let send_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &exchange_salt)
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

                    let recv_key =
                        ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &resp.exchange_salt)
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
                APIError::ServiceInternal
            })?;

            info!("key exchange and password verify success");
            info!("send key: {:?}", send_key);
            info!("recv key: {:?}", recv_key);

            Ok(())
        })
    })
}

// pub fn desktop_connect_offer(ask_device_id: String) -> anyhow::Result<bool, APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         // ask remote device
//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOfferReq(DesktopConnectOfferReq {
//                     offer_device_id,
//                     ask_device_id: ask_device_id.to_owned(),
//                 }),
//                 Duration::from_secs(15),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         let resp_message = match resp {
//             Message::DesktopConnectOfferResp(message) => message,
//             _ => return Err(APIError::InternalError),
//         };

//         // store remote password auth public key
//         if resp_message.agree {
//             let n = BigUint::from_bytes_le(resp_message.password_auth_public_key_n.as_ref());
//             let e = BigUint::from_bytes_le(resp_message.password_auth_public_key_e.as_ref());
//             let remote_password_auth_public_key = RsaPublicKey::new(n, e).map_err(|err| {
//                 error!("failed to create public key: {:?}", err);
//                 APIError::InternalError
//             })?;

//             let mut remote_password_auth_public_key_map =
//                 REMOTE_PASSWORD_AUTH_PUBLIC_KEY_MAP.lock().unwrap();
//             remote_password_auth_public_key_map
//                 .insert(ask_device_id.to_owned(), remote_password_auth_public_key);
//             drop(remote_password_auth_public_key_map);
//         }

//         Ok(resp_message.agree)
//     })
// }

// pub fn desktop_connect_offer_auth_password(
//     ask_device_id: String,
//     device_password: String,
// ) -> anyhow::Result<bool, APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         let mut remote_password_auth_public_key_map =
//             crate::constant::REMOTE_PASSWORD_AUTH_PUBLIC_KEY_MAP
//                 .lock()
//                 .unwrap();
//         let remote_password_auth_public_key =
//             match remote_password_auth_public_key_map.remove(&ask_device_id) {
//                 Some(key) => key,
//                 None => {
//                     error!("remote_password_auth_public_key is None");
//                     return Err(APIError::InternalError);
//                 }
//             };
//         drop(remote_password_auth_public_key_map);

//         let secret_message = remote_password_auth_public_key
//             .encrypt(
//                 &mut rand::rngs::OsRng,
//                 PaddingScheme::PKCS1v15Encrypt,
//                 &device_password.as_bytes(),
//             )
//             .map_err(|err| {
//                 error!("failed to encrypt device password: {:?}", err);
//                 APIError::InternalError
//             })?;

//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOfferAuthReq(DesktopConnectOfferAuthReq {
//                     offer_device_id,
//                     ask_device_id,
//                     secret_message,
//                 }),
//                 Duration::from_secs(10),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         let resp_message = match resp {
//             Message::DesktopConnectOfferAuthResp(message) => message,
//             _ => return Err(APIError::InternalError),
//         };

//         Ok(resp_message.password_correct)
//     })
// }

// pub fn desktop_connect_open_stream(ask_device_id: String) -> anyhow::Result<(), APIError> {
//     RUNTIME.block_on(async move {
//         let offer_device_id = match super::config::read_device_id()? {
//             Some(id) => id,
//             None => return Err(APIError::ConfigError),
//         };

//         let resp = CLIENT
//             .call(
//                 Message::DesktopConnectOpenStreamReq(DesktopConnectOpenStreamReq {
//                     offer_device_id,
//                     ask_device_id,
//                 }),
//                 Duration::from_secs(10),
//             )
//             .await
//             .map_err(|err| map_message_error(err))?;

//         Ok(())
//     })
// }

// fn map_message_error(message_error: MessageError) -> APIError {
//     match message_error {
//         MessageError::InternalError | MessageError::MismatchedResponseMessage => {
//             APIError::InternalError
//         }
//         MessageError::Timeout => APIError::Timeout,
//         MessageError::InvalidArguments => APIError::InvalidArguments,
//         MessageError::RemoteClientOfflineOrNotExist => APIError::RemoteClientOfflineOrNotExist,
//     }
// }

#[inline]
fn provider_do<T, R>(op: T) -> anyhow::Result<R, APIError>
where
    T: Fn(&ServiceProvider, &RuntimeProvider) -> anyhow::Result<R, APIError>,
{
    let service_provider = crate::instance::SERVICE_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| APIError::ServiceNotInitialized)?;

    let runtime_provider = crate::instance::RUNTIME_PROVIDER_INSTANCE
        .get()
        .ok_or_else(|| APIError::RuntimeNotInitialized)?;

    op(service_provider, runtime_provider)
}
