mod event;

use crate::{send_event, utility::format_device_id};
use event::Event;
use mirrorx_core::{
    api::endpoint::{
        client::EndPointClient, create_active_endpoint_client, id::EndPointID,
        message::EndPointNegotiateDesktopParamsResponse, EndPointStream,
    },
    core_error,
    error::{CoreError, CoreResult},
    utility::nonce_value::NonceValue,
    DesktopDecodeFrame,
};
use ring::aead::{BoundKey, OpeningKey, SealingKey};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};

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

    format_remote_device_id: String,
    visit_state: VisitState,
    endpoint_client: Option<Arc<EndPointClient>>,
    desktop_frame_scaled: bool,
    desktop_frame_scalable: bool,
    last_error: Option<CoreError>,
    render_rx: Option<Receiver<DesktopDecodeFrame>>,
    current_frame: Option<DesktopDecodeFrame>,
}

impl State {
    pub fn new(
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        visit_credentials: String,
        addr: SocketAddr,
    ) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(360);

        send_event!(
            tx,
            Event::ConnectEndPoint {
                endpoint_id,
                key_pair,
                visit_credentials,
                addr,
            }
        );

        let mut format_remote_device_id = match endpoint_id {
            EndPointID::DeviceID { local, remote } => format_device_id(remote),
            EndPointID::LANID { local, remote } => remote.to_string(),
        };

        Self {
            tx,
            rx,
            format_remote_device_id,
            visit_state: VisitState::Connecting,
            endpoint_client: None,
            desktop_frame_scaled: true,
            desktop_frame_scalable: true,
            last_error: None,
            render_rx: None,
            current_frame: None,
        }
    }

    pub fn format_remote_device_id(&self) -> &str {
        self.format_remote_device_id.as_ref()
    }

    pub fn endpoint_client(&self) -> Option<Arc<EndPointClient>> {
        self.endpoint_client.clone()
    }

    pub fn visit_state(&self) -> &VisitState {
        &self.visit_state
    }

    pub fn desktop_frame_scaled(&self) -> bool {
        self.desktop_frame_scaled
    }

    pub fn last_error(&self) -> Option<&CoreError> {
        self.last_error.as_ref()
    }

    pub fn current_frame(&mut self) -> Option<DesktopDecodeFrame> {
        if let Some(rx) = &mut self.render_rx {
            while let Ok(frame) = rx.try_recv() {
                self.current_frame = Some(frame);
            }
        }

        self.current_frame.clone()
    }

    pub fn desktop_frame_scalable(&self) -> bool {
        self.desktop_frame_scalable
    }
}

impl State {
    pub fn set_desktop_frame_scaled(&mut self, scaled: bool) {
        self.desktop_frame_scaled = scaled
    }

    pub fn set_desktop_frame_scalable(&mut self, scalable: bool) {
        self.desktop_frame_scalable = scalable
    }
}

impl State {
    pub fn handle_event(&mut self, ctx: &tauri_egui::egui::Context) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::ConnectEndPoint {
                    endpoint_id,
                    key_pair,
                    visit_credentials,
                    addr,
                } => {
                    self.connect_endpoint(endpoint_id, key_pair, visit_credentials, addr);
                }
                Event::UpdateEndPointClient { client } => self.endpoint_client = Some(client),
                Event::UpdateVisitState { new_state } => self.visit_state = new_state,
                Event::UpdateUseOriginalResolution {
                    use_original_resolution,
                } => self.desktop_frame_scaled = use_original_resolution,
                Event::UpdateError { err } => {
                    tracing::error!(?err, "update error event");
                    self.last_error = Some(err);
                }

                Event::EmitNegotiateDesktopParams => self.emit_negotiate_desktop_params(),
                Event::EmitNegotiateFinish {
                    expected_frame_rate,
                } => self.emit_negotiate_finish(expected_frame_rate),
                Event::SetRenderFrameReceiver { render_rx } => self.render_rx = Some(render_rx),
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn connect_endpoint(
        &mut self,
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        visit_credentials: String,
        addr: SocketAddr,
    ) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            send_event!(
                tx,
                Event::UpdateVisitState {
                    new_state: VisitState::Negotiating
                }
            );

            match create_active_endpoint_client(
                endpoint_id,
                key_pair,
                EndPointStream::PublicTCP(addr),
            )
            .await
            {
                Ok((client, render_frame_rx)) => {
                    send_event!(
                        tx,
                        Event::UpdateVisitState {
                            new_state: VisitState::Serving
                        }
                    );
                    send_event!(tx, Event::UpdateEndPointClient { client });
                    send_event!(
                        tx,
                        Event::SetRenderFrameReceiver {
                            render_rx: render_frame_rx
                        }
                    );
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

    fn emit_negotiate_desktop_params(&mut self) {
        if let Some(client) = &self.endpoint_client {
            let tx = self.tx.clone();
            let client = client.clone();
            tokio::spawn(async move {
                // let resp_rx = match client.negotiate_desktop_params() {
                //     Ok(resp_rx) => resp_rx,
                //     Err(err) => {
                //         send_event!(
                //             tx,
                //             Event::UpdateVisitState {
                //                 new_state: VisitState::ErrorOccurred
                //             }
                //         );
                //         send_event!(tx, Event::UpdateError { err });
                //         return;
                //     }
                // };

                // let resp = async {
                //     tokio::time::timeout(Duration::from_secs(30), resp_rx)
                //         .await
                //         .map_err(CoreError::Timeout)?
                //         .map_err(|err| {
                //             core_error!(
                //                 "negotiate desktop params response receive failed ({})",
                //                 err
                //             )
                //         })
                // }
                // .await;

                // match resp {
                //     Ok(resp) => match resp {
                //         EndPointNegotiateDesktopParamsResponse::Error => {
                //             send_event!(
                //                 tx,
                //                 Event::UpdateVisitState {
                //                     new_state: VisitState::ErrorOccurred
                //                 }
                //             );
                //             send_event!(
                //                 tx,
                //                 Event::UpdateError {
                //                     err: core_error!("negotiate desktop params failed",)
                //                 }
                //             );
                //         }
                //         EndPointNegotiateDesktopParamsResponse::Params(params) => {
                //             send_event!(
                //                 tx,
                //                 Event::EmitNegotiateFinish {
                //                     expected_frame_rate: 60
                //                 }
                //             );
                //         }
                //     },
                //     Err(err) => {
                //         send_event!(
                //             tx,
                //             Event::UpdateVisitState {
                //                 new_state: VisitState::ErrorOccurred
                //             }
                //         );
                //         send_event!(tx, Event::UpdateError { err })
                //     }
                // }
            });
        }
    }

    fn emit_negotiate_finish(&mut self, expected_frame_rate: u8) {
        if let Some(client) = &self.endpoint_client {
            let mut client = client.clone();
            let tx = self.tx.clone();

            tokio::spawn(async move {
                // match client.negotiate_finish(expected_frame_rate).await {
                //     Ok(render_rx) => {
                //         send_event!(tx, Event::SetRenderFrameReceiver { render_rx });

                //         send_event!(
                //             tx,
                //             Event::UpdateVisitState {
                //                 new_state: VisitState::Serving
                //             }
                //         );
                //     }
                //     Err(err) => {
                //         send_event!(tx, Event::UpdateError { err });
                //     }
                // }
            });
        }
    }
}
