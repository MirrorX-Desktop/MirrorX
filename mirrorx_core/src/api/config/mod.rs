pub mod entity;

use self::entity::{domain::DomainRepository, kv::KVRepository};
use crate::{core_error, error::CoreResult};
use once_cell::sync::{Lazy, OnceCell};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

static mut REPOSITORY_CELL: Lazy<OnceCell<LocalStorage>> = Lazy::new(OnceCell::new);

pub struct LocalStorage {
    domain: DomainRepository,
    kv: KVRepository,
}

impl LocalStorage {
    pub fn current() -> CoreResult<&'static LocalStorage> {
        unsafe {
            REPOSITORY_CELL
                .get()
                .ok_or_else(|| core_error!("current LocalStorage is empty"))
        }
    }

    pub fn make_current<P>(db_path: P) -> CoreResult<&'static LocalStorage>
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

        let repository = Self {
            domain: domain_repository,
            kv: kv_repository,
        };

        unsafe {
            REPOSITORY_CELL.take();
            let _ = REPOSITORY_CELL
                .set(repository)
                .map_err(|_| core_error!("set repository cell failed"));
        }

        Ok(unsafe { REPOSITORY_CELL.get().unwrap() })
    }

    pub fn domain(&self) -> &DomainRepository {
        &self.domain
    }

    pub fn kv(&self) -> &KVRepository {
        &self.kv
    }
}
