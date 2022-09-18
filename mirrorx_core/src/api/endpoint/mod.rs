pub mod handlers;
pub mod message;

use self::message::EndPointMessage;
use crate::{
    api::endpoint::handlers::{
        audio_frame::handle_audio_frame,
        error::handle_error,
        input::handle_input,
        negotiate_finished::{
            handle_negotiate_finished_request, handle_negotiate_finished_response,
        },
        negotiate_select_monitor::{
            handle_negotiate_select_monitor_request, handle_negotiate_select_monitor_response,
        },
        negotiate_visit_desktop_params::{
            handle_negotiate_visit_desktop_params_request,
            handle_negotiate_visit_desktop_params_response,
        },
        video_frame::handle_video_frame,
    },
    error::CoreResult,
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use bincode::Options;
use bytes::{Bytes, BytesMut};
use dashmap::DashMap;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use moka::future::Cache;
use once_cell::sync::Lazy;
use ring::aead::{OpeningKey, SealingKey};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

const SEND_MESSAGE_TIMEOUT: Duration = Duration::from_secs(10);
const RECV_MESSAGE_TIMEOUT: Duration = Duration::from_secs(30);

pub static RESERVE_STREAMS: Lazy<DashMap<(i64, i64), Framed<TcpStream, LengthDelimitedCodec>>> =
    Lazy::new(|| DashMap::new());

pub static ENDPOINTS: Lazy<Cache<(i64, i64), tokio::sync::mpsc::Sender<EndPointMessage>>> =
    Lazy::new(|| Cache::builder().initial_capacity(1).build());

// pub async fn start_video_capture(
//     &self,
//     display_id: &str,
//     except_fps: u8,
// ) -> CoreResult<Monitor> {
//     let monitors = crate::component::desktop::monitor::get_active_monitors()?;

//     let monitor = match monitors.iter().find(|m| m.id == display_id) {
//         Some(m) => m,
//         None => match monitors.iter().find(|m| m.is_primary) {
//             Some(m) => m,
//             None => {
//                 return Err(core_error!(
//                     "can not find primary monitor or monitor with id ({})",
//                     display_id
//                 ));
//             }
//         },
//     };

//     let width = monitor.width;
//     let height = monitor.height;
//     let fps = monitor.refresh_rate.min(except_fps);

//     // let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(1);

//     // start_video_encode_process(
//     //     self.remote_device_id.clone(),
//     //     self.exit_tx.clone(),
//     //     self.exit_tx.new_receiver(),
//     //     width as i32,
//     //     height as i32,
//     //     fps as i32,
//     //     capture_frame_rx,
//     //     self.packet_tx.clone(),
//     // )?;

//     // start_desktop_capture_process(
//     //     self.remote_device_id.clone(),
//     //     self.exit_tx.clone(),
//     //     self.exit_tx.new_receiver(),
//     //     capture_frame_tx,
//     //     display_id,
//     //    fps,
//     // )?;

//     let _ = self.monitor.set(monitor.clone());

//     Ok(monitor.clone())
// }

// pub async fn start_video_render(
//     &self,
//     width: i32,
//     height: i32,
//     fps: i32,
//     texture_id: i64,
//     video_texture_ptr: i64,
//     update_frame_callback_ptr: i64,
// ) -> Result<(), CoreError> {
//     let (video_frame_tx, video_frame_rx) = crossbeam::channel::bounded(600);
//     let (decoded_frame_tx, decoded_frame_rx) = crossbeam::channel::bounded(600);

//     start_video_decode_process(
//         self.remote_device_id.clone(),
//         self.exit_tx.clone(),
//         self.exit_tx.new_receiver(),
//         width,
//         height,
//         fps,
//         video_frame_rx,
//         decoded_frame_tx,
//     )?;

//     start_desktop_render_process(
//         self.remote_device_id.clone(),
//         decoded_frame_rx,
//         texture_id,
//         video_texture_ptr,
//         update_frame_callback_ptr,
//     )?;

//     let _ = self.video_frame_tx.set(video_frame_tx);

//     Ok(())
// }

// pub async fn start_audio_capture(&self) -> Result<(), CoreError> {
//     let (pcm_tx, pcm_rx) = crossbeam::channel::bounded(48000 / 960 * 2);

//     start_audio_encode_process(
//         self.remote_device_id.clone(),
//         pcm_rx,
//         self.packet_tx.clone(),
//         48000,
//         2,
//     )?;

//     let exit_tx = start_audio_capture_process(self.remote_device_id.clone(), pcm_tx).await?;

//     Ok(())
// }

// pub async fn start_audio_play(&self) -> Result<(), CoreError> {
//     let (audio_frame_tx, audio_frame_rx) =
//         crossbeam::channel::bounded::<AudioFrame>(48000 / 960 * 2);
//     let (pcm_producer, pcm_consumer) = RingBuffer::new(48000 * 2);

//     start_audio_decode_process(
//         self.remote_device_id.clone(),
//         48000,
//         2,
//         audio_frame_rx,
//         pcm_producer,
//     )?;

//     let exit_tx = start_audio_play_process(self.remote_device_id.clone(), pcm_consumer).await?;

//     let _ = self.audio_frame_tx.set(audio_frame_tx);

//     Ok(())
// }

pub fn serve_reader(
    local_device_id: i64,
    remote_device_id: i64,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut stream: SplitStream<Framed<TcpStream, LengthDelimitedCodec>>,
    mut opening_key: OpeningKey<NonceValue>,
    message_tx: tokio::sync::mpsc::Sender<EndPointMessage>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            if let Err(async_broadcast::TryRecvError::Empty) = exit_rx.try_recv() {
                tracing::info!("read processor receive exit tx send signal");
                break;
            }

            match stream.next().await {
                Some(res) => {
                    let mut packet_bytes = match res {
                        Ok(packet_bytes) => packet_bytes,
                        Err(err) => {
                            tracing::error!(
                                ?local_device_id,
                                ?remote_device_id,
                                ?err,
                                "read from stream failed"
                            );
                            break;
                        }
                    };

                    if let Err(err) = open_packet(
                        local_device_id,
                        remote_device_id,
                        &mut opening_key,
                        &mut packet_bytes,
                        message_tx.clone(),
                    ) {
                        tracing::error!(
                            ?local_device_id,
                            ?remote_device_id,
                            ?err,
                            "open packet failed"
                        );
                    }
                }
                None => {
                    tracing::info!(?local_device_id, ?remote_device_id, "stream closed");
                    break;
                }
            }
        }

        let _ = exit_tx.broadcast(()).await;

        ENDPOINTS
            .invalidate(&(local_device_id.to_owned(), remote_device_id.to_owned()))
            .await;

        tracing::info!(?local_device_id, ?remote_device_id, "read process exit");
    });
}

pub fn serve_writer(
    local_device_id: i64,
    remote_device_id: i64,
    exit_tx: async_broadcast::Sender<()>,
    mut exit_rx: async_broadcast::Receiver<()>,
    mut sink: SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>,
    mut sealing_key: SealingKey<NonceValue>,
    mut message_rx: tokio::sync::mpsc::Receiver<EndPointMessage>,
) {
    TOKIO_RUNTIME.spawn(async move {
        loop {
            if let Err(async_broadcast::TryRecvError::Empty) = exit_rx.try_recv() {
                tracing::info!("write processor receive exit tx send signal");
                break;
            }

            match message_rx.recv().await {
                Some(message) => {
                    let buffer = match seal_packet(&mut sealing_key, &message) {
                        Ok(buffer) => buffer,
                        Err(err) => {
                            tracing::error!(
                                ?local_device_id,
                                ?remote_device_id,
                                ?err,
                                "seal packet failed"
                            );
                            continue;
                        }
                    };

                    if let Err(err) = sink.send(buffer).await {
                        tracing::error!(
                            ?local_device_id,
                            ?remote_device_id,
                            ?err,
                            "write to stream failed"
                        );
                        break;
                    }
                }
                None => {
                    tracing::info!(?local_device_id, ?remote_device_id, "writer tx closed");
                    break;
                }
            }
        }

        let _ = exit_tx.broadcast(()).await;

        ENDPOINTS
            .invalidate(&(local_device_id.to_owned(), remote_device_id.to_owned()))
            .await;

        tracing::info!(?local_device_id, ?remote_device_id, "write process exit");
    });
}

fn open_packet(
    active_device_id: i64,
    passive_device_id: i64,
    opening_key: &mut OpeningKey<NonceValue>,
    buffer: &mut BytesMut,
    message_tx: tokio::sync::mpsc::Sender<EndPointMessage>,
) -> CoreResult<()> {
    let opened_buffer = opening_key.open_in_place(ring::aead::Aad::empty(), buffer)?;
    let message = BINCODE_SERIALIZER.deserialize::<EndPointMessage>(&opened_buffer)?;

    TOKIO_RUNTIME.spawn(async move {
        handle_message(active_device_id, passive_device_id, message, message_tx).await;
    });

    Ok(())
}

fn seal_packet(
    sealing_key: &mut SealingKey<NonceValue>,
    message: &EndPointMessage,
) -> CoreResult<Bytes> {
    let mut packet_buffer = BINCODE_SERIALIZER.serialize(message)?;
    sealing_key.seal_in_place_append_tag(ring::aead::Aad::empty(), &mut packet_buffer)?;
    Ok(Bytes::from(packet_buffer))
}

async fn handle_message(
    active_device_id: i64,
    passive_device_id: i64,
    message: EndPointMessage,
    message_tx: tokio::sync::mpsc::Sender<EndPointMessage>,
) {
    macro_rules! match_and_handle_message {
        ($message:expr, $(error $err_message_type:path => $err_handler:ident,)? $(reply $req_message_type:path => $req_handler:ident,)* $(noreply $other_message_type:path => $other_handler:ident,)*) => {
            match $message{
                $($err_message_type => $err_handler(active_device_id, passive_device_id).await,)?
                $($req_message_type(req) => $req_handler(active_device_id, passive_device_id, req, message_tx).await,)+
                $($other_message_type(req) => $other_handler(active_device_id, passive_device_id, req).await,)+
            }
        };
    }

    match_and_handle_message!(message,
        error EndPointMessage::Error => handle_error,
        reply EndPointMessage::NegotiateVisitDesktopParamsRequest => handle_negotiate_visit_desktop_params_request,
        reply EndPointMessage::NegotiateSelectMonitorRequest => handle_negotiate_select_monitor_request,
        reply EndPointMessage::NegotiateFinishedRequest => handle_negotiate_finished_request,
        noreply EndPointMessage::NegotiateVisitDesktopParamsResponse => handle_negotiate_visit_desktop_params_response,
        noreply EndPointMessage::NegotiateSelectMonitorResponse => handle_negotiate_select_monitor_response,
        noreply EndPointMessage::NegotiateFinishedResponse => handle_negotiate_finished_response,
        noreply EndPointMessage::VideoFrame => handle_video_frame,
        noreply EndPointMessage::AudioFrame => handle_audio_frame,
        noreply EndPointMessage::Input => handle_input,
    );
}
