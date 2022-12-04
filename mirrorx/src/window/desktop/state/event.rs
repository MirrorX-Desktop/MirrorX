use super::VisitState;
use mirrorx_core::{
    api::endpoint::{client::EndPointClient, id::EndPointID},
    error::CoreError,
    utility::nonce_value::NonceValue,
    DesktopDecodeFrame,
};
use ring::aead::{OpeningKey, SealingKey};
use std::{net::SocketAddr, sync::Arc};
use strum_macros::AsRefStr;
use tokio::sync::mpsc::Receiver;

#[derive(AsRefStr)]
pub enum Event {
    ConnectEndPoint {
        endpoint_id: EndPointID,
        key_pair: Option<(OpeningKey<NonceValue>, SealingKey<NonceValue>)>,
        visit_credentials: Option<String>,
        addr: SocketAddr,
    },

    UpdateEndPointClient {
        client: Arc<EndPointClient>,
    },

    UpdateVisitState {
        new_state: VisitState,
    },

    UpdateUseOriginalResolution {
        use_original_resolution: bool,
    },

    UpdateError {
        err: CoreError,
    },

    SetRenderFrameReceiver {
        render_rx: Receiver<DesktopDecodeFrame>,
    },
}
