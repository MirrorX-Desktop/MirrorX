use crate::error::CoreResult;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct DomainConfig {
    pub uri: String,
    pub device_id: i64,
    pub device_finger_print: String,
    pub device_password: String,
}

pub struct ConfigManager {
    conn: Mutex<Connection>,
}

impl ConfigManager {
    pub fn new(db_path: &Path) -> CoreResult<ConfigManager> {
        let conn = Connection::open(db_path)?;
        ensure_tables(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub async fn domain(&self) -> CoreResult<Option<String>> {
        self.read("domain").await
    }

    pub async fn save_domain(&self, value: &str) -> CoreResult<()> {
        self.save("domain", value).await
    }

    pub async fn domain_config(&self, domain: &str) -> CoreResult<Option<DomainConfig>> {
        self.read(&build_key(vec!["domain", domain]))
            .await?
            .map_or(Ok(None), |v| Ok(Some(serde_json::from_str(&v)?)))
    }

    pub async fn save_domain_config(&self, domain: &str, value: &DomainConfig) -> CoreResult<()> {
        self.save(
            &build_key(vec!["domain", domain]),
            &serde_json::to_string(value)?,
        )
        .await
    }

    async fn read(&self, key: &str) -> CoreResult<Option<String>> {
        match self
            .conn
            .lock()
            .await
            .query_row(
                "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
                [key],
                |row| row.get::<_, String>(0),
            )
            .optional()?
        {
            Some(res) => Ok(Some(res)),
            None => Ok(None),
        }
    }

    async fn save(&self, key: &str, value: &str) -> CoreResult<()> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")?;
        stmt.execute(params![key, value]).map(|_| ())?;

        Ok(())
    }
}

fn build_key(paths: Vec<&str>) -> String {
    paths.join(".")
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
