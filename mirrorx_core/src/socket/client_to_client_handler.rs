use super::{
    endpoint::EndPoint,
    message::client_to_client::{
        ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
        KeyExchangeAndVerifyPasswordRequest, MediaTransmission, StartMediaTransmissionReply,
        StartMediaTransmissionRequest,
    },
};
use crate::{
    media::{desktop_duplicator::DesktopDuplicator, video_encoder::VideoEncoder},
    provider::{config::ConfigProvider, runtime::RuntimeProvider, socket::SocketProvider},
    socket::endpoint::CacheKey,
};
use anyhow::anyhow;
use ring::rand::SecureRandom;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::{sync::Arc, time::Duration};
use tracing::{error, info};

pub async fn handle_connect(
    endpoint: Arc<EndPoint>,
    req: ConnectRequest,
) -> anyhow::Result<ConnectReply> {
    tracing::trace!(req = %req, "connect");

    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 4096)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_n = pub_key.n().to_bytes_le();
    let pub_key_e = pub_key.e().to_bytes_le();

    endpoint
        .cache()
        .set(CacheKey::PasswordVerifyPrivateKey, priv_key);

    Ok(ConnectReply {
        pub_key_n,
        pub_key_e,
    })
}

pub async fn handle_key_exchange_and_verify_password(
    endpoint: Arc<EndPoint>,
    req: KeyExchangeAndVerifyPasswordRequest,
) -> anyhow::Result<KeyExchangeAndVerifyPasswordReply> {
    tracing::trace!(req = %req, "key_exchange_and_verify_password");

    // todo: check white list

    let local_password = ConfigProvider::current()?
        .read_device_password()?
        .ok_or(anyhow!(
            "key_exchange_and_verify_password: local password not set, refuse request"
        ))?;

    let priv_key = endpoint
        .cache()
        .take::<RsaPrivateKey>(CacheKey::PasswordVerifyPrivateKey)
        .ok_or(anyhow::anyhow!(
            "key_exchange_and_verify_password: no private key found"
        ))?;

    let req_password = priv_key
        .decrypt(PaddingScheme::PKCS1v15Encrypt, &req.password_secret)
        .map_err(|err| {
            anyhow!(
                "key_exchange_and_verify_password: decrypt password secret failed: {}",
                err
            )
        })?;

    let req_password = String::from_utf8(req_password).map_err(|err| {
        anyhow!(
            "key_exchange_and_verify_password: parse local password bytes to utf8 failed: {}",
            err
        )
    })?;

    if req_password != local_password {
        return Ok(KeyExchangeAndVerifyPasswordReply {
            password_correct: false,
            exchange_pub_key: Vec::default(),
            exchange_salt: Vec::default(),
        });
    }

    // gen key exchange
    let ephemeral_rng = ring::rand::SystemRandom::new();
    let local_private_key =
        ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519, &ephemeral_rng)
            .map_err(|err| {
                anyhow!(
                    "key_exchange_and_verify_password: generate ephemeral private key failed: {}",
                    err
                )
            })?;

    let local_public_key = local_private_key.compute_public_key().map_err(|err| {
        anyhow::anyhow!(
            "key_exchange_and_verify_password: compute public key failed: {}",
            err
        )
    })?;

    let exchange_pub_key = local_public_key.as_ref().to_vec();

    let mut exchange_salt = Vec::<u8>::new();
    exchange_salt.resize(32, 0);
    ephemeral_rng.fill(&mut exchange_salt).map_err(|err| {
        anyhow::anyhow!(
            "key_exchange_and_verify_password: generate exchange salt failed: {}",
            err
        )
    })?;

    let remote_public_key =
        ring::agreement::UnparsedPublicKey::new(&ring::agreement::X25519, &req.exchange_pub_key);

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

            let recv_key = ring::hkdf::Salt::new(ring::hkdf::HKDF_SHA512, &req.exchange_salt)
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
            "key_exchange_and_verify_password: agree ephemeral key failed: {:?}",
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
        unsafe { u64::from_le_bytes(*(req.exchange_salt[..8].as_ptr() as *const [u8; 8])) };

    endpoint
        .set_sealing_key(unbound_sealing_key, sealing_initial_nonce)
        .await;

    tracing::trace!("key_exchange_and_verify_password success");

    Ok(KeyExchangeAndVerifyPasswordReply {
        password_correct: true,
        exchange_pub_key,
        exchange_salt,
    })
}

pub async fn handle_start_media_transmission(
    endpoint: Arc<EndPoint>,
    req: StartMediaTransmissionRequest,
) -> anyhow::Result<StartMediaTransmissionReply> {
    tracing::trace!(req = %req, "start_media_transmission");

    let encoder_name: &str;

    if cfg!(target_os = "macos") {
        encoder_name = "h264_videotoolbox";
    } else if cfg!(target_os = "windows") {
        encoder_name = "libx264";
    } else {
        panic!("unsupported platform");
    }

    let mut encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

    encoder.set_opt("profile", "high", 0)?;
    encoder.set_opt("level", "5.2", 0)?;

    if encoder_name == "libx264" {
        encoder.set_opt("preset", "ultrafast", 0)?;
        encoder.set_opt("tune", "zerolatency", 0)?;
        encoder.set_opt("sc_threshold", "499", 0)?;
    } else {
        encoder.set_opt("realtime", "1", 0)?;
        encoder.set_opt("allow_sw", "0", 0)?;
    }

    let packet_rx = encoder.open()?;
    let (mut desktop_duplicator, capture_frame_rx) = DesktopDuplicator::new(60)?;

    std::thread::spawn(move || {
        // make sure the media_transmission after start_media_transmission send
        std::thread::sleep(Duration::from_secs(1));

        if let Err(err) = desktop_duplicator.start() {
            error!(?err, "DesktopDuplicator start capture failed");
            return;
        }

        loop {
            let capture_frame = match capture_frame_rx.recv() {
                Ok(frame) => frame,
                Err(err) => {
                    tracing::error!(?err, "capture_frame_rx.recv");
                    break;
                }
            };

            // encode will block current thread until capture_frame released (FFMpeg API 'avcodec_send_frame' finished)
            encoder.encode(capture_frame);
        }
        desktop_duplicator.stop();
    });

    std::thread::spawn(move || {
        let runtime_provider = match RuntimeProvider::current() {
            Ok(provider) => provider,
            Err(err) => {
                error!(?err, "handle_start_media_transmission");
                return;
            }
        };

        let socket_provider = match SocketProvider::current() {
            Ok(provider) => provider,
            Err(err) => {
                error!(?err, "handle_start_media_transmission");
                return;
            }
        };

        loop {
            match packet_rx.recv() {
                Ok(packet) => {
                    if let Err(err) =
                        runtime_provider.block_on(socket_provider.desktop_media_transmission(
                            endpoint.clone(),
                            MediaTransmission {
                                data: packet.data,
                                timestamp: 0,
                            },
                        ))
                    {
                        error!(?err, "desktop_media_transmission failed");
                    }
                }
                Err(err) => {
                    error!(err=?err, "packet_rx.recv");
                    break;
                }
            };
        }
    });

    let reply = StartMediaTransmissionReply {
        os_name: crate::constants::OS_NAME
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
        os_version: crate::constants::OS_VERSION
            .get()
            .map(|v| v.clone())
            .unwrap_or(String::from("Unknown")),
        video_type: String::from("todo"),
        audio_type: String::from("todo"),
    };

    Ok(reply)
}

pub async fn handle_media_transmission(
    endpoint: Arc<EndPoint>,
    media_transmission: MediaTransmission,
) {
    info!(
        "receive media transmission, length: {}",
        media_transmission.data.len()
    );
    endpoint.transfer_desktop_video_frame(media_transmission.data);
}
