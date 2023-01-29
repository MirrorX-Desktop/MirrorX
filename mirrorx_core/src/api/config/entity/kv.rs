use crate::{core_error, error::CoreResult};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl<'a> From<Theme> for &'a str {
    fn from(val: Theme) -> Self {
        match val {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::Auto => "auto",
        }
    }
}

impl FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            "auto" => Ok(Theme::Auto),
            _ => Err(String::from("Unknown theme type")),
        }
    }
}

pub struct KVRepository {
    pool: Pool<SqliteConnectionManager>,
}

impl KVRepository {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    pub fn ensure_table(&self) -> CoreResult<()> {
        let conn = self.pool.get()?;

        const COMMAND: &str = r"
        CREATE TABLE IF NOT EXISTS kv(
            id INTEGER PRIMARY KEY,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL
        )";

        conn.execute(COMMAND, [])?;

        Ok(())
    }

    pub fn set_language(&self, value: &str) -> CoreResult<()> {
        self.set("language", value)
    }

    pub fn get_language(&self) -> CoreResult<Option<String>> {
        self.get("language")
    }

    pub fn set_theme(&self, value: Theme) -> CoreResult<()> {
        self.set("theme", value.into())
    }

    pub fn get_theme(&self) -> CoreResult<Option<Theme>> {
        match self.get("theme")? {
            Some(theme_str) => match Theme::from_str(&theme_str) {
                Ok(theme) => Ok(Some(theme)),
                Err(err) => Err(core_error!("{}", err)),
            },
            None => Ok(None),
        }
    }

    fn set(&self, key: &str, value: &str) -> CoreResult<()> {
        const COMMAND: &str =
            r"INSERT INTO kv(key, value) VALUES(?, ?) ON CONFLICT DO UPDATE SET value = ?";

        let _ = self.pool.get()?.execute(COMMAND, [key, value, value])?;

        Ok(())
    }

    fn get(&self, key: &str) -> CoreResult<Option<String>> {
        const COMMAND: &str = r"SELECT value FROM kv WHERE key = ? LIMIT 1";

        let value = self
            .pool
            .get()?
            .query_row(COMMAND, [key], |row| row.get(0))
            .optional()?;

        Ok(value)
    }
}
