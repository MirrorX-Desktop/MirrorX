use crate::{
    component::{
        audio::{encoder::AudioEncoder, recorder::new_record_stream_and_rx},
        screen::Screen,
    },
    core_error,
    error::{CoreError, CoreResult},
    service::endpoint::{
        self,
        message::{EndPointMessage, EndPointSwitchScreenReply, EndPointSwitchScreenRequest},
        Service,
    },
};
use cpal::traits::StreamTrait;
use std::sync::Arc;

pub async fn handle_switch_screen_request(
    service: Arc<Service>,
    req: EndPointSwitchScreenRequest,
) -> CoreResult<EndPointSwitchScreenReply> {
    let new_screen = Screen::new(&req.display_id, service.clone())?;
    if service.update_screen(new_screen).await.is_err() {
        return Err(core_error!("switch screen failed"));
    }

    spawn_audio_capture_and_encode_process(service);

    Ok(EndPointSwitchScreenReply {
        display_id: req.display_id,
    })
}

#[cfg(target_os = "macos")]
fn spawn_desktop_capture_and_encode_process(client: Arc<EndPointClient>) {
    let (capture_frame_tx, mut capture_frame_rx) = tokio::sync::mpsc::channel(180);

    tokio::task::spawn_blocking(move || {
        tracing::info_span!("desktop_capture_and_encode_process", client = ?client);

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

        let mut encoder = match VideoEncoder::new(libx264::Libx264Config::default(), client.clone())
        {
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

        // PASSIVE_ENDPOINTS_MONITORS.insert(client.id, select_monitor);

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

// #[cfg(target_os = "windows")]
// fn spawn_desktop_capture_and_encode_process(client: Arc<EndPointClient>) {
//     let (capture_frame_tx, mut capture_frame_rx) = tokio::sync::mpsc::channel(180);

//     tokio::task::spawn_blocking(move || {
//         defer! {
//             tracing::info!( "desktop capture process exit");
//         }

//         let primary_monitor = monitors.iter().find(|monitor| monitor.is_primary);

//         let (mut duplicator, _) =
//             match Duplicator::new(primary_monitor.map(|monitor| monitor.id.to_owned())) {
//                 Ok(duplicator) => duplicator,
//                 Err(err) => {
//                     tracing::error!(?err, "initialize encoder failed");
//                     return;
//                 }
//             };

//         loop {
//             match duplicator.capture() {
//                 Ok(capture_frame) => {
//                     if capture_frame_tx.blocking_send(capture_frame).is_err() {
//                         return;
//                     }
//                 }
//                 Err(err) => {
//                     tracing::error!(?err, "desktop duplicator capture failed");
//                     break;
//                 }
//             };
//         }
//     });

//     tokio::task::spawn_blocking(move || {
//         loop {
//             // defer! {
//             //     tracing::info!(?active_device_id, ?passive_device_id, "video encode process exit");
//             // }

//             let mut encoder =
//                 match VideoEncoder::new(libx264::Libx264Config::default(), client.clone()) {
//                     Ok(encoder) => encoder,
//                     Err(err) => {
//                         tracing::error!(?err, "video encoder initialize failed");
//                         return;
//                     }
//                 };

//             loop {
//                 match capture_frame_rx.blocking_recv() {
//                     Some(capture_frame) => {
//                         if let Err(err) = encoder.encode(capture_frame) {
//                             if let CoreError::OutgoingMessageChannelDisconnect = err {
//                                 tracing::info!("desktop capture and encode process exit");
//                                 return;
//                             } else {
//                                 tracing::error!(?err, "video encode failed");
//                             }
//                         }
//                     }
//                     None => {
//                         tracing::error!("capture frame channel closed");
//                         return;
//                     }
//                 }
//             }
//         }
//     });
// }

fn spawn_audio_capture_and_encode_process(service: Arc<endpoint::Service>) {
    // let mut exit_rx = client.close_receiver();

    tokio::task::spawn_blocking(move || loop {
        // let Err(async_broadcast::TryRecvError::Empty) = exit_rx.try_recv() else {
        //     tracing::info!("receive exit signal, exit");
        //     return;
        // };

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

        loop {
            let mut audio_encoder = AudioEncoder::default();

            loop {
                // let Err(async_broadcast::TryRecvError::Empty) = exit_rx.try_recv() else {
                //     tracing::info!("receive exit signal, exit");
                //     return;
                // };

                match rx.blocking_recv() {
                    Some(audio_frame) => match audio_encoder.encode(audio_frame) {
                        Ok(frame) => {
                            if let Err(err) =
                                service.blocking_send(&EndPointMessage::AudioFrame(frame))
                            {
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
        }
    });
}
