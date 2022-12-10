pub mod entity;

use self::entity::{domain::DomainRepository, kv::KVRepository};
use crate::{core_error, error::CoreResult};
use r2d2_sqlite::SqliteConnectionManager;
use std::{path::Path, sync::Arc};

#[derive(Clone)]
pub struct LocalStorage {
    domain: Arc<DomainRepository>,
    kv: Arc<KVRepository>,
}

impl LocalStorage {
    pub fn new<P>(db_path: P) -> CoreResult<LocalStorage>
    where
        P: AsRef<Path>,
    {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = r2d2::Pool::new(manager)
            .map_err(|err| core_error!("init sqlite connection pool failed ({})", err))?;

        let domain_repository = DomainRepository::new(pool.clone());
        domain_repository.ensure_table()?;

        let kv_repository = KVRepository::new(pool);
        kv_repository.ensure_table()?;

        Ok(Self {
            domain: Arc::new(domain_repository),
            kv: Arc::new(kv_repository),
        })
    }

    pub fn domain(&self) -> &DomainRepository {
        &self.domain
    }

    pub fn kv(&self) -> &KVRepository {
        &self.kv
    }
}
