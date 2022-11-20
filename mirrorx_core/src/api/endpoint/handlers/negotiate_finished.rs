use crate::api::endpoint::PASSIVE_ENDPOINTS_MONITORS;
use crate::component::audio::duplicator::new_record_stream_and_rx;
use crate::component::audio_encoder::audio_encoder::AudioEncoder;
use crate::error::CoreError;
use crate::{
    api::endpoint::EndPointClient,
    component::{
        desktop::{monitor::get_active_monitors, Duplicator},
        video_encoder::video_encoder::VideoEncoder,
    },
};
use cpal::traits::StreamTrait;
use scopeguard::defer;

pub struct NegotiateFinishedRequest {
    pub active_device_id: i64,
    pub passive_device_id: i64,
    pub expect_frame_rate: u8,
    pub texture_id: i64,
}

pub fn handle_negotiate_finished_request(client: EndPointClient) {
    spawn_desktop_capture_and_encode_process(client.clone());
    spawn_audio_capture_and_encode_process(client);
}

#[cfg(target_os = "macos")]
fn spawn_desktop_capture_and_encode_process(client: EndPointClient) {
    use crate::component::video_encoder::config::libx264::Libx264Config;

    let (capture_frame_tx, mut capture_frame_rx) = tokio::sync::mpsc::channel(180);

    tokio::task::spawn_blocking(move || {
        tracing::info_span!("desktop_capture_and_encode_process", id = ?client.id());

        defer! {
            tracing::info!("desktop capture process exit");
        }

        let monitors = match get_active_monitors(false) {
            Ok(params) => params,
            Err(err) => {
                tracing::error!(?err, "get_primary_monitor_params failed");
                return;
            }
        };

        let mut encoder = match VideoEncoder::new(Libx264Config::new(), client.clone()) {
            Ok(encoder) => encoder,
            Err(err) => {
                tracing::error!(?err, "initialize encoder failed");
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
                tracing::error!(?err, "initialize encoder failed");
                return;
            }
        };

        let select_monitor = match monitors
            .into_iter()
            .find(|monitor| monitor.id == monitor_id)
        {
            Some(monitor) => monitor,
            None => {
                tracing::error!("can't find selected monitor");
                return;
            }
        };

        tracing::info!(?select_monitor.width,?select_monitor.height,"select monitor");

        PASSIVE_ENDPOINTS_MONITORS.insert(client.id, select_monitor);

        if let Err(err) = duplicator.start() {
            tracing::error!(?err, "desktop capture process start failed");
            return;
        }

        defer! {
            let _ = duplicator.stop();
        }

        loop {
            match capture_frame_rx.blocking_recv() {
                Some(capture_frame) => {
                    if let Err(err) = encoder.encode(capture_frame) {
                        if let CoreError::OutgoingMessageChannelDisconnect = err {
                            tracing::info!("desktop capture and encode process exit");
                            client.close();
                            return;
                        } else {
                            tracing::error!("video encode failed");
                            break;
                        }
                    }
                }
                None => {
                    tracing::error!("capture frame rx recv error");
                    break;
                }
            }
        }
    });
}

#[cfg(target_os = "windows")]
fn spawn_desktop_capture_and_encode_process(client: EndPointClient) {
    let monitors = match get_active_monitors(false) {
        Ok(params) => params,
        Err(err) => {
            tracing::error!(?err, "get_active_monitors failed");
            return;
        }
    };

    let (capture_frame_tx, mut capture_frame_rx) = tokio::sync::mpsc::channel(180);

    tokio::task::spawn_blocking(move || {
        defer! {
            tracing::info!( "desktop capture process exit");
        }

        let primary_monitor = monitors.iter().find(|monitor| monitor.is_primary);

        let (mut duplicator, monitor_id) =
            match Duplicator::new(primary_monitor.map(|monitor| monitor.id.to_owned())) {
                Ok(duplicator) => duplicator,
                Err(err) => {
                    tracing::error!(?err, "initialize encoder failed");
                    return;
                }
            };

        let select_monitor = match monitors
            .into_iter()
            .find(|monitor| monitor.id == monitor_id)
        {
            Some(monitor) => monitor,
            None => {
                tracing::error!("can't find selected monitor");
                return;
            }
        };

        PASSIVE_ENDPOINTS_MONITORS.insert(client.id, select_monitor);

        loop {
            match duplicator.capture() {
                Ok(capture_frame) => {
                    if let Err(_) = capture_frame_tx.blocking_send(capture_frame) {
                        tracing::error!("capture frame tx closed");
                        return;
                    }
                }
                Err(err) => {
                    tracing::error!(?err, "desktop duplicator capture failed");
                    break;
                }
            };
        }
    });

    tokio::task::spawn_blocking(move || {
        loop {
            // defer! {
            //     tracing::info!(?active_device_id, ?passive_device_id, "video encode process exit");
            // }

            let mut encoder = match VideoEncoder::new(EncoderType::Libx264, client.clone()) {
                Ok(encoder) => encoder,
                Err(err) => {
                    tracing::error!(?err, "video encoder initialize failed");
                    return;
                }
            };

            loop {
                match capture_frame_rx.blocking_recv() {
                    Some(capture_frame) => {
                        if let Err(err) = encoder.encode(capture_frame) {
                            if let CoreError::OutgoingMessageChannelDisconnect = err {
                                tracing::info!("desktop capture and encode process exit");
                                client.close();
                                return;
                            } else {
                                tracing::error!(?err, "video encode failed");
                            }
                        }
                    }
                    None => {
                        tracing::error!("capture frame channel closed");
                        return;
                    }
                }
            }
        }
    });
}

fn spawn_audio_capture_and_encode_process(client: EndPointClient) {
    let mut exit_rx = client.exit_tx.new_receiver();

    tokio::task::spawn_blocking(move || loop {
        match exit_rx.try_recv() {
            Ok(_) => return,
            Err(err) => match err {
                async_broadcast::TryRecvError::Empty => {}
                _ => return,
            },
        }

        let (stream, mut rx) = match new_record_stream_and_rx() {
            Ok((stream, rx)) => (stream, rx),
            Err(err) => {
                tracing::error!(?err, "initialize audio record stream failed");
                continue;
            }
        };

        if let Err(err) = stream.play() {
            tracing::error!(?err, "play audio stream failed");
            continue;
        }

        let mut audio_encoder = AudioEncoder::default();

        loop {
            match exit_rx.try_recv() {
                Ok(_) => return,
                Err(err) => match err {
                    async_broadcast::TryRecvError::Empty => {}
                    _ => return,
                },
            }

            match rx.blocking_recv() {
                Some(audio_frame) => match audio_encoder.encode(audio_frame) {
                    Ok(frame) => {
                        if let Err(err) = client.send_audio_frame(frame) {
                            match err {
                                CoreError::OutgoingMessageChannelDisconnect => {
                                    tracing::info!("audio encode process exit");
                                    return;
                                }
                                _ => {
                                    tracing::error!(?err, "audio encode failed");
                                }
                            }
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "audio encode failed");
                        break;
                    }
                },
                None => {
                    tracing::error!("audio duplicator tx closed");
                    break;
                }
            }
        }
    });
}
