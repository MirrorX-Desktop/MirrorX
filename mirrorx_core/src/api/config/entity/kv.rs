use crate::{core_error, error::CoreResult};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;

pub struct KVRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl KVRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn ensure_table(&self) -> CoreResult<()> {
        let conn = self.get_connection()?;

        const COMMAND: &str = r"
        CREATE TABLE IF NOT EXISTS kv(
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        )";

        conn.execute(COMMAND, [])?;

        Ok(())
    }

    pub fn set_language(&self, value: &str) -> CoreResult<()> {
        self.set("language", value)
    }

    pub fn get_language(&self) -> CoreResult<Option<String>> {
        self.get("language")
    }

    fn set(&self, key: &str, value: &str) -> CoreResult<()> {
        const COMMAND: &str =
            r"INSERT INTO kv(key, value) VALUES(?, ?) ON CONFLICT DO UPDATE SET value = ?";

        let _ = self
            .get_connection()?
            .execute(COMMAND, [key, value, value])?;

        Ok(())
    }

    fn get(&self, key: &str) -> CoreResult<Option<String>> {
        const COMMAND: &str = r"SELECT value FROM kv WHERE key = ? LIMIT 1";

        let value = self
            .get_connection()?
            .query_row(COMMAND, [key], |row| row.get(0))
            .optional()?;

        Ok(value)
    }

    fn get_connection(&self) -> CoreResult<PooledConnection<SqliteConnectionManager>> {
        let conn = self
            .pool
            .get()
            .map_err(|err| core_error!("get db connection failed ({})", err))?;

        Ok(conn)
    }
}
