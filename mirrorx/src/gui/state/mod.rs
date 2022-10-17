mod event;

use self::event::Event;
use mirrorx_core::{
    api::{
        config::Config,
        signaling::{KeyExchangeResponse, SignalingClient, VisitResponse},
    },
    error::CoreError,
};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct State {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,

    current_page_name: String,
    config: Option<Config>,
    config_path: PathBuf,
    signaling_client: Option<SignalingClient>,
    signaling_visit_response: Option<VisitResponse>,
    signaling_key_exchange_response: Option<KeyExchangeResponse>,
    dialog_input_visit_password_visible: bool,
    dialog_key_exchange_processing_visible: bool,
    connect_page_password_visible: bool,
    connect_page_password_editing: bool,
    connect_page_password: String,
    connect_page_visit_device_id: String,
    connect_page_desktop_connecting: bool,

    last_error: Option<CoreError>,
}

impl State {
    pub fn new(initial_page_name: &str) -> Self {
        let (tx, rx) = unbounded_channel();

        Self {
            tx,
            rx,
            current_page_name: initial_page_name.to_string(),
            config: Default::default(),
            config_path: Default::default(),
            signaling_client: Default::default(),
            signaling_visit_response: Default::default(),
            signaling_key_exchange_response: Default::default(),
            dialog_input_visit_password_visible: Default::default(),
            dialog_key_exchange_processing_visible: Default::default(),
            connect_page_password_visible: Default::default(),
            connect_page_password_editing: Default::default(),
            connect_page_password: Default::default(),
            connect_page_visit_device_id: Default::default(),
            connect_page_desktop_connecting: Default::default(),
            last_error: Default::default(),
        }
    }
    pub fn current_page_name(&self) -> &str {
        self.current_page_name.as_ref()
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn config_path(&self) -> &Path {
        self.config_path.as_path()
    }

    pub fn signaling_client(&self) -> Option<&SignalingClient> {
        self.signaling_client.as_ref()
    }

    pub fn signaling_visit_response(&self) -> Option<&VisitResponse> {
        self.signaling_visit_response.as_ref()
    }

    pub fn signaling_key_exchange_response(&self) -> Option<&KeyExchangeResponse> {
        self.signaling_key_exchange_response.as_ref()
    }

    pub fn dialog_input_visit_password_visible(&self) -> bool {
        self.dialog_input_visit_password_visible
    }

    pub fn dialog_key_exchange_processing_visible(&self) -> bool {
        self.dialog_key_exchange_processing_visible
    }

    pub fn connect_page_password_visible(&self) -> bool {
        self.connect_page_password_visible
    }

    pub fn connect_page_password_editing(&self) -> bool {
        self.connect_page_password_editing
    }

    pub fn connect_page_password(&self) -> &str {
        self.connect_page_password.as_str()
    }

    pub fn connect_page_visit_device_id(&self) -> &str {
        self.connect_page_visit_device_id.as_str()
    }

    pub fn connect_page_desktop_connecting(&self) -> bool {
        self.connect_page_desktop_connecting
    }

    pub fn take_error(&mut self) -> Option<CoreError> {
        self.last_error.take()
    }
}

impl State {
    pub fn new_state_updater(&self) -> StateUpdater {
        StateUpdater {
            tx: self.tx.clone(),
        }
    }

    pub fn handle_event(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::UpdateCurrentPage { page_name } => self.current_page_name = page_name,
                Event::UpdateConfig { config } => {
                    let config_path = self.config_path.clone();
                    let tx = self.tx.clone();
                    tokio::task::spawn_blocking(move || {
                        if let Err(err) = mirrorx_core::api::config::save(&config_path, &config) {
                            tracing::error!(?err, "config save failed");
                            if tx.send(Event::UpdateLastError { err }).is_err() {
                                tracing::error!("send UpdateLastError event failed");
                            }
                        } else if tx.send(Event::UpdateConfigSuccess { config }).is_err() {
                            tracing::error!("send UpdateLastError event failed");
                        }
                    });
                }
                Event::UpdateConfigSuccess { config } => self.config = Some(config),
                Event::UpdateConfigPath { config_path } => self.config_path = config_path,
                Event::UpdateSignalingClient => {
                    if let Some(config) = &self.config {
                        let config = config.clone();
                        let config_path = self.config_path.clone();
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            match SignalingClient::new(config, config_path).await {
                                Ok((signaling_client, _, _)) => {
                                    // todo: handle config update and publish message receiver
                                    if tx
                                        .send(Event::UpdateSignalingClientSuccess {
                                            signaling_client,
                                        })
                                        .is_err()
                                    {
                                        tracing::error!(
                                            "send UpdateSignalingClientSuccess event failed"
                                        );
                                    }
                                }
                                Err(err) => {
                                    if tx.send(Event::UpdateLastError { err }).is_err() {
                                        tracing::error!("send UpdateLastError event failed");
                                    }
                                }
                            }
                        });
                    }
                }
                Event::UpdateSignalingClientSuccess { signaling_client } => {
                    self.signaling_client = Some(signaling_client)
                }
                Event::UpdateSignalingVisitResponse { resp } => {
                    self.signaling_visit_response = Some(resp)
                }
                Event::UpdateSignalingKeyExchangeResponse { resp } => {
                    self.signaling_key_exchange_response = Some(resp)
                }
                Event::UpdateDialogInputVisitPasswordVisible { visible } => {
                    self.dialog_input_visit_password_visible = visible
                }
                Event::UpdateDialogKeyExchangeProcessingVisible { visible } => {
                    self.dialog_key_exchange_processing_visible = visible
                }
                Event::UpdateConnectPagePasswordVisible { visible } => {
                    self.connect_page_password_visible = visible
                }
                Event::UpdateConnectPagePasswordEditing { editing } => {
                    self.connect_page_password_editing = editing
                }
                Event::UpdateConnectPagePassword { password } => {
                    self.connect_page_password = password
                }
                Event::UpdateConnectPageVisitDeviceId { device_id } => {
                    self.connect_page_visit_device_id = device_id
                }
                Event::UpdateConnectPageDesktopConnecting { connecting } => {
                    self.connect_page_desktop_connecting = connecting
                }
                Event::UpdateLastError { err } => self.last_error = Some(err),
            }
        }
    }
}

pub struct StateUpdater {
    tx: UnboundedSender<Event>,
}

impl StateUpdater {
    pub fn update_current_page_name(&self, page_name: &str) {
        if self
            .tx
            .send(Event::UpdateCurrentPage {
                page_name: page_name.to_string(),
            })
            .is_err()
        {
            tracing::error!("send UpdateCurrentPage event failed");
        }
    }

    pub fn update_config(&self, config: &Config) {
        if self
            .tx
            .send(Event::UpdateConfig {
                config: config.clone(),
            })
            .is_err()
        {
            tracing::error!("send UpdateConfig event failed");
        }
    }

    pub fn update_config_path(&self, config_path: &Path) {
        if self
            .tx
            .send(Event::UpdateConfigPath {
                config_path: config_path.to_path_buf(),
            })
            .is_err()
        {
            tracing::error!("send UpdateConfigPath event failed");
        }
    }

    pub fn update_signaling_client(&self) {
        if self.tx.send(Event::UpdateSignalingClient).is_err() {
            tracing::error!("send UpdateSignalingClient event failed");
        }
    }

    pub fn update_signaling_visit_response(&self, resp: &VisitResponse) {
        if self
            .tx
            .send(Event::UpdateSignalingVisitResponse { resp: resp.clone() })
            .is_err()
        {
            tracing::error!("send UpdateSignalingVisitResponse event failed");
        }
    }

    pub fn update_signaling_key_exchange_response(&self, resp: &KeyExchangeResponse) {
        if self
            .tx
            .send(Event::UpdateSignalingKeyExchangeResponse { resp: resp.clone() })
            .is_err()
        {
            tracing::error!("send UpdateSignalingKeyExchangeResponse event failed");
        }
    }

    pub fn update_dialog_input_visit_password_visible(&self, visible: bool) {
        if self
            .tx
            .send(Event::UpdateDialogInputVisitPasswordVisible { visible })
            .is_err()
        {
            tracing::error!("send UpdateDialogInputVisitPasswordVisible event failed");
        }
    }

    pub fn update_dialog_key_exchange_processing_visible(&self, visible: bool) {
        if self
            .tx
            .send(Event::UpdateDialogKeyExchangeProcessingVisible { visible })
            .is_err()
        {
            tracing::error!("send UpdateDialogKeyExchangeProcessingVisible event failed");
        }
    }

    pub fn update_connect_page_password_visible(&self, visible: bool) {
        if self
            .tx
            .send(Event::UpdateConnectPagePasswordVisible { visible })
            .is_err()
        {
            tracing::error!("send UpdateConnectPagePasswordVisible event failed");
        }
    }

    pub fn update_connect_page_password_editing(&self, editing: bool) {
        if self
            .tx
            .send(Event::UpdateConnectPagePasswordEditing { editing })
            .is_err()
        {
            tracing::error!("send UpdateConnectPagePasswordEditing event failed");
        }
    }

    pub fn update_connect_page_password(&self, password: &str) {
        if self
            .tx
            .send(Event::UpdateConnectPagePassword {
                password: password.to_string(),
            })
            .is_err()
        {
            tracing::error!("send UpdateConnectPagePassword event failed");
        }
    }

    pub fn update_connect_page_visit_device_id(&self, device_id: &str) {
        if self
            .tx
            .send(Event::UpdateConnectPageVisitDeviceId {
                device_id: device_id.to_string(),
            })
            .is_err()
        {
            tracing::error!("send UpdateConnectPageVisitDeviceId event failed");
        }
    }

    pub fn update_connect_page_desktop_connecting(&self, connecting: bool) {
        if self
            .tx
            .send(Event::UpdateConnectPageDesktopConnecting { connecting })
            .is_err()
        {
            tracing::error!("send UpdateConnectPageDesktopConnecting event failed");
        }
    }

    pub fn update_last_error(&self, error: CoreError) {
        if self.tx.send(Event::UpdateLastError { err: error }).is_err() {
            tracing::error!("send UpdateLastError event failed");
        }
    }
}
