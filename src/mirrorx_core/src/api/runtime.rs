use log::warn;

use crate::provider::runtime::RuntimeProvider;

pub fn init() -> anyhow::Result<()> {
    let provider = RuntimeProvider::new()?;

    if let Err(_) = crate::instance::RUNTIME_PROVIDER_INSTANCE.set(provider) {
        warn!("runtime already initialized");
    }

    Ok(())
}
