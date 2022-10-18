use std::path::PathBuf;

use mirrorx_core::{
    api::{
        config::Config,
        signaling::{
            KeyExchangeResponse, PublishMessage, ResourceType, SignalingClient, VisitResponse,
        },
    },
    error::CoreError,
};

#[derive(Debug)]
pub enum Event {
    UpdateCurrentPage {
        page_name: String,
    },
    UpdateConfig {
        config: Config,
    },
    UpdateConfigSuccess {
        config: Config,
    },
    UpdateConfigPath {
        config_path: PathBuf,
    },
    UpdateSignalingClient,
    UpdateSignalingClientSuccess {
        signaling_client: SignalingClient,
    },
    UpdateSignalingPublishMessage {
        publish_message: PublishMessage,
    },
    UpdateSignalingKeyExchangeResponse {
        resp: KeyExchangeResponse,
    },
    UpdateDialogInputVisitPasswordVisible {
        visible: Option<(i64, i64)>,
    },
    UpdateDialogInputVisitPassword {
        password: String,
    },
    UpdateDialogKeyExchangeProcessingVisible {
        visible: bool,
    },
    UpdateDialogVisitRequestVisible {
        visible: Option<(i64, i64, ResourceType)>,
    },
    UpdateConnectPagePasswordVisible {
        visible: bool,
    },
    UpdateConnectPagePasswordEditing {
        editing: bool,
    },
    UpdateConnectPagePassword {
        password: String,
    },
    UpdateConnectPageVisitDeviceId {
        device_id: String,
    },
    UpdateConnectPageDesktopConnecting {
        connecting: bool,
    },
    UpdateLastError {
        err: CoreError,
    },

    EmitSignalingVisit {
        local_device_id: i64,
        remote_device_id: i64,
        resource_type: ResourceType,
    },
    EmitSignalingVisitReply {
        active_device_id: i64,
        passive_device_id: i64,
        allow: bool,
    },
    EmitSignalingKeyExchange {
        active_device_id: i64,
        passive_device_id: i64,
    },
}
