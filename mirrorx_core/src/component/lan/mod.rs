use crate::error::CoreResult;
use futures::{pin_mut, StreamExt};
use mdns::RecordKind;
use serde::Serialize;
use std::{
    net::IpAddr,
    sync::{atomic::Ordering, Arc},
    time::Duration,
};
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
pub struct Node {
    pub host_name: String,
    pub addr: IpAddr,
}

pub struct LanDiscover {
    nodes: Arc<Mutex<Vec<Node>>>,
    should_exit: Arc<std::sync::atomic::AtomicBool>,
}

impl LanDiscover {
    pub fn new() -> CoreResult<LanDiscover> {
        let nodes = Arc::new(Mutex::new(Vec::new()));
        let should_exit = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stream = mdns::discover::all("mirrorx.cloud", Duration::from_secs(10))?.listen();

        let nodes_copy = nodes.clone();
        let should_exit_copy = should_exit.clone();
        tokio::spawn(async move {
            pin_mut!(stream);

            loop {
                if should_exit_copy.load(Ordering::SeqCst) {
                    break;
                }

                if let Some(Ok(response)) = stream.next().await {
                    let mut nodes = Vec::new();
                    for record in response.records() {
                        let addr: Option<IpAddr> = match record.kind {
                            RecordKind::A(addr) => Some(addr.into()),
                            RecordKind::AAAA(addr) => Some(addr.into()),
                            _ => None,
                        };

                        if let Some(addr) = addr {
                            nodes.push(Node {
                                host_name: record.name.clone(),
                                addr,
                            })
                        }
                    }

                    *nodes_copy.lock().await = nodes;
                }
            }

            tracing::info!("lan discover exit");
        });

        Ok(LanDiscover { nodes, should_exit })
    }

    pub async fn nodes_snapshot(&self) -> Vec<Node> {
        (*self.nodes.lock().await).to_vec()
    }
}

impl Drop for LanDiscover {
    fn drop(&mut self) {
        self.should_exit.store(true, Ordering::SeqCst);
    }
}
