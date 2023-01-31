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
    pub display_name: String,
    pub addrs: FxHashMap<IpAddr, i64>,
    pub os: String,
    pub os_version: String,
}

pub struct LANProvider {
    nodes_cache: Arc<RwLock<FxHashMap<String, Node>>>,
    discoverable: Arc<AtomicBool>,
    _discovers: Vec<discover::Discover>,
    _server: server::Server,
}

impl LANProvider {
    pub async fn new() -> CoreResult<Self> {
        let hostname = format!("{}.mirrorx.lan", get_hostname()?);
        let mut discovers = Vec::new();
        let discoverable = Arc::new(AtomicBool::new(true));
        let (packet_tx, packet_rx) = tokio::sync::mpsc::channel(64);

        if cfg!(target_os = "windows") {
            let broadcast_interfaces = enum_broadcast_network_interfaces()?;
            for (interface_name, ip) in broadcast_interfaces {
                discovers.push(
                    discover::Discover::new(
                        &hostname,
                        &interface_name,
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
                    &hostname,
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

        serve_discover_nodes(hostname, nodes_cache.clone(), packet_rx);

        Ok(LANProvider {
            nodes_cache,
            discoverable,
            _discovers: discovers,
            _server: server,
        })
    }

    pub async fn nodes(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = (*self.nodes_cache.read().await).values().cloned().collect();
        nodes.sort_by(|a, b| b.display_name.cmp(&a.display_name));
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
    self_hostname: String,
    nodes_cache: Arc<RwLock<FxHashMap<String, Node>>>,
    mut packet_rx: Receiver<(SocketAddr, discover::BroadcastPacket)>,
) {
    let mut ticker = tokio::time::interval(Duration::from_secs(10));
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = ticker.tick() => clear_timeout_nodes(nodes_cache.clone()).await,
                packet = packet_rx.recv() => match packet {
                    Some(packet) => update_nodes(&self_hostname, nodes_cache.clone(), packet).await,
                    None => {
                        tracing::error!("lan discover packet channel closed");
                        return;
                    }
                }
            }
        }
    });
}

async fn clear_timeout_nodes(nodes_cache: Arc<RwLock<FxHashMap<String, Node>>>) {
    let mut nodes = nodes_cache.write().await;
    let now_ts = chrono::Utc::now().timestamp();

    // remove live timeout node
    (*nodes).iter_mut().for_each(|(_, node)| {
        node.addrs.retain(|_, ts| now_ts - *ts <= 30);
    });

    (*nodes).retain(|_, node| !node.addrs.is_empty());
}

async fn update_nodes(
    self_hostname: &str,
    nodes_cache: Arc<RwLock<FxHashMap<String, Node>>>,
    packet: (SocketAddr, BroadcastPacket),
) {
    let (addr, packet) = packet;
    match packet {
        BroadcastPacket::TargetLive(live_packet) => {
            if live_packet.hostname == self_hostname {
                return;
            }

            let mut nodes = nodes_cache.write().await;
            if let Some(node) = (*nodes).get_mut(&live_packet.hostname) {
                if let Some(ts) = node.addrs.get_mut(&addr.ip()) {
                    (*ts) = chrono::Utc::now().timestamp();
                } else {
                    node.addrs.insert(addr.ip(), chrono::Utc::now().timestamp());
                }
            } else {
                let display_name = live_packet
                    .hostname
                    .trim_end_matches(".mirrorx.lan")
                    .to_string();

                let mut addrs = FxHashMap::default();
                addrs.insert(addr.ip(), chrono::Utc::now().timestamp());

                (*nodes).insert(
                    live_packet.hostname,
                    Node {
                        display_name,
                        addrs,
                        os: live_packet.os,
                        os_version: live_packet.os_version,
                    },
                );
            }
        }
        BroadcastPacket::TargetDead(hostname) => {
            if hostname == self_hostname {
                return;
            }

            let mut nodes = nodes_cache.write().await;
            if let Some(node) = (*nodes).get_mut(&hostname) {
                node.addrs.remove(&addr.ip());
                if node.addrs.is_empty() {
                    (*nodes).remove(&hostname);
                }
            }
        }
    }
}

fn get_hostname() -> CoreResult<String> {
    let hostname = hostname::get()?;

    #[cfg(target_os = "windows")]
    {
        use crate::core_error;
        use std::os::windows::ffi::OsStrExt;

        let result: Vec<u16> = hostname.encode_wide().collect();
        String::from_utf16(&result)
            .map_err(|err| core_error!("convert host name to string failed ({:?})", err))
    }

    #[cfg(not(target_os = "windows"))]
    Ok(hostname.to_string_lossy().to_string())
}
