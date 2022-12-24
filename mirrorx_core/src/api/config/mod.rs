pub mod entity;

use self::entity::{domain::DomainRepository, history::HistoryRepository, kv::KVRepository};
use crate::error::CoreResult;
use r2d2_sqlite::SqliteConnectionManager;
use std::{path::Path, sync::Arc};

#[derive(Clone)]
pub struct LocalStorage {
    domain: Arc<DomainRepository>,
    kv: Arc<KVRepository>,
    history: Arc<HistoryRepository>,
}

impl LocalStorage {
    pub fn new<P>(db_path: P) -> CoreResult<LocalStorage>
    where
        P: AsRef<Path>,
    {
        let manager = SqliteConnectionManager::file(db_path);
        let pool = r2d2::Pool::new(manager)?;

        let domain_repository = DomainRepository::new(pool.clone());
        domain_repository.ensure_table()?;

        let kv_repository = KVRepository::new(pool.clone());
        kv_repository.ensure_table()?;

        let history_repository = HistoryRepository::new(pool);
        history_repository.ensure_table()?;

        Ok(Self {
            domain: Arc::new(domain_repository),
            kv: Arc::new(kv_repository),
            history: Arc::new(history_repository),
        })
    }

    pub fn domain(&self) -> &DomainRepository {
        &self.domain
    }

    pub fn kv(&self) -> &KVRepository {
        &self.kv
    }

    pub fn history(&self) -> &HistoryRepository {
        &self.history
    }
}
