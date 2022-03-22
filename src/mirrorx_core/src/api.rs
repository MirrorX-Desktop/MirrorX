use env_logger::{Builder, Target};
use log::{error, LevelFilter};
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
                if let Err(err) = crate::service::config::init_config(config_db_path) {
                    error!("init_sdk: init_config returns error: {:?}", &err);
                    return Err(anyhow::anyhow!(""));
                };

                if let Err(err) = crate::service::runtime::init_async_runtime() {
                    error!("init_sdk: init_async_runtime returns error: {:?}", &err);
                    return Err(anyhow::anyhow!(""));
                }

                if let Err(err) = crate::service::desktop::init_client() {
                    error!("init_sdk: init_client returns error: {:?}", &err);
                    return Err(anyhow::anyhow!(""));
                }

                Ok(())
            };

            INIT_ONCE_RESULT = init_fn().is_ok();
        });

        INIT_ONCE_RESULT
    }
}

pub fn create_or_update_token(token: Option<String>) -> anyhow::Result<String> {
    crate::service::http::create_or_update_token(token).or_else(|err| {
        error!("create_or_update_token returns error: {:?}", &err);
        Err(anyhow::anyhow!(""))
    })
}

pub fn read_config(key: String) -> anyhow::Result<Option<String>> {
    crate::service::config::read_config(&key).or_else(|err| {
        error!("read_config returns error: {:?}", &err);
        Err(anyhow::anyhow!(""))
    })
}

pub fn store_config(key: String, value: String) -> anyhow::Result<()> {
    crate::service::config::save_config(&key, &value).or_else(|err| {
        error!("store_config returns error: {:?}", &err);
        Err(anyhow::anyhow!(""))
    })
}

pub fn generate_device_password() -> String {
    crate::service::profile::generate_device_password()
}

pub fn desktop_connect_to(device_id: String) -> anyhow::Result<bool> {
    crate::service::desktop::connect_to(device_id).or_else(|err| {
        error!("desktop_connect_to returns error: {:?}", &err);
        Err(anyhow::anyhow!(""))
    })
}
