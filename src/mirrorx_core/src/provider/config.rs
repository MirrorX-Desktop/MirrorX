use anyhow::bail;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

static CURRENT_CONFIG_PROVIDER: OnceCell<ConfigProvider> = OnceCell::new();

pub struct ConfigProvider {
    db_file_path: PathBuf,
}

impl ConfigProvider {
    pub fn current() -> anyhow::Result<&'static ConfigProvider> {
        CURRENT_CONFIG_PROVIDER
            .get()
            .ok_or_else(|| anyhow::anyhow!("ConfigProvider: uninitialized"))
    }

    pub fn make_current(db_dir: &Path) -> anyhow::Result<()> {
        match CURRENT_CONFIG_PROVIDER.get_or_try_init(|| -> anyhow::Result<ConfigProvider> {
            let provider = ConfigProvider {
                db_file_path: db_dir.join("config.db"),
            };

            let conn = provider.open_connection()?;
            provider.ensure_db_exist(&conn)?;

            Ok(provider)
        }) {
            Ok(_) => Ok(()),
            Err(err) => bail!("ConfigProvider: make current failed: {}", err),
        }
    }

    #[inline(always)]
    pub fn read_device_id(&self) -> anyhow::Result<Option<String>> {
        self.read_item("device_id")
    }

    #[inline(always)]
    pub fn save_device_id(&self, device_id: &str) -> anyhow::Result<()> {
        self.save_item("device_id", device_id)
    }

    #[inline(always)]
    pub fn read_device_id_expiration(&self) -> anyhow::Result<Option<u32>> {
        match self.read_item("device_id_expiration")? {
            Some(value) => match u32::from_str_radix(&value, 10) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(anyhow::anyhow!(err)),
            },
            None => Ok(None),
        }
    }

    #[inline(always)]
    pub fn save_device_id_expiration(&self, time_stamp: &u32) -> anyhow::Result<()> {
        self.save_item("device_id_expiration", &time_stamp.to_string())
    }

    #[inline(always)]
    pub fn read_device_password(&self) -> anyhow::Result<Option<String>> {
        self.read_item("device_password")
    }

    #[inline(always)]
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
