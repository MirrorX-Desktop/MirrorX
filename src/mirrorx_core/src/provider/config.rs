use std::path::{Path, PathBuf};

use rusqlite::{params, Connection, OptionalExtension};

pub struct ConfigProvider {
    pub db_file_path: PathBuf,
}

impl ConfigProvider {
    pub fn new(db_path: &Path) -> anyhow::Result<Self> {
        let config_provider = ConfigProvider {
            db_file_path: db_path.join("config.db"),
        };

        let conn = config_provider.open_connection()?;
        config_provider.ensure_db_exist(&conn)?;

        Ok(config_provider)
    }

    pub fn read_device_id(&self) -> anyhow::Result<Option<String>> {
        self.read_item("device_id")
    }

    pub fn save_device_id(&self, device_id: &str) -> anyhow::Result<()> {
        self.save_item("device_id", device_id)
    }

    pub fn read_device_id_expiration(&self) -> anyhow::Result<Option<u32>> {
        match self.read_item("device_id_expiration")? {
            Some(value) => match u32::from_str_radix(&value, 10) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(anyhow::anyhow!(err)),
            },
            None => Ok(None),
        }
    }

    pub fn save_device_id_expiration(&self, time_stamp: &u32) -> anyhow::Result<()> {
        self.save_item("device_id_expiration", &time_stamp.to_string())
    }

    pub fn read_device_password(&self) -> anyhow::Result<Option<String>> {
        self.read_item("device_password")
    }

    pub fn save_device_password(&self, device_password: &str) -> anyhow::Result<()> {
        self.save_item("device_password", device_password)
    }

    fn ensure_db_exist(&self, conn: &Connection) -> anyhow::Result<()> {
        const SQL_COMMAND: &str = r#"
        CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#;

        conn.execute(SQL_COMMAND, [])
            .map_or_else(|err| Err(anyhow::anyhow!(err)), |_| Ok(()))
    }

    fn open_connection(&self) -> anyhow::Result<Connection> {
        match Connection::open(&self.db_file_path) {
            Ok(conn) => Ok(conn),
            Err(err) => Err(anyhow::anyhow!(err)),
        }
    }

    fn read_item(&self, key: &str) -> anyhow::Result<Option<String>> {
        let conn = self.open_connection()?;
        self.ensure_db_exist(&conn)?;

        conn.query_row(
            "SELECT value FROM kv WHERE key = ?1 LIMIT 1;",
            [key],
            |row| row.get(0),
        )
        .optional()
        .map_err(|err| anyhow::anyhow!(err))
    }

    fn save_item(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let conn = self.open_connection()?;
        self.ensure_db_exist(&conn)?;

        let mut stmt = conn
            .prepare("INSERT OR REPLACE INTO kv (key, value) VALUES (?1,?2);")
            .map_err(|err| anyhow::anyhow!(err))?;

        stmt.execute(params![key, value])
            .map_or_else(|err| Err(anyhow::anyhow!(err)), |_| Ok(()))
    }
}
