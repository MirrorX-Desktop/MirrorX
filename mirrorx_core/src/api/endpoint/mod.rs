pub mod flutter_message;
pub mod handlers;
pub mod message;

use self::message::EndPointMessage;
use crate::{
    api::endpoint::handlers::{
        audio_frame::handle_audio_frame,
        error::handle_error,
        input::handle_input,
        negotiate_finished::handle_negotiate_finished_request,
        negotiate_select_monitor::{
            handle_negotiate_select_monitor_request, handle_negotiate_select_monitor_response,
        },
        negotiate_visit_desktop_params::{
            handle_negotiate_visit_desktop_params_request,
            handle_negotiate_visit_desktop_params_response,
        },
        video_frame::handle_video_frame,
    },
    component::desktop::monitor::Monitor,
    error::CoreResult,
    utility::{nonce_value::NonceValue, runtime::TOKIO_RUNTIME, serializer::BINCODE_SERIALIZER},
};
use async_broadcast::TryRecvError;
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
    Lazy::new(DashMap::new);

pub static ENDPOINTS: Lazy<Cache<(i64, i64), tokio::sync::mpsc::Sender<EndPointMessage>>> =
    Lazy::new(|| Cache::builder().initial_capacity(1).build());

pub static ENDPOINTS_MONITOR: Lazy<Cache<(i64, i64), Monitor>> =
    Lazy::new(|| Cache::builder().initial_capacity(1).build());

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
            if exit_rx
                .try_recv()
                .map_err(|err| {
                    if err == TryRecvError::Empty {
                        Err(())
                    } else {
                        Ok(())
                    }
                })
                .is_ok()
            {
                tracing::info!(
                    ?local_device_id,
                    ?remote_device_id,
                    "read processor receive exit tx signal"
                );
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
            .invalidate(&(local_device_id, remote_device_id))
            .await;

        ENDPOINTS_MONITOR
            .invalidate(&(local_device_id, remote_device_id))
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
            if exit_rx
                .try_recv()
                .map_err(|err| {
                    if err == TryRecvError::Empty {
                        Err(())
                    } else {
                        Ok(())
                    }
                })
                .is_ok()
            {
                tracing::info!(
                    ?local_device_id,
                    ?remote_device_id,
                    "write processor receive exit tx signal"
                );
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
            .invalidate(&(local_device_id, remote_device_id))
            .await;

        ENDPOINTS_MONITOR
            .invalidate(&(local_device_id, remote_device_id))
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
    let message = BINCODE_SERIALIZER.deserialize::<EndPointMessage>(opened_buffer)?;

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
    match message {
        EndPointMessage::Error => handle_error(active_device_id, passive_device_id).await,
        EndPointMessage::NegotiateVisitDesktopParamsRequest(req) => {
            handle_negotiate_visit_desktop_params_request(
                active_device_id,
                passive_device_id,
                req,
                message_tx,
            )
            .await
        }
        EndPointMessage::NegotiateVisitDesktopParamsResponse(resp) => {
            handle_negotiate_visit_desktop_params_response(
                active_device_id,
                passive_device_id,
                resp,
            )
            .await
        }
        EndPointMessage::NegotiateSelectMonitorRequest(req) => {
            handle_negotiate_select_monitor_request(
                active_device_id,
                passive_device_id,
                req,
                message_tx,
            )
            .await
        }
        EndPointMessage::NegotiateSelectMonitorResponse(resp) => {
            handle_negotiate_select_monitor_response(active_device_id, passive_device_id, resp)
                .await
        }
        EndPointMessage::NegotiateFinishedRequest(req) => {
            handle_negotiate_finished_request(active_device_id, passive_device_id, req, message_tx)
                .await
        }
        EndPointMessage::VideoFrame(video_frame) => {
            handle_video_frame(active_device_id, passive_device_id, video_frame).await
        }
        EndPointMessage::AudioFrame(audio_frame) => {
            handle_audio_frame(active_device_id, passive_device_id, audio_frame).await
        }
        EndPointMessage::Input(input_event) => {
            handle_input(active_device_id, passive_device_id, input_event).await
        }
    }
}
