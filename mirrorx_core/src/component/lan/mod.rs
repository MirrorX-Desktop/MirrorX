use crate::error::CoreResult;
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;
use std::{collections::HashMap, ffi::OsStr, net::Ipv4Addr, sync::Arc};
use tokio::sync::Mutex;

const LOCAL_DOMAIN: &str = "_mirrorx._udp.local.";

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    pub host_name: String,
    pub addr: Ipv4Addr,
    pub os: String,
    pub os_version: String,
    pub tcp_port: u16,
    pub udp_port: u16,
}

pub struct LanDiscover {
    name: String,
    service_daemon: ServiceDaemon,
    nodes: Arc<Mutex<HashMap<String, Node>>>,
}

impl LanDiscover {
    pub fn new(local_tcp_port: u16, local_udp_port: u16) -> CoreResult<LanDiscover> {
        let local_ip = local_ip()?;
        let host_name = convert_host_name_to_string(&hostname::get()?)?;
        let nodes = Arc::new(Mutex::new(HashMap::new()));
        let service_daemon = ServiceDaemon::new()?;
        let name: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

        let os_info = os_info::get();
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
        };

        let mut properties = HashMap::new();
        properties.insert("os".to_string(), os.to_string());
        properties.insert("os_version".to_string(), os_info.version().to_string());
        properties.insert("tcp_port".to_string(), local_tcp_port.to_string());
        properties.insert("udp_port".to_string(), local_udp_port.to_string());

        let service = ServiceInfo::new(
            LOCAL_DOMAIN,
            &name,
            &host_name,
            local_ip.to_string().as_str(),
            u16::MAX,
            Some(properties),
        )?;

        let self_full_name = service.get_fullname().to_string();

        service_daemon.register(service)?;

        let rx = service_daemon.browse(LOCAL_DOMAIN)?;
        let nodes_copy = nodes.clone();
        tokio::spawn(async move {
            loop {
                match rx.recv_async().await {
                    Ok(event) => match event {
                        mdns_sd::ServiceEvent::SearchStarted(service_type) => {
                            tracing::debug!(?service_type, "mdns searching started")
                        }
                        mdns_sd::ServiceEvent::ServiceFound(service_type, service_full_name) => {
                            tracing::debug!(?service_type, ?service_full_name, "mdns service found")
                        }
                        mdns_sd::ServiceEvent::ServiceResolved(service) => {
                            tracing::debug!("mdns service resolved");

                            if service.get_fullname() == self_full_name {
                                continue;
                            }

                            let mut nodes = nodes_copy.lock().await;

                            let os = service
                                .get_properties()
                                .get("os")
                                .map(|v| v.to_owned())
                                .unwrap_or_default();

                            let os_version = service
                                .get_properties()
                                .get("os_version")
                                .map(|v| v.to_owned())
                                .unwrap_or_default();

                            let Ok(tcp_port) = service
                                .get_properties()
                                .get("tcp_port")
                                .map(|v| v.to_owned())
                                .unwrap_or_default().parse::<u16>() else{
                                    continue;
                                };

                            let Ok(udp_port) = service
                                .get_properties()
                                .get("udp_port")
                                .map(|v| v.to_owned())
                                .unwrap_or_default().parse::<u16>() else{
                                    continue;
                                };

                            if let Some(addr) = service.get_addresses().iter().next() {
                                nodes.insert(
                                    service.get_fullname().to_string(),
                                    Node {
                                        host_name: service.get_hostname().to_string(),
                                        addr: *addr,
                                        os,
                                        os_version,
                                        tcp_port,
                                        udp_port,
                                    },
                                );
                            }
                        }
                        mdns_sd::ServiceEvent::ServiceRemoved(service_type, service_full_name) => {
                            tracing::debug!(
                                ?service_type,
                                ?service_full_name,
                                "mdns service removed"
                            );
                            nodes_copy.lock().await.remove(&service_full_name);
                        }
                        mdns_sd::ServiceEvent::SearchStopped(service_type) => {
                            tracing::debug!(?service_type, "mdns searching stopped")
                        }
                    },
                    Err(_) => {
                        tracing::info!("mdns browse channel closed");
                        return;
                    }
                }
            }
        });

        Ok(LanDiscover {
            name,
            service_daemon,
            nodes,
        })
    }

    pub async fn nodes_snapshot(&self) -> Vec<Node> {
        (*self.nodes.lock().await)
            .values()
            .map(|v| v.to_owned())
            .collect()
    }
}

impl Drop for LanDiscover {
    fn drop(&mut self) {
        let _ = self
            .service_daemon
            .unregister(&format!("{}.{}", self.name, LOCAL_DOMAIN));
        let _ = self.service_daemon.stop_browse(LOCAL_DOMAIN);
        let _ = self.service_daemon.shutdown();
    }
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
