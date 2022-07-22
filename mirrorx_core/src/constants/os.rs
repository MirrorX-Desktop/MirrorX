use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum LinuxType {
    CentOS,
    Fedora,
    Redhat,
    #[allow(non_camel_case_types)]
    openSUSE,
    Ubuntu,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum OperatingSystemType {
    Windows,
    #[allow(non_camel_case_types)]
    macOS,
    #[allow(non_camel_case_types)]
    iOS,
    #[allow(non_camel_case_types)]
    Android,
    Linux(LinuxType),
    Unknown,
}

pub static OS_TYPE: Lazy<OperatingSystemType> = Lazy::new(|| {
    if cfg!(target_os = "windows") {
        OperatingSystemType::Windows
    } else if cfg!(target_os = "macos") {
        OperatingSystemType::macOS
    } else if cfg!(target_os = "ios") {
        OperatingSystemType::iOS
    } else if cfg!(target_os = "android") {
        OperatingSystemType::Android
    } else if cfg!(target_os = "linux") {
        let os_info = os_info::get();
        let linux_type = match os_info.os_type() {
            os_info::Type::CentOS => LinuxType::CentOS,
            os_info::Type::Fedora => LinuxType::Fedora,
            os_info::Type::Redhat | os_info::Type::RedHatEnterprise => LinuxType::Redhat,
            os_info::Type::SUSE | os_info::Type::openSUSE => LinuxType::openSUSE,
            os_info::Type::Ubuntu => LinuxType::Ubuntu,
            _ => LinuxType::Other,
        };

        OperatingSystemType::Linux(linux_type)
    } else {
        OperatingSystemType::Unknown
    }
});

pub static OS_VERSION: OnceCell<String> = OnceCell::new();
