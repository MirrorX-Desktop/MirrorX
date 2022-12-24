use crate::error::{CoreError, CoreResult};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, Row};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Domain {
    pub id: i64,
    pub name: String,
    pub addr: String,
    pub signaling_port: u16,
    pub subscribe_port: u16,
    pub is_primary: bool,
    pub device_id: i64,
    pub password: String,
    pub finger_print: String,
    pub remarks: String,
}

pub struct DomainRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl DomainRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn ensure_table(&self) -> CoreResult<()> {
        let conn = self.pool.get()?;

        const COMMAND: &str = r"
        CREATE TABLE IF NOT EXISTS domains(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            addr TEXT NOT NULL,
            signaling_port INTEGER NOT NULL,
            subscribe_port INTEGER NOT NULL,
            is_primary BOOLEAN NOT NULL,
            device_id INTEGER NOT NULL,
            password TEXT NOT NULL,
            finger_print TEXT NOT NULL,
            remarks TEXT NOT NULL
        )";

        conn.execute(COMMAND, [])?;

        Ok(())
    }

    pub fn add_domain(&self, mut domain: Domain) -> CoreResult<Domain> {
        const COMMAND: &str = r#"
        INSERT INTO domains(
            name,
            addr,
            signaling_port,
            subscribe_port,
            is_primary,
            device_id,
            password,
            finger_print,
            remarks
        )
        VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let conn = self.pool.get()?;
        conn.execute(
            COMMAND,
            params![
                domain.name,
                domain.addr,
                domain.signaling_port,
                domain.subscribe_port,
                domain.is_primary,
                domain.device_id,
                domain.password,
                domain.finger_print,
                domain.remarks,
            ],
        )?;

        domain.id = conn.last_insert_rowid();

        Ok(domain)
    }

    pub fn get_primary_domain(&self) -> CoreResult<Domain> {
        const COMMAND: &str = r"SELECT * FROM domains WHERE is_primary = 1 LIMIT 1";

        self.pool
            .get()?
            .query_row_and_then(COMMAND, [], parse_domain)
    }

    pub fn domain_exist(&self, name: &str) -> CoreResult<bool> {
        const COMMAND: &str = r"SELECT 1 FROM domains WHERE name = ?";

        let res = self
            .pool
            .get()?
            .query_row(COMMAND, [name], |row| row.get::<_, u32>(0))
            .optional()?;

        Ok(res.is_some())
    }

    pub fn get_domain_id_and_names(&self) -> CoreResult<Vec<(i64, String)>> {
        const COMMAND: &str = r"SELECT id, name FROM domains";

        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(COMMAND)?;
        let rows = stmt.query_and_then([], |row| -> CoreResult<(i64, String)> {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        let mut id_and_names = Vec::new();
        for row in rows {
            id_and_names.push(row?);
        }

        Ok(id_and_names)
    }

    pub fn get_domain_by_name(&self, name: String) -> CoreResult<Domain> {
        const COMMAND: &str = r"SELECT * FROM domains WHERE name = ? LIMIT 1";

        let domain = self
            .pool
            .get()?
            .query_row_and_then(COMMAND, [name], parse_domain)?;

        Ok(domain)
    }

    pub fn get_domain_by_id(&self, domain_id: i64) -> CoreResult<Domain> {
        const COMMAND: &str = r"SELECT * FROM domains WHERE id = ?";

        let domain = self
            .pool
            .get()?
            .query_row_and_then(COMMAND, [domain_id], parse_domain)?;

        Ok(domain)
    }

    pub fn get_domains(&self, page: u32, limit: u32) -> CoreResult<(u32, Vec<Domain>)> {
        const COUNT_COMMAND: &str = r"SELECT COUNT(*) FROM domains";
        const PAGINATION_COMMAND: &str = r"SELECT * FROM domains LIMIT ? OFFSET ?";

        let conn = self.pool.get()?;

        let count = conn.query_row_and_then(COUNT_COMMAND, [], |row| -> CoreResult<u32> {
            Ok(row.get(0)?)
        })?;

        let mut stmt = conn.prepare(PAGINATION_COMMAND)?;
        let rows = stmt.query_and_then([limit, (page - 1) * limit], parse_domain)?;

        let mut domains = Vec::new();
        for row in rows {
            domains.push(row?);
        }

        Ok((count, domains))
    }

    pub fn get_domain_count(&self) -> CoreResult<u32> {
        const COMMAND: &str = r"SELECT COUNT(*) FROM domains";
        self.pool
            .get()?
            .query_row_and_then(COMMAND, [], |row| Ok(row.get(0)?))
    }

    pub fn set_domain_is_primary(&self, domain_id: i64) -> CoreResult<()> {
        const UNSET_PRIMARY_COMMAND: &str =
            r"UPDATE domains SET is_primary = 0 WHERE is_primary = 1";
        const SET_PRIMARY_COMMAND: &str = r"UPDATE domains SET is_primary = 1 WHERE id = ?";

        let mut conn = self.pool.get()?;
        let tx = conn.transaction()?;
        if let Err(err) = tx.execute(UNSET_PRIMARY_COMMAND, []) {
            tx.rollback()?;
            return Err(CoreError::SQLiteError(err));
        }

        if let Err(err) = tx.execute(SET_PRIMARY_COMMAND, [domain_id]) {
            tx.rollback()?;
            return Err(CoreError::SQLiteError(err));
        }

        tx.commit()?;

        Ok(())
    }

    pub fn set_domain_device_id(&self, domain_id: i64, device_id: i64) -> CoreResult<()> {
        const COMMAND: &str = r"UPDATE domains SET device_id = ? WHERE id =?";

        self.pool
            .get()?
            .execute(COMMAND, params![device_id, domain_id])?;

        Ok(())
    }

    pub fn set_domain_device_password(&self, domain_id: i64, password: &str) -> CoreResult<()> {
        const COMMAND: &str = r"UPDATE domains SET password = ? WHERE id =?";

        self.pool
            .get()?
            .execute(COMMAND, params![password, domain_id])?;

        Ok(())
    }

    pub fn set_domain_remarks(&self, domain_id: i64, remarks: &str) -> CoreResult<()> {
        const COMMAND: &str = r"UPDATE domains SET remarks = ? WHERE id =?";

        self.pool
            .get()?
            .execute(COMMAND, params![remarks, domain_id])?;

        Ok(())
    }

    pub fn delete_domain(&self, domain_id: i64) -> CoreResult<()> {
        const COMMAND: &str = r"DELETE FROM domains WHERE id = ?";

        self.pool.get()?.execute(COMMAND, [domain_id])?;

        Ok(())
    }
}

fn parse_domain(row: &Row) -> CoreResult<Domain> {
    Ok(Domain {
        id: row.get(0)?,
        name: row.get(1)?,
        addr: row.get(2)?,
        signaling_port: row.get(3)?,
        subscribe_port: row.get(4)?,
        is_primary: row.get(5)?,
        device_id: row.get(6)?,
        password: row.get(7)?,
        finger_print: row.get(8)?,
        remarks: row.get(9)?,
    })
}
