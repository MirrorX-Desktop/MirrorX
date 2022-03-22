use lazy_static::lazy_static;
use rusqlite::Connection;
use std::{collections::HashMap, error::Error, path::PathBuf, sync::RwLock};

lazy_static! {
    static ref CONFIG_DB_PATH: RwLock<PathBuf> = RwLock::new(PathBuf::new());
    static ref CONFIG_KV_MAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub fn init_config(path: PathBuf) -> anyhow::Result<()> {
    update_config_db_path(path)?;

    let config_db_path = CONFIG_DB_PATH
        .read()
        .map_err(|err| anyhow::anyhow!("init_config: read config_db_path error, {}", err))?;

    let db = Connection::open(config_db_path.to_path_buf())?;

    check_table(&db)?;

    load_config(&db)?;

    Ok(())
}

pub fn read_config(key: &str) -> anyhow::Result<Option<String>> {
    let kv = CONFIG_KV_MAP
        .read()
        .map_err(|err| anyhow::anyhow!("read_config: read config_kv_map error, {}", err))?;

    let value = kv.get(key).map(|v| v.to_string());
    Ok(value)
}

pub fn save_config(key: &str, value: &str) -> anyhow::Result<()> {
    let config_db_path = CONFIG_DB_PATH
        .read()
        .map_err(|err| anyhow::anyhow!("save_config: read config_db_path error, {}", err))?;

    let db = Connection::open(config_db_path.to_path_buf())?;
    check_table(&db)?;
    let mut stmt = db.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2)")?;
    stmt.execute(&[&key, &value])?;

    let mut kv = CONFIG_KV_MAP
        .write()
        .map_err(|err| anyhow::anyhow!("save_config: write config_kv_map error, {}", err))?;
    kv.insert(key.to_string(), value.to_string());

    Ok(())
}

fn update_config_db_path(path: PathBuf) -> anyhow::Result<()> {
    let mut db_path = CONFIG_DB_PATH.write().map_err(|err| {
        anyhow::anyhow!("update_config_db_path: read config_db_path error, {}", err)
    })?;

    db_path.clear();
    db_path.push(path);
    db_path.push("config.db");
    Ok(())
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

fn load_config(db: &Connection) -> anyhow::Result<()> {
    let mut stmt = db.prepare("SELECT key, value FROM kv")?;

    let mut config_kv_map = CONFIG_KV_MAP
        .write()
        .map_err(|err| anyhow::anyhow!("load_config: write config_kv_map error, {}", err))?;

    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let key = row.get(0)?;
        let value = row.get(1)?;
        config_kv_map.insert(key, value);
    }

    Ok(())
}
