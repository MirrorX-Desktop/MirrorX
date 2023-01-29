use crate::error::CoreResult;
use network_interface::NetworkInterfaceConfig;
use serde::Serialize;
use std::net::IpAddr;

#[derive(Debug, Serialize)]
pub struct GraphicsCards {
    name: String,
    is_default: bool,
}

pub fn enum_graphics_cards() -> CoreResult<Vec<GraphicsCards>> {
    let mut graphics_cards = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let default_device = metal::Device::system_default();
        let default_device_name =
            default_device.map_or(String::default(), |device| device.name().to_string());

        let devices = metal::Device::all();
        for device in devices {
            let device_name = device.name().to_string();
            let is_default = device_name == default_device_name;
            graphics_cards.push(GraphicsCards {
                name: device_name,
                is_default,
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        use crate::core_error;

        #[derive(serde::Deserialize, Debug)]
        #[serde(rename(deserialize = "Win32_VideoController"))]
        struct VideoControllerInfo {
            #[serde(rename(deserialize = "Name"))]
            name: String,
        }

        let com_con = unsafe { wmi::COMLibrary::assume_initialized() };
        let wmi_con = wmi::WMIConnection::new(com_con)
            .map_err(|err| core_error!("initialize wmi connect error ({})", err))?;
        let result: Vec<VideoControllerInfo> = wmi_con
            .query()
            .map_err(|err| core_error!("wmi query error ({})", err))?;

        for info in result {
            graphics_cards.push(GraphicsCards {
                name: info.name,
                is_default: false,
            });
        }
    }

    Ok(graphics_cards)
}

pub fn enum_broadcast_network_interfaces() -> CoreResult<Vec<(String, IpAddr)>> {
    let interfaces = network_interface::NetworkInterface::show()?;
    let mut valid_interfaces = Vec::new();

    for interface in interfaces {
        let Some(addr) = interface.addr else {
           continue;
        };

        if addr.broadcast().is_none() {
            continue;
        }

        let ip = addr.ip();

        if ip.is_loopback() || ip.is_ipv6() {
            continue;
        }

        valid_interfaces.push((interface.name, ip));
    }

    Ok(valid_interfaces)
}
