use std::path::PathBuf;

use mirrorx_core::{
    api::{
        config::Config,
        signaling::{KeyExchangeResponse, SignalingClient, VisitResponse},
    },
    error::CoreError,
};

#[derive(Debug)]
pub enum Event {
    UpdateCurrentPage { page_name: String },
    UpdateConfig { config: Config },
    UpdateConfigSuccess { config: Config },
    UpdateConfigPath { config_path: PathBuf },
    UpdateSignalingClient,
    UpdateSignalingClientSuccess { signaling_client: SignalingClient },
    UpdateSignalingVisitResponse { resp: VisitResponse },
    UpdateSignalingKeyExchangeResponse { resp: KeyExchangeResponse },
    UpdateDialogInputVisitPasswordVisible { visible: bool },
    UpdateDialogKeyExchangeProcessingVisible { visible: bool },
    UpdateConnectPagePasswordVisible { visible: bool },
    UpdateConnectPagePasswordEditing { editing: bool },
    UpdateConnectPagePassword { password: String },
    UpdateConnectPageVisitDeviceId { device_id: String },
    UpdateConnectPageDesktopConnecting { connecting: bool },
    UpdateLastError { err: CoreError },
}
