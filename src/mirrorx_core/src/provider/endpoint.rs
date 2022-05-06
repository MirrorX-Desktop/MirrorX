use crate::socket::endpoint::EndPoint;
use anyhow::bail;
use once_cell::sync::OnceCell;
use rustc_hash::FxHasher;
use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    sync::{Arc, RwLock},
};

static CURRENT_ENDPOINT_PROVIDER: OnceCell<EndPointProvider> = OnceCell::new();

pub struct EndPointProvider {
    endpoints: RwLock<HashMap<String, Arc<EndPoint>, BuildHasherDefault<FxHasher>>>,
}

impl EndPointProvider {
    pub fn current() -> anyhow::Result<&'static EndPointProvider> {
        CURRENT_ENDPOINT_PROVIDER
            .get()
            .ok_or_else(|| anyhow::anyhow!("EndPointProvider: uninitialized"))
    }

    pub fn make_current() -> anyhow::Result<()> {
        match CURRENT_ENDPOINT_PROVIDER.get_or_try_init(|| -> anyhow::Result<EndPointProvider> {
            let provider = EndPointProvider {
                endpoints: RwLock::new(HashMap::with_capacity_and_hasher(
                    8,
                    BuildHasherDefault::<FxHasher>::default(),
                )),
            };

            Ok(provider)
        }) {
            Ok(_) => Ok(()),
            Err(err) => bail!("EndPointProvider: make current failed: {}", err),
        }
    }

    #[inline(always)]
    pub fn contains(&self, remote_device_id: &str) -> bool {
        self.endpoints
            .read()
            .unwrap()
            .contains_key(remote_device_id)
    }

    #[inline(always)]
    pub fn get(&self, remote_device_id: &str) -> Option<Arc<EndPoint>> {
        self.endpoints
            .read()
            .unwrap()
            .get(remote_device_id)
            .map(|endpoint| endpoint.clone())
    }

    #[inline(always)]
    pub fn insert(
        &self,
        remote_device_id: String,
        endpoint: Arc<EndPoint>,
    ) -> Option<Arc<EndPoint>> {
        self.endpoints
            .write()
            .unwrap()
            .insert(remote_device_id, endpoint)
    }

    #[inline(always)]
    pub fn remove(&self, remote_device_id: &str) -> Option<Arc<EndPoint>> {
        self.endpoints.write().unwrap().remove(remote_device_id)
    }
}
