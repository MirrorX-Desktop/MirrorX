use super::{app_state_updater::AppStateUpdater, event::Event};
use crate::send_event;
use mirrorx_core::{
    api::{
        config::Config,
        signaling::{
            KeyExchangeRequest, KeyExchangeResponse, PublishMessage, ResourceType, SignalingClient,
            VisitReplyRequest,
        },
    },
    core_error,
    error::CoreError,
};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

static SIGNALING_CONNECTION_BROKEN_ERROR: Lazy<String> = Lazy::new(|| {
    String::from(
        r#"Signaling connection has broken, please click Domain "🔄" button to reconnect Signaling Server"#,
    )
});

pub struct AppState {
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
}

impl AppState {
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
}

impl AppState {
    pub fn new_state_updater(&self) -> AppStateUpdater {
        AppStateUpdater::new(self.tx.clone())
    }

    pub fn handle_event(&mut self) -> Option<CoreError> {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::UpdateCurrentPage { page_name } => self.current_page_name = page_name,
                Event::UpdateConfig { config } => {
                    update_config(self.config_path.clone(), config, self.tx.clone());
                }
                Event::UpdateConfigSuccess { config } => {
                    self.config = Some(config);
                    send_event!(self.tx, Event::UpdateSignalingClient);
                }
                Event::UpdateConfigPath { config_path } => self.config_path = config_path,
                Event::UpdateSignalingClient => {
                    update_signaling_client(
                        self.config.clone(),
                        self.signaling_client.clone(),
                        self.tx.clone(),
                        self.config_path.clone(),
                    );
                }
                Event::UpdateSignalingClientSuccess { signaling_client } => {
                    self.signaling_client = Some(signaling_client)
                }
                Event::UpdateSignalingPublishMessage { publish_message } => {
                    self.handle_signaling_publish_message(publish_message)
                }
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
                Event::UpdateLastError { err } => return Some(err),

                Event::EmitSignalingVisit {
                    local_device_id,
                    remote_device_id,
                    resource_type,
                } => self.emit_signaling_visit(local_device_id, remote_device_id, resource_type),

                Event::EmitSignalingVisitReply {
                    active_device_id,
                    passive_device_id,
                    allow,
                } => self.emit_visit_request_reply(active_device_id, passive_device_id, allow),

                Event::EmitSignalingKeyExchange {
                    active_device_id,
                    passive_device_id,
                } => {
                    self.emit_signaling_key_exchange(active_device_id, passive_device_id);
                }
            }
        }

        None
    }

    fn emit_signaling_visit(
        &mut self,
        local_device_id: i64,
        remote_device_id: i64,
        resource_type: ResourceType,
    ) {
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
                            send_event!(
                                tx,
                                Event::UpdateDialogInputVisitPasswordVisible {
                                    visible: Some((local_device_id, remote_device_id)),
                                }
                            );
                        } else {
                            send_event!(
                                tx,
                                Event::UpdateLastError {
                                    err: core_error!("remote device reject your request"),
                                }
                            );
                            send_event!(
                                tx,
                                Event::UpdateConnectPageDesktopConnecting { connecting: false }
                            );
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "signaling visit request failed");
                        send_event!(
                            tx,
                            Event::UpdateConnectPageDesktopConnecting { connecting: false }
                        );
                        send_event!(tx, Event::UpdateLastError { err });
                    }
                }
            });
        } else {
            send_event!(
                self.tx,
                Event::UpdateLastError {
                    err: core_error!("{}", SIGNALING_CONNECTION_BROKEN_ERROR.as_str())
                }
            );
        }
    }

    fn emit_visit_request_reply(
        &mut self,
        active_device_id: i64,
        passive_device_id: i64,
        allow: bool,
    ) {
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
                    send_event!(
                        tx,
                        Event::UpdateLastError {
                            err: core_error!("reply visit request failed"),
                        }
                    );
                }
            });
        } else {
            send_event!(
                self.tx,
                Event::UpdateLastError {
                    err: core_error!("{}", SIGNALING_CONNECTION_BROKEN_ERROR.as_str())
                }
            );
        }
    }

    fn emit_signaling_key_exchange(&mut self, active_device_id: i64, passive_device_id: i64) {
        if let Some(signaling_client) = &self.signaling_client {
            self.dialog_input_visit_password_visible = None;
            let password = self.dialog_input_visit_password.clone();
            self.dialog_input_visit_password.clear();

            let signaling_client = signaling_client.clone();
            let tx = self.tx.clone();
            tokio::spawn(async move {
                match signaling_client
                    .key_exchange(KeyExchangeRequest {
                        local_device_id: active_device_id,
                        remote_device_id: passive_device_id,
                        password,
                    })
                    .await
                {
                    Ok(resp) => {
                        tracing::info!(?resp, "key exchange finish");
                    }
                    Err(err) => {
                        tracing::error!(?err, "signaling key exchange failed");
                        send_event!(
                            tx,
                            Event::UpdateLastError {
                                err: core_error!("request key exchange failed"),
                            }
                        );
                    }
                }

                send_event!(
                    tx,
                    Event::UpdateConnectPageDesktopConnecting { connecting: false }
                );
            });
        } else {
            send_event!(
                self.tx,
                Event::UpdateLastError {
                    err: core_error!("{}", SIGNALING_CONNECTION_BROKEN_ERROR.as_str())
                }
            );
        }
    }

    fn handle_signaling_publish_message(&mut self, publish_message: PublishMessage) {
        match publish_message {
            PublishMessage::VisitRequest {
                active_device_id,
                passive_device_id,
                resource_type,
            } => {
                if let Some(config) = &self.config {
                    if let Some(domain_config) = config.domain_configs.get(&config.primary_domain) {
                        if domain_config.device_id == passive_device_id {
                            send_event!(
                                self.tx,
                                Event::UpdateDialogVisitRequestVisible {
                                    visible: Some((
                                        active_device_id,
                                        passive_device_id,
                                        resource_type,
                                    )),
                                }
                            );
                        }
                    }
                }
            }
        }
    }
}

fn update_config(config_path: PathBuf, config: Config, tx: UnboundedSender<Event>) {
    tokio::task::spawn_blocking(move || {
        if let Err(err) = mirrorx_core::api::config::save(&config_path, &config) {
            tracing::error!(?err, "config save failed");
            send_event!(tx, Event::UpdateLastError { err });
        } else {
            send_event!(tx, Event::UpdateConfigSuccess { config });
        }
    });
}

fn update_signaling_client(
    config: Option<Config>,
    signaling_client: Option<SignalingClient>,
    tx: UnboundedSender<Event>,
    config_path: PathBuf,
) {
    if let Some(mut config) = config {
        tokio::spawn(async move {
            let domain_config = match config.domain_configs.get_mut(&config.primary_domain) {
                Some(domain_config) => domain_config,
                None => return,
            };

            if let Some(signaling_client) = signaling_client {
                if signaling_client.domain() == config.primary_domain {
                    return;
                }
            }

            let publish_message_tx = tx.clone();
            let publish_message_fn = Box::new(move |message: PublishMessage| {
                send_event!(
                    publish_message_tx,
                    Event::UpdateSignalingPublishMessage {
                        publish_message: message,
                    }
                );
            });

            match SignalingClient::new(
                config.primary_domain.clone(),
                domain_config.clone(),
                config_path,
                publish_message_fn,
            )
            .await
            {
                Ok((signaling_client, device_id)) => {
                    domain_config.device_id = device_id;
                    send_event!(tx, Event::UpdateConfig { config });
                    send_event!(tx, Event::UpdateSignalingClientSuccess { signaling_client });
                }
                Err(err) => {
                    send_event!(tx, Event::UpdateLastError { err });
                }
            }
        });
    } else {
        send_event!(
            tx,
            Event::UpdateLastError {
                err: core_error!("signaling client update while config is empty!")
            }
        );
    }
}
