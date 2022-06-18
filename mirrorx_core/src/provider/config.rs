use crate::error::{MirrorXError, MirrorXResult};
use rusqlite::{params, Connection, OptionalExtension};
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicPtr, Ordering},
};
use tracing::error;

static CURRENT_CONFIG_DB_PATH: AtomicPtr<PathBuf> = AtomicPtr::new(std::ptr::null_mut());

#[inline(always)]
pub fn init(config_db_path: &Path) -> MirrorXResult<()> {
    CURRENT_CONFIG_DB_PATH.swap(&mut config_db_path.to_path_buf(), Ordering::SeqCst);
    Ok(())
}

#[inline(always)]
pub fn read_device_id() -> MirrorXResult<Option<String>> {
    read_item("device_id")
}

#[inline(always)]
pub fn save_device_id(device_id: &str) -> MirrorXResult<()> {
    save_item("device_id", device_id)
}

#[inline(always)]
pub fn read_unique_id() -> MirrorXResult<Option<String>> {
    read_item("unique_id")
}

#[inline(always)]
pub fn save_unique_id(unique_id: &str) -> MirrorXResult<()> {
    save_item("unique_id", unique_id)
}

#[inline(always)]
pub fn read_device_id_expiration() -> MirrorXResult<Option<u32>> {
    match read_item("device_id_expiration")? {
        Some(value) => match u32::from_str_radix(&value, 10) {
            Ok(value) => Ok(Some(value)),
            Err(err) => {
                error!(err=?err,"read_device_id_expiration: read failed");
                Err(MirrorXError::Raw(err.to_string()))
            }
        },
        None => Ok(None),
    }
}

#[inline(always)]
pub fn save_device_id_expiration(time_stamp: &u32) -> MirrorXResult<()> {
    save_item("device_id_expiration", &time_stamp.to_string())
}

#[inline(always)]
pub fn read_device_password() -> MirrorXResult<Option<String>> {
    read_item("device_password")
}

#[inline(always)]
pub fn save_device_password(device_password: &str) -> MirrorXResult<()> {
    save_item("device_password", device_password)
}

fn open_connection() -> MirrorXResult<Connection> {
    let path = CURRENT_CONFIG_DB_PATH.load(Ordering::SeqCst);
    if path.is_null() {
        error!("open_connection: config db path not set");
        return Err(MirrorXError::ProviderNotInitialized);
    }

    unsafe {
        match Connection::open(*path) {
            Ok(conn) => Ok(conn),
            Err(err) => {
                error!(err=?err,"open_connection: open failed");
                Err(MirrorXError::Raw(err.to_string()))
            }
        }
    }
}

fn ensure_db_exist(conn: &Connection) -> MirrorXResult<()> {
    const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

    if let Err(err) = conn.execute(SQL_COMMAND, []) {
        error!(err=?err,"ensure_db_exist: execute failed");
        Err(MirrorXError::Raw(err.to_string()))
    } else {
        Ok(())
    }
}

fn read_item(key: &str) -> MirrorXResult<Option<String>> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    conn.query_row(
        "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
        [key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|err| {
        error!(key=key, err=?err,"read_item: execute failed");
        MirrorXError::Raw(err.to_string())
    })
}

fn save_item(key: &str, value: &str) -> MirrorXResult<()> {
    let conn = open_connection()?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn
        .prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")
        .map_err(|err| {
            error!(key=key, err=?err,"save_item: prepare failed");
            MirrorXError::Raw(err.to_string())
        })?;

    if let Err(err) = stmt.execute(params![key, value]) {
        error!(key=key,err=?err,"save_item: execute failed");
        Err(MirrorXError::Raw(err.to_string()))
    } else {
        Ok(())
    }
}
