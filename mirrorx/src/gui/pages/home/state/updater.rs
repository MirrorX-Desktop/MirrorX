use super::event::Event;
use crate::send_event;
use mirrorx_core::{
    api::{
        config::Config,
        signaling::{KeyExchangeResponse, ResourceType},
    },
    error::CoreError,
};
use std::path::Path;
use tokio::sync::mpsc::UnboundedSender;

pub struct StateUpdater {
    tx: UnboundedSender<Event>,
}

impl StateUpdater {
    pub fn new(tx: UnboundedSender<Event>) -> Self {
        Self { tx }
    }

    pub fn update_current_page_name(&self, page_name: &str) {
        send_event!(
            self.tx,
            Event::UpdateCurrentPage {
                page_name: page_name.to_string(),
            }
        )
    }

    pub fn update_config(&self, config: &Config) {
        send_event!(
            self.tx,
            Event::UpdateConfig {
                config: config.clone(),
            }
        )
    }

    pub fn update_config_path(&self, config_path: &Path) {
        send_event!(
            self.tx,
            Event::UpdateConfigPath {
                config_path: config_path.to_path_buf(),
            }
        )
    }

    pub fn update_signaling_client(&self) {
        send_event!(self.tx, Event::UpdateSignalingClient)
    }

    pub fn update_signaling_key_exchange_response(&self, resp: &KeyExchangeResponse) {
        send_event!(
            self.tx,
            Event::UpdateSignalingKeyExchangeResponse { resp: resp.clone() }
        )
    }

    pub fn update_dialog_input_visit_password_visible(&self, visible: Option<(i64, i64)>) {
        send_event!(
            self.tx,
            Event::UpdateDialogInputVisitPasswordVisible { visible }
        )
    }

    pub fn update_dialog_input_visit_password(&self, password: &str) {
        send_event!(
            self.tx,
            Event::UpdateDialogInputVisitPassword {
                password: password.to_string(),
            }
        )
    }

    pub fn update_dialog_key_exchange_processing_visible(&self, visible: bool) {
        send_event!(
            self.tx,
            Event::UpdateDialogKeyExchangeProcessingVisible { visible }
        )
    }

    pub fn update_dialog_visit_request_visible(&self, visible: Option<(i64, i64, ResourceType)>) {
        send_event!(self.tx, Event::UpdateDialogVisitRequestVisible { visible })
    }

    pub fn update_connect_page_password_visible(&self, visible: bool) {
        send_event!(self.tx, Event::UpdateConnectPagePasswordVisible { visible })
    }

    pub fn update_connect_page_password_editing(&self, editing: bool) {
        send_event!(self.tx, Event::UpdateConnectPagePasswordEditing { editing })
    }

    pub fn update_connect_page_password(&self, password: &str) {
        send_event!(
            self.tx,
            Event::UpdateConnectPagePassword {
                password: password.to_string(),
            }
        )
    }

    pub fn update_connect_page_visit_device_id(&self, device_id: &str) {
        send_event!(
            self.tx,
            Event::UpdateConnectPageVisitDeviceId {
                device_id: device_id.to_string(),
            }
        )
    }

    pub fn update_connect_page_desktop_connecting(&self, connecting: bool) {
        send_event!(
            self.tx,
            Event::UpdateConnectPageDesktopConnecting { connecting }
        )
    }

    pub fn update_last_error(&self, error: CoreError) {
        send_event!(self.tx, Event::UpdateLastError { err: error })
    }

    pub fn emit_signaling_visit(
        &self,
        local_device_id: i64,
        remote_device_id: i64,
        resource_type: ResourceType,
    ) {
        send_event!(
            self.tx,
            Event::EmitSignalingVisit {
                local_device_id,
                remote_device_id,
                resource_type,
            }
        )
    }

    pub fn emit_signaling_visit_reply(
        &self,
        allow: bool,
        active_device_id: i64,
        passive_device_id: i64,
    ) {
        send_event!(
            self.tx,
            Event::EmitSignalingVisitReply {
                allow,
                active_device_id,
                passive_device_id,
            }
        )
    }

    pub fn emit_signaling_key_exchange(&self, active_device_id: i64, passive_device_id: i64) {
        send_event!(
            self.tx,
            Event::EmitSignalingKeyExchange {
                active_device_id,
                passive_device_id,
            }
        )
    }
}
