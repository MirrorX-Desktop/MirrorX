use crate::error::CoreResult;
use serde::{Deserialize, Serialize};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum BroadcastPacket {
    TargetLive(TargetLivePacket),
    TargetDead(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TargetLivePacket {
    pub hostname: String,
    pub os: String,
    pub os_version: String,
}

pub struct Discover {
    write_exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
    read_exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl Discover {
    pub async fn new(
        hostname: &str,
        interface_name: &str,
        ip: IpAddr,
        discoverable: Arc<AtomicBool>,
        packet_tx: tokio::sync::mpsc::Sender<(SocketAddr, BroadcastPacket)>,
    ) -> CoreResult<Self> {
        let stream = tokio::net::UdpSocket::bind((ip, 48000)).await?;
        stream.set_broadcast(true)?;

        tracing::info!(interface = interface_name, ?ip, "lan discover listen");

        let dead_packet = bincode::serialize(&BroadcastPacket::TargetDead(hostname.to_string()))?;
        let live_packet =
            bincode::serialize(&BroadcastPacket::TargetLive(create_live_packet(hostname)?))?;

        let writer = Arc::new(stream);
        let reader = writer.clone();

        let (write_exit_tx, mut write_exit_rx) = tokio::sync::oneshot::channel();
        let (read_exit_tx, mut read_exit_rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let mut buffer = [0u8; 512];

            loop {
                let Err(tokio::sync::oneshot::error::TryRecvError::Empty) = read_exit_rx.try_recv() else {
                    tracing::info!("lan discover broadcast recv loop exit");
                    return;
                };

                let (buffer_len, target_addr) = match reader.recv_from(&mut buffer).await {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(?err, "lan discover broadcast packet recv failed");
                        continue;
                    }
                };

                let packet = match bincode::deserialize::<BroadcastPacket>(&buffer[..buffer_len]) {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(
                            ?err,
                            ?target_addr,
                            "deserialize lan discover broadcast packet failed"
                        );
                        continue;
                    }
                };

                let _ = packet_tx.send((target_addr, packet)).await;
            }
        });

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(11));

            loop {
                tokio::select! {
                    _ = ticker.tick() => (),
                    _ = &mut write_exit_rx => {
                        let _ = writer.send(&dead_packet).await;
                        tracing::info!("lan discover broadcast loop exit");
                        return;
                    }
                };

                if !discoverable.load(Ordering::SeqCst) {
                    continue;
                }

                if let Err(err) = writer
                    .send_to(&live_packet, (Ipv4Addr::BROADCAST, 48000))
                    .await
                {
                    tracing::warn!(?err, "lan discover broadcast failed");
                }
            }
        });

        Ok(Self {
            write_exit_tx: Some(write_exit_tx),
            read_exit_tx: Some(read_exit_tx),
        })
    }
}

impl Drop for Discover {
    fn drop(&mut self) {
        if let Some(tx) = self.write_exit_tx.take() {
            let _ = tx.send(());
        }

        if let Some(tx) = self.read_exit_tx.take() {
            let _ = tx.send(());
        }
    }
}

fn create_live_packet(hostname: &str) -> CoreResult<TargetLivePacket> {
    let os_info = os_info::get();
    let os_version = os_info.version().to_string();
    let os = match os_info.os_type() {
        os_info::Type::Linux
        | os_info::Type::Alpine
        | os_info::Type::Arch
        | os_info::Type::Debian
        | os_info::Type::EndeavourOS
        | os_info::Type::Garuda
        | os_info::Type::Gentoo
        | os_info::Type::Manjaro
        | os_info::Type::Mariner
        | os_info::Type::Mint
        | os_info::Type::NixOS
        | os_info::Type::OracleLinux
        | os_info::Type::Pop
        | os_info::Type::Raspbian
        | os_info::Type::Solus => "Linux",

        os_info::Type::HardenedBSD
        | os_info::Type::MidnightBSD
        | os_info::Type::NetBSD
        | os_info::Type::OpenBSD
        | os_info::Type::DragonFly => "BSD",

        os_info::Type::Unknown
        | os_info::Type::Emscripten
        | os_info::Type::Redox
        | os_info::Type::Illumos => "Unknown",

        os_info::Type::Amazon => "Amazon",
        os_info::Type::FreeBSD => "FreeBSD",
        os_info::Type::Android => "Android",
        os_info::Type::CentOS => "CentOS",
        os_info::Type::Fedora => "Fedora",
        os_info::Type::Macos => "macOS",
        os_info::Type::openSUSE => "openSUSE",
        os_info::Type::Redhat => "Redhat",
        os_info::Type::RedHatEnterprise => "Redhat Enterprise",
        os_info::Type::SUSE => "SUSE",
        os_info::Type::Ubuntu => "Ubuntu",
        os_info::Type::Windows => "Windows",

        _ => "Unknown",
    }
    .to_string();

    Ok(TargetLivePacket {
        hostname: hostname.to_string(),
        os,
        os_version,
    })
}
