use super::{handler::handle_passive_visit_request, message::*};
use crate::{
    component::config::ConfigStorage,
    core_error,
    error::{CoreError, CoreResult},
    utility::bincode::{bincode_deserialize, bincode_serialize},
};
use quinn::VarInt;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Default)]
pub struct Client {
    endpoint: Option<quinn::Endpoint>,
    conn: Option<quinn::Connection>,
}

impl Client {
    pub async fn switch_domain<F>(
        &mut self,
        addr: SocketAddr,
        server_name: &str,
        storage: ConfigStorage,
        visit_callback: F,
    ) -> CoreResult<()>
    where
        F: Send + Sync + Clone + 'static + Fn(i64, i64, bool) -> bool,
    {
        if let (Some(ref old_endpoint), Some(old_conn)) = (self.endpoint.take(), self.conn.take()) {
            old_conn.close(VarInt::from_u32(0), b"done");
            old_endpoint.close(VarInt::from_u32(0), b"done");
            old_endpoint.wait_idle().await;
        }

        let endpoint = quinn::Endpoint::client((Ipv4Addr::UNSPECIFIED, 0).into())?;
        let conn = endpoint.connect(addr, server_name)?.await?;

        self.endpoint = Some(endpoint);
        self.conn = Some(conn.clone());

        Self::accept_server_call(conn, storage, visit_callback);

        Ok(())
    }

    pub async fn get_server_config(&self) -> CoreResult<ServerConfigReply> {
        let request_bytes = bincode_serialize(&PortalClientMessage::ServerConfigRequest)?;
        let reply_bytes = self.call(&request_bytes).await?;
        let reply_message: PortalServerMessage = bincode_deserialize(&reply_bytes)?;
        let PortalServerMessage::ServerConfigReply(server_config) = reply_message else {
            return Err(CoreError::PortalInvalidReplyMessageError(String::from("PortalServerMessage::ServerConfigReply")));
        };
        Ok(server_config)
    }

    pub async fn client_register(
        &self,
        device_id: i64,
        device_finger_print: &str,
    ) -> CoreResult<ClientRegisterReply> {
        let request_bytes = bincode_serialize(&PortalClientMessage::ClientRegisterRequest(
            ClientRegisterRequest {
                device_id,
                device_finger_print: device_finger_print.into(),
            },
        ))?;
        let reply_bytes = self.call(&request_bytes).await?;
        let reply_message: PortalServerMessage = bincode_deserialize(&reply_bytes)?;
        let PortalServerMessage::ClientRegisterReply(result) = reply_message else {
            return Err(CoreError::PortalInvalidReplyMessageError(String::from("PortalServerMessage::ClientRegisterReply")));
        };
        Ok(result)
    }

    pub async fn check_remote_device_is_online(&self, device_id: i64) -> CoreResult<bool> {
        let request_bytes =
            bincode_serialize(&PortalClientMessage::CheckRemoteDeviceIsOnlineRequest(
                CheckRemoteDeviceIsOnlineRequest { device_id },
            ))?;
        let reply_bytes = self.call(&request_bytes).await?;
        let reply_message: PortalServerMessage = bincode_deserialize(&reply_bytes)?;
        let PortalServerMessage::CheckRemoteDeviceIsOnlineReply(is_online) = reply_message else {
            return Err(CoreError::PortalInvalidReplyMessageError(String::from("PortalServerMessage::CheckRemoteDeviceIsOnlineReply")));
        };
        Ok(is_online)
    }

    pub async fn visit(
        &self,
        active_device_id: i64,
        passive_device_id: i64,
        visit_desktop: bool,
        password_salt: Vec<u8>,
        secret: Vec<u8>,
        secret_nonce: Vec<u8>,
    ) -> CoreResult<ActiveVisitReply> {
        let request_bytes = bincode_serialize(&PortalClientMessage::ActiveVisitRequest(
            ActiveVisitRequest {
                active_device_id,
                passive_device_id,
                visit_desktop,
                password_salt,
                secret,
                secret_nonce,
            },
        ))?;
        let reply_bytes = self.call(&request_bytes).await?;
        let reply_message: PortalServerMessage = bincode_deserialize(&reply_bytes)?;
        match reply_message {
            PortalServerMessage::Error(err) => Err(CoreError::PortalCallError(err)),
            PortalServerMessage::ActiveVisitReply(reply) => Ok(reply),
            _ => Err(CoreError::PortalInvalidReplyMessageError(String::from(
                "PortalServerMessage::VisitPassiveReply",
            ))),
        }
    }

    async fn call(&self, request_bytes: &[u8]) -> CoreResult<Vec<u8>> {
        let Some(ref conn) = self.conn else {
            return Err(core_error!("portal client haven't bind to domain"));
        };

        let (mut tx, rx) = conn.open_bi().await?;
        tx.write_all(request_bytes).await?;
        tx.finish().await?;

        let reply_bytes = rx.read_to_end(2048).await?;
        Ok(reply_bytes)
    }

    fn accept_server_call<F>(conn: quinn::Connection, storage: ConfigStorage, visit_callback: F)
    where
        F: Send + Sync + Clone + 'static + Fn(i64, i64, bool) -> bool,
    {
        tokio::spawn(async move {
            loop {
                match conn.accept_bi().await {
                    Ok((tx, rx)) => {
                        Self::handle_server_call(tx, rx, storage.clone(), visit_callback.clone())
                            .await;
                    }
                    Err(err) => match err {
                        quinn::ConnectionError::VersionMismatch
                        | quinn::ConnectionError::TimedOut
                        | quinn::ConnectionError::TransportError(_) => continue,

                        quinn::ConnectionError::ConnectionClosed(_)
                        | quinn::ConnectionError::ApplicationClosed(_)
                        | quinn::ConnectionError::Reset
                        | quinn::ConnectionError::LocallyClosed => {
                            tracing::info!("[Portal] server call accept loop exit because peer closed or reset");
                            return;
                        }
                    },
                }
            }
        });
    }

    async fn handle_server_call<F>(
        mut tx: quinn::SendStream,
        rx: quinn::RecvStream,
        storage: ConfigStorage,
        visit_callback: F,
    ) where
        F: Send + Sync + 'static + Fn(i64, i64, bool) -> bool,
    {
        let request_bytes = match rx.read_to_end(2048).await {
            Ok(request_bytes) => request_bytes,
            Err(err) => {
                tracing::error!(?err, "read stream failed");
                return;
            }
        };

        let request_message: PortalServerMessage = match bincode_deserialize(&request_bytes) {
            Ok(request_message) => request_message,
            Err(err) => {
                tracing::error!(?err, "deserialize portal server message failed");
                return;
            }
        };

        let client_message = match request_message {
            PortalServerMessage::VisitPassiveRequest(req) => {
                let visit_credentials = uuid::Uuid::new_v4().to_string();

                if visit_callback(
                    req.active_visit_req.active_device_id,
                    req.active_visit_req.passive_device_id,
                    req.active_visit_req.visit_desktop,
                ) {
                    handle_passive_visit_request(storage, req, visit_credentials.clone()).await
                } else {
                    PortalClientMessage::Error(PortalError::Refuse)
                }
            }
            _ => {
                let _ = tx.finish().await;
                return;
            }
        };

        let reply_bytes = match bincode_serialize(&client_message) {
            Ok(reply_bytes) => reply_bytes,
            Err(err) => {
                tracing::error!(?err, "serialize portal client message failed");
                return;
            }
        };

        if let Err(err) = tx.write_all(&reply_bytes).await {
            tracing::error!(?err, "portal reply visit passive result failed");
        }

        let _ = tx.finish().await;
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let (Some(ref endpoint), Some(conn)) = (self.endpoint.take(), self.conn.take()) {
            conn.close(VarInt::from_u32(0), b"done");
            endpoint.close(VarInt::from_u32(0), b"done");
        }
    }
}
