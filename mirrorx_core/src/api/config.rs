use crate::error::MirrorXError;
use anyhow::anyhow;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

static CURRENT_CONFIG_DB_PATH: OnceCell<PathBuf> = OnceCell::new();

#[inline(always)]
pub fn init(config_db_path: String) -> Result<(), MirrorXError> {
    CURRENT_CONFIG_DB_PATH
        .set(PathBuf::from(config_db_path).join("config.db"))
        .map_err(|_| MirrorXError::ComponentUninitialized)
}

#[inline(always)]
pub fn read_device_id() -> Result<Option<String>, MirrorXError> {
    read_item("device_id")
}

#[inline(always)]
pub fn save_device_id(device_id: &str) -> Result<(), MirrorXError> {
    save_item("device_id", device_id)
}

#[inline(always)]
pub fn read_device_hash() -> Result<Option<String>, MirrorXError> {
    read_item("device_hash")
}

#[inline(always)]
pub fn save_device_hash(device_hash: &str) -> Result<(), MirrorXError> {
    save_item("device_hash", device_hash)
}

#[inline(always)]
pub fn read_device_id_expiration() -> Result<Option<u32>, MirrorXError> {
    match read_item("device_id_expiration")? {
        Some(value) => match u32::from_str_radix(&value, 10) {
            Ok(value) => Ok(Some(value)),
            Err(err) => Err(MirrorXError::Other(anyhow!(err))),
        },
        None => Ok(None),
    }
}

#[inline(always)]
pub fn save_device_id_expiration(time_stamp: &i32) -> Result<(), MirrorXError> {
    save_item("device_id_expiration", &time_stamp.to_string())
}

#[inline(always)]
pub fn read_device_password() -> Result<Option<String>, MirrorXError> {
    read_item("device_password")
}

#[inline(always)]
pub fn save_device_password(device_password: &str) -> Result<(), MirrorXError> {
    save_item("device_password", device_password)
}

fn open_connection() -> Result<Connection, MirrorXError> {
    let path = CURRENT_CONFIG_DB_PATH
        .get()
        .ok_or(MirrorXError::ComponentUninitialized)?;

    Connection::open(path).map_err(|err| MirrorXError::Other(anyhow!(err)))
}

fn ensure_db_exist(conn: &Connection) -> Result<(), MirrorXError> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    conn.execute(SQL_COMMAND, [])
        .map(|_| ())
        .map_err(|err| MirrorXError::Other(anyhow!(err)))
}

fn read_item(key: &str) -> Result<Option<String>, MirrorXError> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    conn.query_row(
        "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
        [key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|err| MirrorXError::Other(anyhow!(err)))
}

fn save_item(key: &str, value: &str) -> Result<(), MirrorXError> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")
        .map_err(|err| MirrorXError::Other(anyhow!(err)))?;

    stmt.execute(params![key, value])
        .map(|_| ())
        .map_err(|err| MirrorXError::Other(anyhow!(err)))
}
