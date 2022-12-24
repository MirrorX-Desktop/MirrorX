use crate::error::CoreResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Row};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub id: i64,
    pub device_id: i64,
    pub domain: String,
    pub timestamp: i64,
}

pub struct HistoryRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl HistoryRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn ensure_table(&self) -> CoreResult<()> {
        let conn = self.pool.get()?;

        const CREATE_TABLE_COMMAND: &str = r"
        CREATE TABLE IF NOT EXISTS history(
            id INTEGER PRIMARY KEY,
            device_id INTEGER NOT NULL,
            domain TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )";

        conn.execute(CREATE_TABLE_COMMAND, [])?;

        const CREATE_UNIQUE_INDEX_COMMAND: &str = r"
        CREATE UNIQUE INDEX IF NOT EXISTS uq_device_id_domain ON history(device_id, domain)";

        conn.execute(CREATE_UNIQUE_INDEX_COMMAND, [])?;

        Ok(())
    }

    pub fn create(&self, device_id: i64, domain: &str) -> CoreResult<()> {
        const COMMAND: &str = r"INSERT INTO history(device_id, domain, timestamp) VALUES(?, ?, ?) ON CONFLICT DO UPDATE SET timestamp = ?";

        let timestamp = chrono::Utc::now().timestamp();

        let _ = self
            .pool
            .get()?
            .execute(COMMAND, params![device_id, domain, timestamp, timestamp])?;

        Ok(())
    }

    pub fn query(&self, time_range: Option<(i64, i64)>) -> CoreResult<Vec<Record>> {
        const COMMAND: &str =
            r"SELECT * FROM history WHERE timestamp BETWEEN ? AND ? ORDER BY timestamp DESC";

        let (start, end) = time_range.unwrap_or_else(|| (0, chrono::Utc::now().timestamp()));

        let conn = self.pool.get()?;

        let mut stmt = conn.prepare(COMMAND)?;
        let rows = stmt.query_and_then([start, end], parse_record)?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row?);
        }

        Ok(records)
    }

    pub fn delete_domain_related(&self, domain: &str) -> CoreResult<()> {
        const COMMAND: &str = r"DELETE FROM history WHERE domain = ?";

        let _ = self.pool.get()?.execute(COMMAND, params![domain])?;

        Ok(())
    }
}

fn parse_record(row: &Row) -> CoreResult<Record> {
    Ok(Record {
        id: row.get(0)?,
        device_id: row.get(1)?,
        domain: row.get(2)?,
        timestamp: row.get(3)?,
    })
}
