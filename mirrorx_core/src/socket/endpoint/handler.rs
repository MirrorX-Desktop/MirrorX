// use super::{
//     endpoint::client_to_client::{
//         ConnectReply, ConnectRequest, KeyExchangeAndVerifyPasswordReply,
//         KeyExchangeAndVerifyPasswordRequest, MediaTransmission, StartMediaTransmissionReply,
//         StartMediaTransmissionRequest,
//     },
//     endpoint::EndPoint,
// };
// use crate::{
//     media::{desktop_duplicator::DesktopDuplicator, video_encoder::VideoEncoder},
//     provider::{config::ConfigProvider, runtime::RuntimeProvider, signal_a::SocketProvider},
//     socket::endpoint::CacheKey,
// };
// use anyhow::anyhow;
// use ring::rand::SecureRandom;
// use rsa::{PaddingScheme, PublicKeyParts, RsaPrivateKey, RsaPublicKey};
// use std::{sync::Arc, time::Duration};
// use tracing::{error, info};

// pub async fn handle_start_media_transmission(
//     endpoint: Arc<EndPoint>,
//     req: StartMediaTransmissionRequest,
// ) -> anyhow::Result<StartMediaTransmissionReply> {
//     tracing::trace!(req = %req, "start_media_transmission");

//     let encoder_name: &str;

//     if cfg!(target_os = "macos") {
//         encoder_name = "h264_videotoolbox";
//     } else if cfg!(target_os = "windows") {
//         encoder_name = "libx264";
//     } else {
//         panic!("unsupported platform");
//     }

//     let mut encoder = VideoEncoder::new(encoder_name, 60, 1920, 1080)?;

//     encoder.set_opt("profile", "high", 0)?;
//     encoder.set_opt("level", "5.2", 0)?;

//     if encoder_name == "libx264" {
//         encoder.set_opt("preset", "ultrafast", 0)?;
//         encoder.set_opt("tune", "zerolatency", 0)?;
//         encoder.set_opt("sc_threshold", "499", 0)?;
//     } else {
//         encoder.set_opt("realtime", "1", 0)?;
//         encoder.set_opt("allow_sw", "0", 0)?;
//     }

//     let packet_rx = encoder.open()?;
//     let (mut desktop_duplicator, capture_frame_rx) = DesktopDuplicator::new(60)?;

//     std::thread::spawn(move || {
//         // make sure the media_transmission after start_media_transmission send
//         std::thread::sleep(Duration::from_secs(1));

//         if let Err(err) = desktop_duplicator.start() {
//             error!(?err, "DesktopDuplicator start capture failed");
//             return;
//         }

//         loop {
//             let capture_frame = match capture_frame_rx.recv() {
//                 Ok(frame) => frame,
//                 Err(err) => {
//                     tracing::error!(?err, "capture_frame_rx.recv");
//                     break;
//                 }
//             };

//             // encode will block current thread until capture_frame released (FFMpeg API 'avcodec_send_frame' finished)
//             encoder.encode(capture_frame);
//         }
//         desktop_duplicator.stop();
//     });

//     std::thread::spawn(move || {
//         let runtime_provider = match RuntimeProvider::current() {
//             Ok(provider) => provider,
//             Err(err) => {
//                 error!(?err, "handle_start_media_transmission");
//                 return;
//             }
//         };

//         let socket_provider = match SocketProvider::current() {
//             Ok(provider) => provider,
//             Err(err) => {
//                 error!(?err, "handle_start_media_transmission");
//                 return;
//             }
//         };

//         loop {
//             match packet_rx.recv() {
//                 Ok(packet) => {
//                     if let Err(err) =
//                         runtime_provider.block_on(socket_provider.desktop_media_transmission(
//                             endpoint.clone(),
//                             MediaTransmission {
//                                 data: packet.data,
//                                 timestamp: 0,
//                             },
//                         ))
//                     {
//                         error!(?err, "desktop_media_transmission failed");
//                     }
//                 }
//                 Err(err) => {
//                     error!(err=?err, "packet_rx.recv");
//                     break;
//                 }
//             };
//         }
//     });

//     let reply = StartMediaTransmissionReply {
//         os_name: crate::constants::OS_NAME
//             .get()
//             .map(|v| v.clone())
//             .unwrap_or(String::from("Unknown")),
//         os_version: crate::constants::OS_VERSION
//             .get()
//             .map(|v| v.clone())
//             .unwrap_or(String::from("Unknown")),
//         video_type: String::from("todo"),
//         audio_type: String::from("todo"),
//     };

//     Ok(reply)
// }

// pub async fn handle_media_transmission(
//     endpoint: Arc<EndPoint>,
//     media_transmission: MediaTransmission,
// ) {
//     info!(
//         "receive media transmission, length: {}",
//         media_transmission.data.len()
//     );
//     endpoint.transfer_desktop_video_frame(media_transmission.data);
// }

// fn select_endpoint(
//     local_device_id: String,
//     remote_device_id: String,
// ) -> anyhow::Result<Arc<EndPoint>> {
//     if !EndPointProvider::current()?.contains(&remote_device_id) {
//         let ep = Arc::new(EndPoint::new(local_device_id, remote_device_id.to_owned()));
//         EndPointProvider::current()?.insert(remote_device_id.to_owned(), ep);
//     }

//     EndPointProvider::current()?
//         .get(&remote_device_id)
//         .ok_or_else(|| anyhow::anyhow!("select_endpoint: endpoint not found"))
// }

// fn handle_client_to_client_message(
//     endpoint: Arc<EndPoint>,
//     call_id: u16,
//     message: ClientToClientMessage,
// ) -> anyhow::Result<()> {
//     match message {
//         ClientToClientMessage::ConnectRequest(req) => {
//             handle_client_to_client_call_message!(
//                 endpoint,
//                 call_id,
//                 client_to_client_handler::handle_connect,
//                 req,
//                 ClientToClientMessage::ConnectReply
//             );
//         }
//         ClientToClientMessage::KeyExchangeAndVerifyPasswordRequest(req) => {
//             handle_client_to_client_call_message!(
//                 endpoint,
//                 call_id,
//                 client_to_client_handler::handle_key_exchange_and_verify_password,
//                 req,
//                 ClientToClientMessage::KeyExchangeAndVerifyPasswordReply
//             );
//         }
//         ClientToClientMessage::StartMediaTransmissionRequest(req) => {
//             handle_client_to_client_call_message!(
//                 endpoint,
//                 call_id,
//                 client_to_client_handler::handle_start_media_transmission,
//                 req,
//                 ClientToClientMessage::StartMediaTransmissionReply
//             );
//         }
//         ClientToClientMessage::MediaTransmission(media_transmission) => {
//             RuntimeProvider::current()?.spawn(async move {
//                 client_to_client_handler::handle_media_transmission(endpoint, media_transmission)
//                     .await;
//             });
//         }
//         _ => {
//             SocketProvider::current()?.set_client_call_reply(call_id, message);
//         }
//     };

//     Ok(())
// }
