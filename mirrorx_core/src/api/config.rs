use crate::error::CoreResult;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DomainConfig {
    pub uri: String,
    pub device_id: i64,
    pub device_finger_print: String,
    pub device_password: String,
}

pub fn read_primary_domain(path: &str) -> CoreResult<Option<String>> {
    read(path, "primary_domain")
}

pub fn save_primary_domain(path: &str, value: &str) -> CoreResult<()> {
    save(path, "primary_domain", value)
}

pub fn read_domain_config(path: &str, domain: &str) -> CoreResult<Option<DomainConfig>> {
    read(path, domain)?.map_or(Ok(None), |v| Ok(Some(serde_json::from_str(&v)?)))
}

pub fn save_domain_config(path: &str, domain: &str, value: &DomainConfig) -> CoreResult<()> {
    save(path, domain, &serde_json::to_string(value)?)
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

fn read(path: &str, key: &str) -> CoreResult<Option<String>> {
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

// pub fn read_all(path: &str) -> CoreResult<HashMap<String, String>> {
//     let conn = Connection::open(path)?;
//     ensure_db_exist(&conn)?;

//     let mut stmt = conn.prepare("SELECT * FROM kv;")?;
//     let entry_iter = stmt.query_map([], |row| {
//         let key = row.get::<_, String>(0)?;
//         let value = row.get::<_, String>(1)?;
//         Ok((key, value))
//     })?;

//     let mut all_config_properties = HashMap::new();
//     for entry in entry_iter {
//         let (key, value) = entry?;
//         let config_properties = serde_json::from_str(&value)?;
//         all_config_properties.insert(key, config_properties);
//     }

//     Ok(all_config_properties)
// }

fn save(path: &str, key: &str, value: &str) -> CoreResult<()> {
    let conn = Connection::open(path)?;
    ensure_db_exist(&conn)?;

    let mut stmt = conn.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")?;
    stmt.execute(params![key, value]).map(|_| ())?;

    Ok(())
}
