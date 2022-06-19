use super::{
    endpoint::EndPoint,
    endpoint::{
        client_to_client::{
            ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
            KeyExchangeAndVerifyPasswordRequest, MediaTransmission, StartMediaTransmissionReply,
            StartMediaTransmissionRequest,
        },
        ENDPOINTS,
    },
};
use crate::error::MirrorXError;
use crate::{
    error::anyhow::Result,
    media::{desktop_duplicator::DesktopDuplicator, video_encoder::VideoEncoder},
    provider::{config::ConfigProvider, runtime::RuntimeProvider, signal_a::SocketProvider},
    socket::endpoint::message::StartMediaTransmissionRequest,
    socket::endpoint::CacheKey,
};
use anyhow::anyhow;
use ring::rand::SecureRandom;
use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use std::{sync::Arc, time::Duration};
use tracing::{error, info};

pub async fn handle_start_media_transmission_request(
    endpoint: &EndPoint,
    req: StartMediaTransmissionRequest,
) -> Result<StartMediaTransmissionReply, MirrorXError> {
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
    remove_device_id: String,
    media_transmission: MediaTransmission,
) -> anyhow::Result<()> {
    // info!(
    //     "receive media transmission, length: {}",
    //     media_transmission.data.len()
    // );
    // endpoint.transfer_desktop_video_frame(media_transmission.data);
    Ok(())
}
