mod event;
mod updater;

use crate::{gui::CustomEvent, send_event};
use egui::ColorImage;
use event::Event;
use mirrorx_core::{
    api::endpoint::{message::EndPointNegotiateDesktopParamsResponse, EndPointClient},
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
    DesktopDecodeFrame,
};
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender};
use winit::{event_loop::EventLoopProxy, window::WindowId};

pub use updater::StateUpdater;

#[derive(Debug)]
pub enum VisitState {
    Connecting,
    Negotiating,
    Serving,
    ErrorOccurred,
}

pub struct State {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,

    // window_id: WindowId,
    local_device_id: i64,
    remote_device_id: i64,

    visit_state: VisitState,
    endpoint_client: Option<EndPointClient>,
    frame_image: Option<ColorImage>,
    event_loop_proxy: EventLoopProxy<CustomEvent>,

    last_error: Option<CoreError>,
}

impl State {
    pub fn new(
        // window_id: WindowId,
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
        event_loop_proxy: EventLoopProxy<CustomEvent>,
    ) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        send_event!(
            tx,
            Event::ConnectEndPoint {
                local_device_id,
                remote_device_id,
                opening_key,
                opening_nonce,
                sealing_key,
                sealing_nonce,
                visit_credentials,
            }
        );

        Self {
            tx,
            rx,
            // window_id,
            local_device_id,
            remote_device_id,
            visit_state: VisitState::Connecting,
            endpoint_client: None,
            frame_image: None,
            event_loop_proxy,
            last_error: None,
        }
    }

    pub fn local_device_id(&self) -> i64 {
        self.local_device_id
    }

    pub fn remote_device_id(&self) -> i64 {
        self.remote_device_id
    }

    pub fn endpoint_client(&self) -> Option<&EndPointClient> {
        self.endpoint_client.as_ref()
    }

    pub fn visit_state(&self) -> &VisitState {
        &self.visit_state
    }

    pub fn take_frame_image(&mut self) -> Option<ColorImage> {
        self.frame_image.take()
    }

    pub fn last_error(&self) -> Option<&CoreError> {
        self.last_error.as_ref()
    }

    pub fn take_last_error(&mut self) -> Option<CoreError> {
        self.last_error.take()
    }
}

impl State {
    pub fn new_state_updater(&self) -> StateUpdater {
        StateUpdater::new(self.tx.clone())
    }

    pub fn handle_event(&mut self) {
        tracing::info!("handle event begin");
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::ConnectEndPoint {
                    local_device_id,
                    remote_device_id,
                    opening_key,
                    opening_nonce,
                    sealing_key,
                    sealing_nonce,
                    visit_credentials,
                } => {
                    self.connect_endpoint(
                        local_device_id,
                        remote_device_id,
                        sealing_key,
                        sealing_nonce,
                        opening_key,
                        opening_nonce,
                        visit_credentials,
                    );
                }
                Event::UpdateEndPointClient { client } => self.endpoint_client = Some(client),
                Event::UpdateVisitState { new_state } => self.visit_state = new_state,
                Event::UpdateFrameImage { frame_image } => {
                    tracing::info!("update frame image");
                    self.frame_image = Some(frame_image);
                    // self.event_loop_proxy
                    //     .send_event(CustomEvent::Repaint(self.window_id));
                    // tracing::info!("send repaint event: {:?}", self.window_id);
                    return;
                }
                Event::UpdateError { err } => {
                    tracing::error!(?err, "update error event");
                    self.last_error = Some(err);
                }
                Event::EmitNegotiateDesktopParams => self.emit_negotiate_desktop_params(),
                Event::EmitNegotiateFinish {
                    expected_frame_rate,
                } => self.emit_negotiate_finish(expected_frame_rate),
            }
        }
        tracing::info!("handle event end");
    }

    fn connect_endpoint(
        &mut self,
        local_device_id: i64,
        remote_device_id: i64,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        visit_credentials: String,
    ) {
        let res = || -> CoreResult<(SealingKey<NonceValue>, OpeningKey<NonceValue>)> {
            let unbound_sealing_key =
                ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &sealing_key)?;

            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&sealing_nonce);
            let sealing_key =
                ring::aead::SealingKey::new(unbound_sealing_key, NonceValue::new(nonce));

            let unbound_opening_key =
                ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &opening_key)?;

            let mut nonce = [0u8; 12];
            nonce.copy_from_slice(&opening_nonce);
            let opening_key =
                ring::aead::OpeningKey::new(unbound_opening_key, NonceValue::new(nonce));

            Ok((sealing_key, opening_key))
        }();

        match res {
            Ok((sealing_key, opening_key)) => {
                let tx = self.tx.clone();
                tokio::spawn(async move {
                    match mirrorx_core::api::endpoint::EndPointClient::new(
                        local_device_id,
                        remote_device_id,
                        opening_key,
                        sealing_key,
                        visit_credentials,
                    )
                    .await
                    {
                        Ok(client) => {
                            send_event!(
                                tx,
                                Event::UpdateVisitState {
                                    new_state: VisitState::Negotiating
                                }
                            );
                            send_event!(tx, Event::UpdateEndPointClient { client });
                            send_event!(tx, Event::EmitNegotiateDesktopParams);
                        }
                        Err(err) => {
                            send_event!(
                                tx,
                                Event::UpdateVisitState {
                                    new_state: VisitState::ErrorOccurred
                                }
                            );
                            send_event!(tx, Event::UpdateError { err });
                        }
                    }
                });
            }
            Err(err) => {
                send_event!(
                    self.tx,
                    Event::UpdateVisitState {
                        new_state: VisitState::ErrorOccurred
                    }
                );
                send_event!(self.tx, Event::UpdateError { err });
            }
        }
    }

    fn emit_negotiate_desktop_params(&mut self) {
        if let Some(client) = &self.endpoint_client {
            let tx = self.tx.clone();
            let client = client.clone();
            tokio::spawn(async move {
                let resp_rx = match client.negotiate_desktop_params() {
                    Ok(resp_rx) => resp_rx,
                    Err(err) => {
                        send_event!(
                            tx,
                            Event::UpdateVisitState {
                                new_state: VisitState::ErrorOccurred
                            }
                        );
                        send_event!(tx, Event::UpdateError { err });
                        return;
                    }
                };

                let resp = async {
                    tokio::time::timeout(Duration::from_secs(30), resp_rx)
                        .await
                        .map_err(CoreError::Timeout)?
                        .map_err(|err| {
                            core_error!(
                                "negotiate desktop params response receive failed ({})",
                                err
                            )
                        })
                }
                .await;

                match resp {
                    Ok(resp) => match resp {
                        EndPointNegotiateDesktopParamsResponse::Error => {
                            send_event!(
                                tx,
                                Event::UpdateVisitState {
                                    new_state: VisitState::ErrorOccurred
                                }
                            );
                            send_event!(
                                tx,
                                Event::UpdateError {
                                    err: core_error!("negotiate desktop params failed",)
                                }
                            );
                        }
                        EndPointNegotiateDesktopParamsResponse::Params(params) => {
                            // todo: prepare wgpu texture
                            send_event!(
                                tx,
                                Event::EmitNegotiateFinish {
                                    expected_frame_rate: 60
                                }
                            );
                        }
                    },
                    Err(err) => {
                        send_event!(
                            tx,
                            Event::UpdateVisitState {
                                new_state: VisitState::ErrorOccurred
                            }
                        );
                        send_event!(tx, Event::UpdateError { err })
                    }
                }
            });
        }
    }

    fn emit_negotiate_finish(&mut self, expected_frame_rate: u8) {
        if let Some(client) = &self.endpoint_client {
            match client.negotiate_finish(expected_frame_rate) {
                Ok(mut frame_rx) => {
                    let event_tx = self.tx.clone();
                    tokio::spawn(async move {
                        while let Some(desktop_decode_frame) = frame_rx.recv().await {
                            let size = [
                                desktop_decode_frame.width as _,
                                desktop_decode_frame.height as _,
                            ];
                            tracing::info!("receive desktop decode frame");

                            send_event!(
                                event_tx,
                                Event::UpdateFrameImage {
                                    frame_image: ColorImage::from_rgba_unmultiplied(
                                        size,
                                        &desktop_decode_frame.data,
                                    )
                                }
                            );
                            tracing::info!("send update frame image event end");
                        }
                    });

                    send_event!(
                        self.tx,
                        Event::UpdateVisitState {
                            new_state: VisitState::Serving
                        }
                    );
                }
                Err(err) => {
                    send_event!(self.tx, Event::UpdateError { err });
                }
            }
        }
    }
}
