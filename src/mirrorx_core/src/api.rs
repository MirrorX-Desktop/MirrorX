use env_logger::{Builder, Target};
use log::LevelFilter;
use std::{io::Write, path::PathBuf, sync::Once};

static INIT_ONCE: Once = Once::new();
static mut INIT_ONCE_RESULT: bool = false;

pub fn init_sdk(config_db_path: String) -> bool {
    unsafe {
        INIT_ONCE.call_once(|| {
            Builder::new()
                .filter_level(LevelFilter::Info)
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "[{}] [{}({})] {} {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                        record.module_path().unwrap_or(""),
                        record.file().unwrap_or(""),
                        record.level(),
                        record.args()
                    )
                })
                .target(Target::Stdout)
                .init();

            let init_fn = || -> anyhow::Result<()> {
                let config_db_path = PathBuf::from(config_db_path);

                crate::service::config::init_config(config_db_path)?;
                crate::service::runtime::init_async_runtime()?;
                crate::service::network::init_client()?;

                Ok(())
            };

            INIT_ONCE_RESULT = init_fn().is_ok();
        });

        INIT_ONCE_RESULT
    }
}

pub fn read_device_id() -> anyhow::Result<Option<String>> {
    crate::service::config::read_device_id()
}

pub fn read_device_password() -> anyhow::Result<Option<String>> {
    crate::service::config::read_device_password()
}

pub fn save_device_password(device_password: String) -> anyhow::Result<()> {
    crate::service::config::save_device_password(&device_password)
}

pub fn generate_random_device_password() -> anyhow::Result<String> {
    let password = crate::service::utility::generate_device_password();
    crate::service::config::save_device_password(&password).map(|_| password)
}

pub fn device_goes_online() -> anyhow::Result<()> {
    crate::service::network::device_goes_online()
}

pub fn desktop_connect_to(ask_device_id: String) -> anyhow::Result<bool> {
    crate::service::network::connect_to(ask_device_id)
}
