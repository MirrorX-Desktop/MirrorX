use mirrorx_core::{api::endpoint::EndPointClient, error::CoreError};
use strum_macros::AsRefStr;

use super::VisitState;

#[derive(Debug, AsRefStr)]
pub enum Event {
    ConnectEndPoint {
        local_device_id: i64,
        remote_device_id: i64,
        opening_key: Vec<u8>,
        opening_nonce: Vec<u8>,
        sealing_key: Vec<u8>,
        sealing_nonce: Vec<u8>,
        visit_credentials: String,
    },

    UpdateEndPointClient {
        client: EndPointClient,
    },

    UpdateVisitState {
        new_state: VisitState,
    },

    UpdateError {
        err: CoreError,
    },

    EmitNegotiateDesktopParams,
    EmitNegotiateFinish {
        expected_frame_rate: u8,
    },
}
