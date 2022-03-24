use lazy_static::lazy_static;
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::PathBuf, sync::RwLock};

lazy_static! {
    static ref INNER_CONFIG_DB_PATH: RwLock<PathBuf> = RwLock::new(PathBuf::default());
}

enum ConfigKey {
    DeviceID,
    DeviceIDExpireAt,
    DevicePassword,
}

impl ConfigKey {
    fn as_str(self) -> &'static str {
        match self {
            ConfigKey::DeviceID => "device_id",
            ConfigKey::DeviceIDExpireAt => "device_id_expire_at",
            ConfigKey::DevicePassword => "device_password",
        }
    }
}

pub fn init_config(path: PathBuf) -> anyhow::Result<()> {
    let mut db_path = INNER_CONFIG_DB_PATH.write().unwrap();
    db_path.clear();
    db_path.push(path);
    db_path.push("config.db");

    let db = Connection::open(db_path.as_path())?;

    check_table(&db)?;

    Ok(())
}

pub fn read_device_id() -> anyhow::Result<Option<String>> {
    read_config(ConfigKey::DeviceID.as_str())
}

pub fn save_device_id(device_id: &str) -> anyhow::Result<()> {
    save_config(ConfigKey::DeviceID.as_str(), device_id)
}

pub fn read_device_id_expire_at() -> anyhow::Result<Option<u32>> {
    match read_config(ConfigKey::DeviceIDExpireAt.as_str())? {
        Some(value) => u32::from_str_radix(&value, 10)
            .map(|v| Some(v))
            .map_err(|err| anyhow::anyhow!(err)),
        None => Ok(None),
    }
}

pub fn save_device_id_expire_at(time_stamp: &u32) -> anyhow::Result<()> {
    save_config(
        ConfigKey::DeviceIDExpireAt.as_str(),
        &time_stamp.to_string(),
    )
}

pub fn read_device_password() -> anyhow::Result<Option<String>> {
    read_config(ConfigKey::DevicePassword.as_str())
}

pub fn save_device_password(device_password: &str) -> anyhow::Result<()> {
    save_config(ConfigKey::DevicePassword.as_str(), device_password)
}

fn check_table(db: &Connection) -> anyhow::Result<()> {
    db.execute(
        "CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

fn read_config(key: &str) -> anyhow::Result<Option<String>> {
    let db_path = INNER_CONFIG_DB_PATH.read().unwrap();
    let db = Connection::open(db_path.as_path())?;

    db.query_row(
        "SELECT value FROM kv WHERE key = ?1 LIMIT 1",
        [key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|err| anyhow::anyhow!(err))
}

fn save_config(key: &str, value: &str) -> anyhow::Result<()> {
    let db_path = INNER_CONFIG_DB_PATH.read().unwrap();
    let db = Connection::open(db_path.as_path())?;

    check_table(&db)?;

    let mut stmt = db.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2)")?;
    stmt.execute(params![key, value])
        .map(|_| ())
        .map_err(|err| anyhow::anyhow!(err))
}
