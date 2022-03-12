use lazy_static::lazy_static;
use rusqlite::Connection;
use std::{collections::HashMap, error::Error, path::PathBuf, sync::RwLock};

lazy_static! {
    static ref CONFIG_DB_PATH: RwLock<PathBuf> = RwLock::new(PathBuf::new());
    static ref CONFIG_KV_MAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

pub fn init_config(path: PathBuf) -> Result<(), Box<dyn Error>> {
    update_config_db_path(path)?;

    let config_db_path = CONFIG_DB_PATH.read()?;
    let db = Connection::open(config_db_path.to_path_buf())?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    load_config(&db)?;

    Ok(())
}

pub fn read_config(key: &str) -> Result<Option<String>, Box<dyn Error>> {
    let kv = CONFIG_KV_MAP.read()?;
    let value = kv.get(key).map(|v| v.to_string());
    Ok(value)
}

pub fn save_config(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mut kv = CONFIG_KV_MAP.write()?;
    let config_db_path = CONFIG_DB_PATH.read()?;

    let db = Connection::open(config_db_path.to_path_buf())?;
    let mut stmt = db.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2)")?;
    stmt.execute(&[&key, &value])?;

    kv.insert(key.to_string(), value.to_string());

    Ok(())
}

fn update_config_db_path(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut db_path = CONFIG_DB_PATH.write()?;
    db_path.clear();
    db_path.push(path);
    db_path.push("config.db");
    Ok(())
}

fn load_config(db: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = db.prepare("SELECT key, value FROM kv")?;
    let mut config_kv_map = CONFIG_KV_MAP.write()?;

    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let key = row.get(0)?;
        let value = row.get(1)?;
        config_kv_map.insert(key, value);
    }

    Ok(())
}
