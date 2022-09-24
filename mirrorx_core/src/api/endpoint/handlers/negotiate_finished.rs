use super::video_frame::serve_decoder;
use crate::{
    api::endpoint::{
        flutter_message::FlutterMediaMessage,
        message::{EndPointMessage, EndPointNegotiateFinishedRequest},
        ENDPOINTS, ENDPOINTS_MONITOR, SEND_MESSAGE_TIMEOUT,
    },
    component::{
        desktop::{monitor::get_active_monitors, Duplicator},
        video_encoder::{config::EncoderType, video_encoder::VideoEncoder},
    },
    core_error,
    error::{CoreError, CoreResult},
    utility::runtime::TOKIO_RUNTIME,
};
use flutter_rust_bridge::StreamSink;
use scopeguard::defer;
use tokio::sync::mpsc::Sender;

pub struct NegotiateFinishedRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub expect_frame_rate: u8,
    pub texture_id: i64,
    // pub video_texture_pointer: i64,
    // pub update_frame_callback_pointer: i64,
}

pub async fn negotiate_finished(
    req: NegotiateFinishedRequest,
    stream: StreamSink<FlutterMediaMessage>,
) -> CoreResult<()> {
    let message_tx = ENDPOINTS
        .get(&(req.active_device_id, req.passive_device_id))
        .ok_or(core_error!("endpoint not exists"))?;

    let negotiate_req =
        EndPointMessage::NegotiateFinishedRequest(EndPointNegotiateFinishedRequest {
            expected_frame_rate: req.expect_frame_rate,
        });

    if let Err(err) = message_tx
        .send_timeout(negotiate_req, SEND_MESSAGE_TIMEOUT)
        .await
    {
        return Err(core_error!(
            "negotiate_finished: message send failed ({})",
            err
        ));
    }

    serve_decoder(
        req.active_device_id,
        req.passive_device_id,
        req.texture_id,
        stream,
    );

    Ok(())
}

pub async fn handle_negotiate_finished_request(
    active_device_id: i64,
    passive_device_id: i64,
    _: EndPointNegotiateFinishedRequest,
    message_tx: Sender<EndPointMessage>,
) {
    spawn_desktop_capture_and_encode_process(
        active_device_id,
        passive_device_id,
        message_tx.clone(),
    );

    spawn_audio_capture_and_encode_process(active_device_id, passive_device_id, message_tx);
}

#[cfg(target_os = "macos")]
fn spawn_desktop_capture_and_encode_process(
    active_device_id: i64,
    passive_device_id: i64,
    message_tx: Sender<EndPointMessage>,
) {
    let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(180);

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?active_device_id, ?passive_device_id, "desktop capture process exit");
        }

        let monitors = match get_active_monitors(false) {
            Ok(params) => params,
            Err(err) => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "get_primary_monitor_params failed"
                );
                return;
            }
        };

        let mut encoder = match VideoEncoder::new(EncoderType::H264VideoToolbox, message_tx) {
            Ok(encoder) => encoder,
            Err(err) => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "initialize encoder failed"
                );
                return;
            }
        };

        let primary_monitor = monitors.iter().find(|monitor| monitor.is_primary);

        let (duplicator, monitor_id) = match Duplicator::new(
            primary_monitor.map(|monitor| monitor.id.to_owned()),
            capture_frame_tx,
        ) {
            Ok(duplicator) => duplicator,
            Err(err) => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "initialize encoder failed"
                );
                return;
            }
        };

        let select_monitor = match monitors
            .into_iter()
            .find(|monitor| monitor.id == monitor_id)
        {
            Some(monitor) => monitor,
            None => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    "can't find selected monitor"
                );
                return;
            }
        };

        ENDPOINTS_MONITOR
            .blocking()
            .insert((active_device_id, passive_device_id), select_monitor);

        if let Err(err) = duplicator.start() {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                ?err,
                "desktop capture process start failed"
            );
            return;
        }

        defer! {
            let _ = duplicator.stop();
        }

        loop {
            match capture_frame_rx.recv() {
                Ok(capture_frame) => {
                    if let Err(err) = encoder.encode(capture_frame) {
                        tracing::error!(
                            ?active_device_id,
                            ?passive_device_id,
                            ?err,
                            "video encode failed"
                        );
                        break;
                    }
                }
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "capture frame rx recv error"
                    );
                    break;
                }
            }
        }
    });
}

#[cfg(target_os = "windows")]
fn spawn_desktop_capture_and_encode_process(
    active_device_id: i64,
    passive_device_id: i64,
    message_tx: Sender<EndPointMessage>,
) {
    let monitors = match get_active_monitors(false) {
        Ok(params) => params,
        Err(err) => {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                ?err,
                "get_active_monitors failed"
            );
            return;
        }
    };

    let (capture_frame_tx, capture_frame_rx) = crossbeam::channel::bounded(180);

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?active_device_id, ?passive_device_id, "desktop capture process exit");
        }

        let primary_monitor = monitors.iter().find(|monitor| monitor.is_primary);

        let (mut duplicator, monitor_id) =
            match Duplicator::new(primary_monitor.map(|monitor| monitor.id.to_owned())) {
                Ok(duplicator) => duplicator,
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "initialize encoder failed"
                    );
                    return;
                }
            };

        let select_monitor = match monitors
            .into_iter()
            .find(|monitor| monitor.id == monitor_id)
        {
            Some(monitor) => monitor,
            None => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    "can't find selected monitor"
                );
                return;
            }
        };

        ENDPOINTS_MONITOR
            .blocking()
            .insert((active_device_id, passive_device_id), select_monitor);

        loop {
            match duplicator.capture() {
                Ok(capture_frame) => {
                    if let Err(err) = capture_frame_tx.try_send(capture_frame) {
                        if err.is_full() {
                            tracing::warn!("capture frame tx is full!");
                        } else {
                            tracing::info!("capture frame tx closed, capture process will exit");
                            break;
                        }
                    }
                }
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "dekstop duplicator capture loop exit"
                    );
                    break;
                }
            };
        }
    });

    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?active_device_id, ?passive_device_id, "video encode process exit");
        }

        let mut encoder = match VideoEncoder::new(EncoderType::Libx264, message_tx) {
            Ok(encoder) => encoder,
            Err(err) => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "video encoder initialize failed"
                );
                return;
            }
        };

        loop {
            match capture_frame_rx.recv() {
                Ok(capture_frame) => {
                    if let Err(err) = encoder.encode(capture_frame) {
                        tracing::error!(
                            ?active_device_id,
                            ?passive_device_id,
                            ?err,
                            "video encode failed"
                        );
                        break;
                    }
                }
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "capture frame rx recv error"
                    );
                    break;
                }
            }
        }
    });
}

#[cfg(target_os = "windows")]
fn spawn_audio_capture_and_encode_process(
    active_device_id: i64,
    passive_device_id: i64,
    message_tx: Sender<EndPointMessage>,
) {
    TOKIO_RUNTIME.spawn_blocking(move || {
        defer! {
            tracing::info!(?active_device_id, ?passive_device_id, "audio capture process exit");
        }

        let (audio_frame_tx, audio_frame_rx) = crossbeam::channel::bounded(12000);

        let duplicator = match crate::component::audio::duplicator::Duplicator::new(None) {
            Ok(duplicator) => duplicator,
            Err(err) => {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "audio duplicator initialize failed"
                );
                return;
            }
        };

        let sample_rate = duplicator.sample_rate();
        let channels = duplicator.channels();

        TOKIO_RUNTIME.spawn_blocking(move || {
            defer! {
                tracing::info!(?active_device_id, ?passive_device_id, "audio encode process exit");
            }

            let mut audio_encoder = match crate::component::audio::encoder::AudioEncoder::new(
                sample_rate,
                channels,
                message_tx,
            ) {
                Ok(audio_encoder) => audio_encoder,
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "audio encoder initialize failed"
                    );
                    return;
                }
            };

            loop {
                match audio_frame_rx.recv() {
                    Ok(audio_encode_frame) => {
                        if let Err(err) = audio_encoder.encode(audio_encode_frame) {
                            tracing::error!(
                                ?active_device_id,
                                ?passive_device_id,
                                ?err,
                                "audio encode failed"
                            );
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::error!(
                            ?active_device_id,
                            ?passive_device_id,
                            ?err,
                            "audio frame rx recv error"
                        );
                        break;
                    }
                }
            }
        });

        if let Err(err) = duplicator.start() {
            tracing::error!(
                ?active_device_id,
                ?passive_device_id,
                ?err,
                "audio duplicator start capture failed"
            );
            return;
        }

        defer! {
            if let Err(err) = duplicator.stop() {
                tracing::error!(
                    ?active_device_id,
                    ?passive_device_id,
                    ?err,
                    "audio duplicator capture stop failed"
                );
            }
        }

        let should_sleep_duration = duplicator.should_sleep_duration();
        loop {
            std::thread::sleep(should_sleep_duration);

            match duplicator.capture() {
                Ok(frames) => {
                    for frame in frames {
                        if let Err(err) = audio_frame_tx.try_send(frame) {
                            if err.is_full() {
                                tracing::warn!("audio encode tx is full!");
                            } else {
                                tracing::info!("audio encode tx closed, capture process will exit");
                                break;
                            }
                        }
                    }
                }
                Err(err) => {
                    tracing::error!(
                        ?active_device_id,
                        ?passive_device_id,
                        ?err,
                        "audio duplicator capture loop exit"
                    );
                    break;
                }
            }
        }
    });
}
