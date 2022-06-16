use anyhow::bail;
use rusqlite::{params, Connection, OptionalExtension};
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicPtr, Ordering},
};

static CURRENT_CONFIG_DB_PATH: AtomicPtr<PathBuf> = AtomicPtr::new(std::ptr::null_mut());

#[inline(always)]
pub fn init(config_db_path: &Path) -> anyhow::Result<()> {
    CURRENT_CONFIG_DB_PATH.swap(&mut config_db_path.to_path_buf(), Ordering::SeqCst);
    Ok(())
}

#[inline(always)]
pub fn read_device_id() -> anyhow::Result<Option<String>> {
    read_item("device_id")
}

#[inline(always)]
pub fn save_device_id(device_id: &str) -> anyhow::Result<()> {
    save_item("device_id", device_id)
}

#[inline(always)]
pub fn read_device_id_expiration() -> anyhow::Result<Option<u32>> {
    match read_item("device_id_expiration")? {
        Some(value) => match u32::from_str_radix(&value, 10) {
            Ok(value) => Ok(Some(value)),
            Err(err) => Err(anyhow::anyhow!(err)),
        },
        None => Ok(None),
    }
}

#[inline(always)]
pub fn save_device_id_expiration(time_stamp: &u32) -> anyhow::Result<()> {
    save_item("device_id_expiration", &time_stamp.to_string())
}

#[inline(always)]
pub fn read_device_password() -> anyhow::Result<Option<String>> {
    read_item("device_password")
}

#[inline(always)]
pub fn save_device_password(device_password: &str) -> anyhow::Result<()> {
    save_item("device_password", device_password)
}

fn open_connection() -> anyhow::Result<Connection> {
    let path = CURRENT_CONFIG_DB_PATH.load(Ordering::SeqCst);
    if path.is_null() {
        bail!("open_connection: config db path not set");
    }

    unsafe {
        match Connection::open(*path) {
            Ok(conn) => Ok(conn),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }
}

fn ensure_db_exist(conn: &Connection) -> anyhow::Result<()> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    conn.execute(SQL_COMMAND, [])
        .map_or_else(|err| Err(anyhow::anyhow!(err)), |_| Ok(()))
}

fn read_item(key: &str) -> anyhow::Result<Option<String>> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    conn.query_row(
        "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
        [key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|err| anyhow::anyhow!(err))
}

fn save_item(key: &str, value: &str) -> anyhow::Result<()> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")
        .map_err(|err| anyhow::anyhow!(err))?;

    stmt.execute(params![key, value])
        .map_or_else(|err| Err(anyhow::anyhow!(err)), |_| Ok(()))
}
