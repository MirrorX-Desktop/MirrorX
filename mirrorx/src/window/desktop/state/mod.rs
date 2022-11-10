mod event;
mod updater;

use crate::{send_event, utility::format_device_id};
use event::Event;
use mirrorx_core::{
    api::endpoint::{message::EndPointNegotiateDesktopParamsResponse, EndPointClient},
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
    DesktopDecodeFrame,
};
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use std::sync::Mutex;
use std::{sync::Arc, time::Duration};
use tauri_egui::{
    eframe::{
        egui_glow::{self, check_for_gl_error},
        glow::{self, HasContext, NativeTexture},
    },
    egui::{ColorImage, TextureHandle, TextureId},
};
use tokio::sync::mpsc::{Receiver, Sender};

pub use updater::StateUpdater;

#[macro_export]
macro_rules! send_event {
    ($tx:expr, $event:expr) => {
        if let Err(err) = $tx.try_send($event) {
            tracing::error!("send event failed");
        }
    };
}

#[derive(Debug)]
pub enum VisitState {
    Connecting,
    Negotiating,
    Serving,
    ErrorOccurred,
}

pub struct State {
    tx: Sender<Event>,
    rx: Receiver<Event>,

    // window_id: WindowId,
    local_device_id: i64,
    format_local_device_id: String,
    remote_device_id: i64,
    format_remote_device_id: String,

    visit_state: VisitState,
    endpoint_client: Option<EndPointClient>,
    desktop_texture: Option<TextureHandle>,
    frame_count: i64,
    use_original_resolution: bool,

    last_error: Option<CoreError>,

    render_rx: Option<crossbeam::channel::Receiver<DesktopDecodeFrame>>,
    current_frame: Option<DesktopDecodeFrame>,
}

impl State {
    pub fn new(
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
    ) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(360);

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

        let mut format_local_device_id = format_device_id(local_device_id);
        let mut format_remote_device_id = format_device_id(remote_device_id);

        Self {
            tx,
            rx,
            local_device_id,
            format_local_device_id,
            remote_device_id,
            format_remote_device_id,
            visit_state: VisitState::Connecting,
            endpoint_client: None,
            desktop_texture: None,
            use_original_resolution: true,
            frame_count: 0,
            last_error: None,
            render_rx: None,
            current_frame: None,
        }
    }

    pub fn local_device_id(&self) -> i64 {
        self.local_device_id
    }

    pub fn format_local_device_id(&self) -> &str {
        self.format_local_device_id.as_ref()
    }

    pub fn remote_device_id(&self) -> i64 {
        self.remote_device_id
    }

    pub fn format_remote_device_id(&self) -> &str {
        self.format_remote_device_id.as_ref()
    }

    pub fn endpoint_client(&self) -> Option<&EndPointClient> {
        self.endpoint_client.as_ref()
    }

    pub fn visit_state(&self) -> &VisitState {
        &self.visit_state
    }

    pub fn desktop_texture(&self) -> Option<&TextureHandle> {
        self.desktop_texture.as_ref()
    }

    pub fn use_original_resolution(&self) -> bool {
        self.use_original_resolution
    }

    pub fn last_error(&self) -> Option<&CoreError> {
        self.last_error.as_ref()
    }

    pub fn frame(&mut self) -> Option<DesktopDecodeFrame> {
        if let Some(rx) = &mut self.render_rx {
            while let Ok(frame) = rx.try_recv() {
                self.current_frame = Some(frame);
            }
        }

        self.current_frame.clone()
    }
}

impl State {
    pub fn new_state_updater(&self) -> StateUpdater {
        StateUpdater::new(self.tx.clone())
    }

    pub fn handle_event(&mut self, ctx: &tauri_egui::egui::Context) {
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
                Event::UpdateRenderFrameReceiver { render_rx } => self.render_rx = Some(render_rx),
                Event::UpdateUseOriginalResolution {
                    use_original_resolution,
                } => self.use_original_resolution = use_original_resolution,
                Event::UpdateError { err } => {
                    tracing::error!(?err, "update error event");
                    self.last_error = Some(err);
                }
                Event::Input { input_series } => {
                    if let Some(client) = &self.endpoint_client {
                        if let Err(err) = client.send_input_command(input_series) {
                            tracing::error!(?err, "endpoint input failed");
                        }
                    }
                }
                Event::EmitNegotiateDesktopParams => self.emit_negotiate_desktop_params(),
                Event::EmitNegotiateFinish {
                    expected_frame_rate,
                } => self.emit_negotiate_finish(expected_frame_rate),
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
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
            let mut client = client.clone();
            let tx = self.tx.clone();
            tokio::spawn(async move {
                match client.negotiate_finish(expected_frame_rate).await {
                    Ok(render_rx) => {
                        send_event!(tx, Event::UpdateRenderFrameReceiver { render_rx });

                        send_event!(
                            tx,
                            Event::UpdateVisitState {
                                new_state: VisitState::Serving
                            }
                        );
                    }
                    Err(err) => {
                        send_event!(tx, Event::UpdateError { err });
                    }
                }
            });
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if let Some(endpoint_client) = &self.endpoint_client {
            endpoint_client.close();
        }
    }
}
