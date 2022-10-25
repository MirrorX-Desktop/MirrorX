use crate::error::CoreResult;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub primary_domain: String,
    pub domain_configs: HashMap<String, DomainConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DomainConfig {
    pub addr: String,
    pub device_id: i64,
    pub device_finger_print: String,
    pub device_password: String,
}

pub fn read(path: &Path) -> CoreResult<Option<Config>> {
    let conn = Connection::open(path)?;
    ensure_tables(&conn)?;

    match conn
        .query_row(
            "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
            [b"config"],
            |row| row.get::<_, String>(0),
        )
        .optional()?
    {
        Some(res) => {
            let config: Config = serde_json::from_str(&res)?;
            Ok(Some(config))
        }
        None => Ok(None),
    }
}

pub fn save(path: &Path, config: &Config) -> CoreResult<()> {
    let value = serde_json::to_string(config)?;

    let conn = Connection::open(path)?;
    ensure_tables(&conn)?;

    let mut stmt = conn.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")?;
    stmt.execute(params![b"config", value]).map(|_| ())?;

    Ok(())
}

fn ensure_tables(conn: &Connection) -> CoreResult<()> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    conn.execute(SQL_COMMAND, [])?;

    Ok(())
}
