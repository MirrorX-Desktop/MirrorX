use crate::error::CoreResult;
use hostname;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    time::Duration,
};

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub host_name: String,
    pub addr: Ipv4Addr,
    pub os: String,
    pub os_version: String,
    pub tcp_port: u16,
    pub udp_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BroadcastPacket {
    TargetLive(TargetLivePacket),
    TargetDead,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TargetLivePacket {
    host_name: String,
    os: String,
    os_version: String,
    tcp_port: u16,
    udp_port: u16,
}

pub struct LanDiscover {
    cache: Cache<Ipv4Addr, Node>,
    write_exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
    read_exit_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl LanDiscover {
    pub async fn new(tcp_port: u16, udp_port: u16) -> CoreResult<Self> {
        let stream = tokio::net::UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await?;
        stream.set_broadcast(true)?;

        let dead_packet = bincode::serialize(&BroadcastPacket::TargetDead)?;
        let live_packet = bincode::serialize(&BroadcastPacket::TargetLive(
            gen_target_live_packet(tcp_port, udp_port)?,
        ))?;

        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(29))
            .build();

        let writer = Arc::new(stream);
        let reader = writer.clone();

        let (write_exit_tx, mut write_exit_rx) = tokio::sync::oneshot::channel();
        let (read_exit_tx, mut read_exit_rx) = tokio::sync::oneshot::channel();

        let cache_copy = cache.clone();
        tokio::spawn(async move {
            let mut buffer = [0u8; 256];

            loop {
                let Err(tokio::sync::oneshot::error::TryRecvError::Empty) = read_exit_rx.try_recv() else {
                    tracing::info!("udp broadcast read loop exit");
                    return;
                };

                let (buffer_len, from_addr) = match reader.recv_from(&mut buffer).await {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(?err, "udp broadcast recv failed");
                        continue;
                    }
                };

                let IpAddr::V4(target_addr) = from_addr.ip() else {
                    continue;
                };

                let packet = match bincode::deserialize::<BroadcastPacket>(&buffer[..buffer_len]) {
                    Ok(v) => v,
                    Err(err) => {
                        tracing::error!(
                            ?err,
                            ?from_addr,
                            "deserialize udp broadcast packet failed"
                        );
                        continue;
                    }
                };

                match packet {
                    BroadcastPacket::TargetLive(live_packet) => {
                        cache_copy
                            .insert(
                                target_addr,
                                Node {
                                    host_name: live_packet.host_name.to_string(),
                                    addr: target_addr,
                                    os: live_packet.os.to_string(),
                                    os_version: live_packet.os_version.to_string(),
                                    tcp_port: live_packet.tcp_port,
                                    udp_port: live_packet.udp_port,
                                },
                            )
                            .await;
                    }
                    BroadcastPacket::TargetDead => cache_copy.invalidate(&target_addr).await,
                }
            }
        });

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(11));

            loop {
                tokio::select! {
                    _ = ticker.tick() => (),
                    _ = &mut write_exit_rx => {
                        let _ = writer.send(&dead_packet).await;
                        tracing::info!("udp broadcast write loop exit");
                        return;
                    }
                };

                if let Err(err) = writer.send(&live_packet).await {
                    tracing::warn!(?err, "udp broadcast send failed");
                }
            }
        });

        Ok(Self {
            cache,
            write_exit_tx: Some(write_exit_tx),
            read_exit_tx: Some(read_exit_tx),
        })
    }

    pub fn nodes_snapshot(&self) -> Vec<Node> {
        self.cache.iter().map(|(_, node)| node).collect()
    }
}

impl Drop for LanDiscover {
    fn drop(&mut self) {
        if let Some(tx) = self.write_exit_tx.take() {
            let _ = tx.send(());
        }

        if let Some(tx) = self.read_exit_tx.take() {
            let _ = tx.send(());
        }
    }
}

fn gen_target_live_packet(tcp_port: u16, udp_port: u16) -> CoreResult<TargetLivePacket> {
    let host_name = convert_host_name_to_string(&hostname::get()?)?;
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
        host_name,
        os,
        os_version,
        tcp_port,
        udp_port,
    })
}

fn convert_host_name_to_string(v: &OsStr) -> CoreResult<String> {
    #[cfg(target_os = "windows")]
    {
        use crate::core_error;
        use std::os::windows::ffi::OsStrExt;

        let result: Vec<u16> = v.encode_wide().collect();
        String::from_utf16(&result)
            .map_err(|err| core_error!("convert host name to string failed ({:?})", err))
    }

    #[cfg(not(target_os = "windows"))]
    Ok(v.to_string_lossy().to_string())
}
