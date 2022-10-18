mod event;

use self::event::Event;
use mirrorx_core::{
    api::{
        config::{Config, DomainConfig},
        signaling::{
            KeyExchangeRequest, KeyExchangeResponse, PublishMessage, ResourceType, SignalingClient,
            VisitReplyRequest, VisitResponse,
        },
    },
    core_error,
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
    signaling_key_exchange_response: Option<KeyExchangeResponse>,

    dialog_input_visit_password_visible: Option<(i64, i64)>, // active_device_id,passive_device_id
    dialog_input_visit_password: String,
    dialog_key_exchange_processing_visible: bool,
    dialog_visit_request_visible: Option<(i64, i64, ResourceType)>, // active_device_id, passive_device_id, resource_type

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
            signaling_key_exchange_response: Default::default(),
            dialog_input_visit_password_visible: Default::default(),
            dialog_input_visit_password: Default::default(),
            dialog_key_exchange_processing_visible: Default::default(),
            dialog_visit_request_visible: Default::default(),
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

    pub fn signaling_key_exchange_response(&self) -> Option<&KeyExchangeResponse> {
        self.signaling_key_exchange_response.as_ref()
    }

    pub fn dialog_input_visit_password_visible(&self) -> Option<(i64, i64)> {
        self.dialog_input_visit_password_visible
    }

    pub fn dialog_input_visit_password(&self) -> &str {
        &self.dialog_input_visit_password
    }

    pub fn dialog_key_exchange_processing_visible(&self) -> bool {
        self.dialog_key_exchange_processing_visible
    }

    pub fn dialog_visit_request_visible(&self) -> Option<&(i64, i64, ResourceType)> {
        self.dialog_visit_request_visible.as_ref()
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
                Event::UpdateConfigSuccess { config } => {
                    let mut update_signaling_client = false;
                    if let Some(old_config) = &self.config {
                        if old_config.primary_domain != config.primary_domain {
                            update_signaling_client = true;
                        }
                    } else {
                        // initial config is None, and receive UpdateConfigSuccess means config has initialized,
                        // so that signaling client must to update
                        update_signaling_client = true;
                    }

                    self.config = Some(config);

                    if update_signaling_client
                        && self.tx.send(Event::UpdateSignalingClient).is_err()
                    {
                        tracing::error!("send UpdateSignalingClient event failed");
                    }
                }
                Event::UpdateConfigPath { config_path } => self.config_path = config_path,
                Event::UpdateSignalingClient => {
                    if let Some(config) = &self.config {
                        let domain_config = match config.domain_configs.get(&config.primary_domain)
                        {
                            Some(domain_config) => domain_config,
                            None => return,
                        };

                        update_signaling_client(
                            config.clone(),
                            config.primary_domain.clone(),
                            domain_config.clone(),
                            self.config_path.clone(),
                            self.tx.clone(),
                        );
                    }
                }
                Event::UpdateSignalingClientSuccess { signaling_client } => {
                    self.signaling_client = Some(signaling_client)
                }
                Event::UpdateSignalingPublishMessage { publish_message } => match publish_message {
                    PublishMessage::VisitRequest {
                        active_device_id,
                        passive_device_id,
                        resource_type,
                    } => {
                        if let Some(config) = &self.config {
                            if let Some(domain_config) =
                                config.domain_configs.get(&config.primary_domain)
                            {
                                if domain_config.device_id == passive_device_id {
                                    self.tx.send(Event::UpdateDialogVisitRequestVisible {
                                        visible: Some((
                                            active_device_id,
                                            passive_device_id,
                                            resource_type,
                                        )),
                                    });
                                }
                            }
                        }
                    }
                },
                Event::UpdateSignalingKeyExchangeResponse { resp } => {
                    self.signaling_key_exchange_response = Some(resp)
                }
                Event::UpdateDialogInputVisitPasswordVisible { visible } => {
                    self.dialog_input_visit_password_visible = visible
                }
                Event::UpdateDialogInputVisitPassword { password } => {
                    self.dialog_input_visit_password = password
                }
                Event::UpdateDialogKeyExchangeProcessingVisible { visible } => {
                    self.dialog_key_exchange_processing_visible = visible
                }
                Event::UpdateDialogVisitRequestVisible { visible } => {
                    self.dialog_visit_request_visible = visible
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

                Event::EmitSignalingVisit {
                    local_device_id,
                    remote_device_id,
                    resource_type,
                } => {
                    if let Some(signaling_client) = &self.signaling_client {
                        self.connect_page_desktop_connecting = true;

                        let signaling_client = signaling_client.clone();
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            match signaling_client
                                .visit(mirrorx_core::api::signaling::VisitRequest {
                                    local_device_id,
                                    remote_device_id,
                                    resource_type,
                                })
                                .await
                            {
                                Ok(resp) => {
                                    if resp.allow {
                                        tx.send(Event::UpdateDialogKeyExchangeProcessingVisible {
                                            visible: true,
                                        });
                                    } else {
                                        tx.send(Event::UpdateLastError {
                                            err: core_error!("remote device reject your request"),
                                        });
                                    }
                                }
                                Err(err) => {
                                    tracing::error!(?err, "signaling visit request failed");
                                    tx.send(Event::UpdateConnectPageDesktopConnecting {
                                        connecting: false,
                                    });
                                    tx.send(Event::UpdateLastError { err });
                                }
                            }
                        });
                    }
                }

                Event::EmitSignalingVisitReply {
                    allow,
                    active_device_id,
                    passive_device_id,
                } => {
                    if let Some(signaling_client) = &self.signaling_client {
                        let signaling_client = signaling_client.clone();
                        let tx = self.tx.clone();
                        tokio::spawn(async move {
                            if let Err(err) = signaling_client
                                .visit_reply(VisitReplyRequest {
                                    active_device_id,
                                    passive_device_id,
                                    allow,
                                })
                                .await
                            {
                                tracing::error!(?err, "signaling visit reply failed");
                                tx.send(Event::UpdateLastError {
                                    err: core_error!("reply visit request failed"),
                                });
                            }
                        });
                    } else {
                        self.tx.send(Event::UpdateLastError { err: core_error!(
                            "Signaling connection has broken, please click Domain '🔄' button to reconnect Signaling Server"
                        ) });
                    }
                }
                Event::EmitSignalingKeyExchange {
                    active_device_id,
                    passive_device_id,
                } => {
                    if let Some(signaling_client) = &self.signaling_client {
                        let signaling_client = signaling_client.clone();
                        let tx = self.tx.clone();
                        let password = self.dialog_input_visit_password.clone();
                        tokio::spawn(async move {
                            if let Err(err) = signaling_client
                                .key_exchange(KeyExchangeRequest {
                                    local_device_id: active_device_id,
                                    remote_device_id: passive_device_id,
                                    password,
                                })
                                .await
                            {
                                tracing::error!(?err, "signaling key exchange failed");
                                tx.send(Event::UpdateLastError {
                                    err: core_error!("request key exchange failed"),
                                });
                            }
                        });
                    }
                }
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

    pub fn update_signaling_key_exchange_response(&self, resp: &KeyExchangeResponse) {
        if self
            .tx
            .send(Event::UpdateSignalingKeyExchangeResponse { resp: resp.clone() })
            .is_err()
        {
            tracing::error!("send UpdateSignalingKeyExchangeResponse event failed");
        }
    }

    pub fn update_dialog_input_visit_password_visible(&self, visible: Option<(i64, i64)>) {
        if self
            .tx
            .send(Event::UpdateDialogInputVisitPasswordVisible { visible })
            .is_err()
        {
            tracing::error!("send UpdateDialogInputVisitPasswordVisible event failed");
        }
    }

    pub fn update_dialog_input_visit_password(&self, password: &str) {
        if self
            .tx
            .send(Event::UpdateDialogInputVisitPassword {
                password: password.to_string(),
            })
            .is_err()
        {
            tracing::error!("send UpdateDialogInputVisitPasswordContent event failed");
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

    pub fn update_dialog_visit_request_visible(&self, visible: Option<(i64, i64, ResourceType)>) {
        if self
            .tx
            .send(Event::UpdateDialogVisitRequestVisible { visible })
            .is_err()
        {
            tracing::error!("send UpdateDialogVisitRequestVisible event failed");
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

    pub fn emit_signaling_visit(
        &self,
        local_device_id: i64,
        remote_device_id: i64,
        resource_type: ResourceType,
    ) {
        if self
            .tx
            .send(Event::EmitSignalingVisit {
                local_device_id,
                remote_device_id,
                resource_type,
            })
            .is_err()
        {
            tracing::error!("send EmitSignalingVisit event failed");
        }
    }

    pub fn emit_signaling_visit_reply(
        &self,
        allow: bool,
        active_device_id: i64,
        passive_device_id: i64,
    ) {
        if self
            .tx
            .send(Event::EmitSignalingVisitReply {
                allow,
                active_device_id,
                passive_device_id,
            })
            .is_err()
        {
            tracing::error!("send EmitSignalingVisitReply event failed");
        }
    }

    pub fn emit_signaling_key_exchange(&self, active_device_id: i64, passive_device_id: i64) {
        if self
            .tx
            .send(Event::EmitSignalingKeyExchange {
                active_device_id,
                passive_device_id,
            })
            .is_err()
        {
            tracing::error!("send EmitSignalingKeyExchange event failed");
        }
    }
}

fn update_signaling_client(
    mut config: Config,
    domain: String,
    mut domain_config: DomainConfig,
    config_path: PathBuf,
    tx: UnboundedSender<Event>,
) {
    tokio::spawn(async move {
        let publish_message_tx = tx.clone();
        let publish_message_fn = Box::new(move |message: PublishMessage| {
            tracing::info!("publish message");
            if publish_message_tx
                .send(Event::UpdateSignalingPublishMessage {
                    publish_message: message,
                })
                .is_err()
            {
                tracing::error!("send UpdateSignalingPublishMessage event failed");
            }
        });

        match SignalingClient::new(
            domain.clone(),
            domain_config.clone(),
            config_path,
            publish_message_fn,
        )
        .await
        {
            Ok((signaling_client, device_id)) => {
                if tx
                    .send(Event::UpdateSignalingClientSuccess { signaling_client })
                    .is_err()
                {
                    tracing::error!("send UpdateSignalingClientSuccess event failed");
                }

                domain_config.device_id = device_id;
                config.domain_configs.insert(domain, domain_config);

                if tx.send(Event::UpdateConfig { config }).is_err() {
                    tracing::error!("send UpdateConfig event failed");
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
