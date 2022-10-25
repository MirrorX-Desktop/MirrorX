use egui::ColorImage;
use mirrorx_core::{api::endpoint::EndPointClient, error::CoreError};
use strum_macros::AsRefStr;

use super::VisitState;

#[derive(AsRefStr)]
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

    UpdateFrameImage {
        frame_image: ColorImage,
    },

    UpdateUseOriginalResolution {
        use_original_resolution: bool,
    },

    UpdateError {
        err: CoreError,
    },

    EmitNegotiateDesktopParams,
    EmitNegotiateFinish {
        expected_frame_rate: u8,
    },
}
