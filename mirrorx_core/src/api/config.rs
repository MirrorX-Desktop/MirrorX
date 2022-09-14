use crate::error::CoreResult;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ConfigProperties {
    pub domain: String,
    pub device_id: i64,
    pub device_finger_print: String,
    pub device_password: String,
}

fn ensure_db_exist(conn: &Connection) -> CoreResult<()> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    conn.execute(SQL_COMMAND, [])?;

    Ok(())
}

pub fn read(path: &str, key: &str) -> CoreResult<Option<ConfigProperties>> {
    let conn = Connection::open(path)?;
    ensure_db_exist(&conn)?;

    match conn
        .query_row(
            "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
            [key],
            |row| row.get::<_, String>(0),
        )
        .optional()?
    {
        Some(res) => Ok(serde_json::from_str(&res)?),
        None => Ok(None),
    }
}

pub fn read_all(path: &str) -> CoreResult<HashMap<String, ConfigProperties>> {
    let conn = Connection::open(path)?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn.prepare("SELECT * FROM kv;")?;
    let entry_iter = stmt.query_map([], |row| {
        let key = row.get::<_, String>(0)?;
        let value = row.get::<_, String>(1)?;
        Ok((key, value))
    })?;

    let mut all_config_properties = HashMap::new();
    for entry in entry_iter {
        let (key, value) = entry?;
        let config_properties = serde_json::from_str(&value)?;
        all_config_properties.insert(key, config_properties);
    }

    Ok(all_config_properties)
}

pub fn save(path: &str, key: &str, value: &ConfigProperties) -> CoreResult<()> {
    let model_json_string = serde_json::to_string(value)?;

    let conn = Connection::open(path)?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")?;
    stmt.execute(params![key, model_json_string]).map(|_| ())?;

    Ok(())
}
