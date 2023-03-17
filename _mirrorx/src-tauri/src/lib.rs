use mirrorx_core::service::endpoint;
use moka::future::{Cache, CacheBuilder};
use std::sync::Arc;
use tauri::async_runtime::Mutex;

pub mod command;
pub mod utility;
pub mod window;

pub type ConfigService = mirrorx_core::service::config::service::Service;
pub type LANService = mirrorx_core::service::lan::service::Service;

pub struct PortalService(Arc<Mutex<mirrorx_core::service::portal::service::Service>>);

impl PortalService {
    pub fn new(config: ConfigService) -> Self {
        PortalService(Arc::new(Mutex::new(
            mirrorx_core::service::portal::service::Service::new(config),
        )))
    }
}

pub struct FileTransferCache(Cache<String, Arc<endpoint::Service>>);

impl Default for FileTransferCache {
    fn default() -> Self {
        FileTransferCache(CacheBuilder::new(64).build())
    }
}
