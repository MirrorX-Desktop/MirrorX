mod discover;
mod server;

use self::discover::BroadcastPacket;
use crate::{error::CoreResult, utility::os::enum_broadcast_network_interfaces};
use fxhash::FxHashMap;
use serde::Serialize;
use std::{
    net::{IpAddr, SocketAddr},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::{mpsc::Receiver, RwLock};

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub host_name: String,
    pub addrs: Vec<IpAddr>,
    pub os: String,
    pub os_version: String,
}

pub struct LANProvider {
    nodes_cache: Arc<RwLock<FxHashMap<String, (Node, i64)>>>,
    discoverable: Arc<AtomicBool>,
    _discovers: Vec<discover::Discover>,
    _server: server::Server,
}

impl LANProvider {
    pub async fn new() -> CoreResult<Self> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let mut discovers = Vec::new();
        let discoverable = Arc::new(AtomicBool::new(true));
        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(64);

        if cfg!(target_os = "windows") {
            let broadcast_interfaces = enum_broadcast_network_interfaces()?;
            for (name, ip) in broadcast_interfaces {
                discovers.push(
                    discover::Discover::new(
                        &uuid,
                        &name,
                        ip,
                        discoverable.clone(),
                        packet_tx.clone(),
                    )
                    .await?,
                );
            }
        } else {
            discovers.push(
                discover::Discover::new(
                    &uuid,
                    "default",
                    IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
                    discoverable.clone(),
                    packet_tx.clone(),
                )
                .await?,
            );
        }

        let server = server::Server::new().await?;
        let nodes_cache = Arc::new(RwLock::new(FxHashMap::default()));

        serve_discover_nodes(uuid, nodes_cache.clone(), packet_rx);

        Ok(LANProvider {
            nodes_cache,
            discoverable,
            _discovers: discovers,
            _server: server,
        })
    }

    pub async fn nodes(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = (*self.nodes_cache.read().await)
            .values()
            .cloned()
            .map(|(node, _)| node)
            .collect();
        nodes.sort_by(|a, b| b.host_name.cmp(&a.host_name));
        nodes
    }

    pub fn discoverable(&self) -> bool {
        self.discoverable.load(Ordering::SeqCst)
    }

    pub fn set_discoverable(&self, discoverable: bool) {
        self.discoverable.store(discoverable, Ordering::SeqCst)
    }
}

fn serve_discover_nodes(
    self_id: String,
    nodes_cache: Arc<RwLock<FxHashMap<String, (Node, i64)>>>,
    mut packet_rx: Receiver<(SocketAddr, discover::BroadcastPacket)>,
) {
    let mut ticker = tokio::time::interval(Duration::from_secs(10));
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = ticker.tick() => clear_timeout_nodes(nodes_cache.clone()).await,
                packet = packet_rx.recv() => match packet {
                    Some(packet) => update_nodes(&self_id, nodes_cache.clone(), packet).await,
                    None => {
                        tracing::error!("lan discover packet channel closed");
                        return;
                    }
                }
            }
        }
    });
}

async fn clear_timeout_nodes(nodes_cache: Arc<RwLock<FxHashMap<String, (Node, i64)>>>) {
    let mut nodes = nodes_cache.write().await;
    let now_ts = chrono::Utc::now().timestamp();

    // remove live timeout node
    (*nodes).retain(|_, (_, ts)| now_ts - *ts <= 30);
}

async fn update_nodes(
    self_id: &str,
    nodes_cache: Arc<RwLock<FxHashMap<String, (Node, i64)>>>,
    packet: (SocketAddr, BroadcastPacket),
) {
    let (addr, packet) = packet;
    match packet {
        BroadcastPacket::TargetLive(live_packet) => {
            if live_packet.uuid == self_id {
                return;
            }

            let mut nodes = nodes_cache.write().await;
            if let Some((node, ts)) = (*nodes).get_mut(&live_packet.uuid) {
                if !node.addrs.contains(&addr.ip()) {
                    node.addrs.push(addr.ip())
                }
                *ts = chrono::Utc::now().timestamp();
            } else {
                (*nodes).insert(
                    live_packet.uuid,
                    (
                        Node {
                            host_name: live_packet.host_name,
                            addrs: vec![addr.ip()],
                            os: live_packet.os,
                            os_version: live_packet.os_version,
                        },
                        chrono::Utc::now().timestamp(),
                    ),
                );
            }
        }
        BroadcastPacket::TargetDead(uuid) => {
            if uuid == self_id {
                return;
            }

            let mut nodes = nodes_cache.write().await;
            (*nodes).remove(&uuid);
        }
    }
}
