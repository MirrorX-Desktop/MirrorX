use crate::{
    core_error,
    error::{CoreError, CoreResult},
};
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::PathBuf;

static CURRENT_CONFIG_DB_PATH: OnceCell<PathBuf> = OnceCell::new();

#[inline(always)]
pub fn init(config_db_path: String) -> CoreResult<()> {
    CURRENT_CONFIG_DB_PATH
        .set(PathBuf::from(config_db_path).join("config.db"))
        .map_err(|p| core_error!("set CURRENT_CONFIG_DB_PATH failed with path: {:?}", p))
}

#[inline(always)]
pub fn read_device_id() -> CoreResult<Option<String>> {
    read_item("device_id")
}

#[inline(always)]
pub fn save_device_id(device_id: &str) -> CoreResult<()> {
    save_item("device_id", device_id)
}

#[inline(always)]
pub fn read_device_hash() -> CoreResult<Option<String>> {
    read_item("device_hash")
}

#[inline(always)]
pub fn save_device_hash(device_hash: &str) -> CoreResult<()> {
    save_item("device_hash", device_hash)
}

#[inline(always)]
pub fn read_device_id_expiration() -> CoreResult<Option<u32>> {
    match read_item("device_id_expiration")? {
        Some(value) => {
            let exp = u32::from_str_radix(&value, 10)?;
            Ok(Some(exp))
        }
        None => Ok(None),
    }
}

#[inline(always)]
pub fn save_device_id_expiration(time_stamp: &i32) -> CoreResult<()> {
    save_item("device_id_expiration", &time_stamp.to_string())
}

#[inline(always)]
pub fn read_device_password() -> CoreResult<Option<String>> {
    read_item("device_password")
}

#[inline(always)]
pub fn save_device_password(device_password: &str) -> CoreResult<()> {
    save_item("device_password", device_password)
}

fn open_connection() -> CoreResult<Connection> {
    let path = CURRENT_CONFIG_DB_PATH
        .get()
        .ok_or(core_error!("get CURRENT_CONFIG_DB_PATH failed, it's None"))?;

    let conn = Connection::open(path)?;

    Ok(conn)
}

fn ensure_db_exist(conn: &Connection) -> CoreResult<()> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    conn.execute(SQL_COMMAND, []).map(|_| ())?;

    Ok(())
}

fn read_item(key: &str) -> CoreResult<Option<String>> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    let item = conn
        .query_row(
            "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
            [key],
            |row| row.get(0),
        )
        .optional()?;

    Ok(item)
}

fn save_item(key: &str, value: &str) -> CoreResult<()> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")?;
    stmt.execute(params![key, value]).map(|_| ())?;

    Ok(())
}
