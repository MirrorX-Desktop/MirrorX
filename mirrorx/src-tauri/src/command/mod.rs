pub mod config;
pub mod file_manager;
pub mod lan;
pub mod signaling;
pub mod utility;

use mirrorx_core::{
    api::{config::LocalStorage, endpoint::client::EndPointClient, signaling::SignalingClient},
    component::lan::LANProvider,
};
use moka::future::{Cache, CacheBuilder};
use std::sync::Arc;
use tauri::async_runtime::Mutex;

pub struct AppState {
    storage: Mutex<Option<LocalStorage>>,
    signaling_client: Mutex<Option<(i64, SignalingClient)>>,
    lan_provider: Mutex<Option<LANProvider>>,
    files_endpoints: Mutex<Cache<String, Arc<EndPointClient>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(None),
            signaling_client: Mutex::new(None),
            lan_provider: Mutex::new(None),
            files_endpoints: Mutex::new(CacheBuilder::new(64).build()),
        }
    }
}
