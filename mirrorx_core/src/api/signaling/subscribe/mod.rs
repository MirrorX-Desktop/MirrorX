mod key_exchange;

use crate::utility::runtime::TOKIO_RUNTIME;
use signaling_proto::message::{publish_message::InnerPublishMessage, ResourceType};
use std::{path::PathBuf, time::Duration};
use tokio::{
    select,
    sync::mpsc::{error::TryRecvError, Receiver},
};
use tonic::transport::Channel;

#[derive(Debug)]
pub enum PublishMessage {
    VisitRequest {
        active_device_id: i64,
        passive_device_id: i64,
        resource_type: crate::api::signaling::visit::ResourceType,
    },
}

pub async fn subscribe(
    client: &mut signaling_proto::service::signaling_client::SignalingClient<Channel>,
    domain: String,
    device_id: i64,
    device_finger_print: String,
    config_path: PathBuf,
    publish_message_fn: Box<dyn Fn(PublishMessage) + Send>,
    mut exit_tx: Receiver<()>,
) {
    let mut subscribe_client = client.clone();
    TOKIO_RUNTIME.spawn(async move {
        loop {
            match exit_tx.try_recv() {
                Ok(_) => return,
                Err(err) => {
                    if let TryRecvError::Disconnected = err {
                        return;
                    }
                }
            }

            let mut server_stream = match subscribe_client
                .subscribe(signaling_proto::message::SubscribeRequest {
                    device_id,
                    device_finger_print: device_finger_print.clone(),
                })
                .await
            {
                Ok(stream) => stream.into_inner(),
                Err(err) => {
                    tracing::error!(?err, "subscribe stream failed, try again after 1s");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            };

            loop {
                let publish_message = select! {
                    biased;
                    _ = exit_tx.recv() => return,
                    publish_message = server_stream.message() => publish_message,
                };

                let message = match publish_message {
                    Ok(message) => {
                        if let Some(message) = message {
                            match message.inner_publish_message {
                                Some(message) => message,
                                None => {
                                    tracing::warn!("publish message inner is None");
                                    continue;
                                }
                            }
                        } else {
                            tracing::error!(
                                "subscribe server stream disconnect, reconnect after 1s"
                            );
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::error!(?err, "subscribe server stream received an error");
                        continue;
                    }
                };

                match message {
                    InnerPublishMessage::VisitRequest(visit_request) => {
                        let resource_type =
                            match ResourceType::from_i32(visit_request.resource_type) {
                                Some(typ) => match typ {
                                    ResourceType::Desktop => {
                                        crate::api::signaling::visit::ResourceType::Desktop
                                    }
                                    ResourceType::Files => {
                                        crate::api::signaling::visit::ResourceType::Files
                                    }
                                },
                                None => {
                                    tracing::warn!("remote device require unknown resource type");
                                    continue;
                                }
                            };

                        let publish_message = PublishMessage::VisitRequest {
                            active_device_id: visit_request.active_device_id,
                            passive_device_id: visit_request.passive_device_id,
                            resource_type,
                        };

                        (publish_message_fn)(publish_message);
                    }
                    InnerPublishMessage::KeyExchangeRequest(key_exchange_request) => {
                        let mut client = subscribe_client.clone();
                        let domain = domain.clone();
                        let config_path = config_path.clone();
                        TOKIO_RUNTIME.spawn(async move {
                            key_exchange::handle(
                                &mut client,
                                domain,
                                config_path,
                                &key_exchange_request,
                            )
                            .await
                        });
                    }
                }
            }
        }
    });
}
