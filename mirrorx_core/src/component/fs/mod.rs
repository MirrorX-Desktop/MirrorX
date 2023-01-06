#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub mod transfer;

use crate::error::CoreResult;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub is_dir: bool,
    pub path: PathBuf,
    pub modified_time: i64,
    pub size: u64,
    #[serde(with = "serde_bytes")]
    pub icon: Option<Vec<u8>>,
}

pub fn read_root_directory() -> CoreResult<Directory> {
    #[cfg(not(target_os = "windows"))]
    return read_directory("/");

    #[cfg(target_os = "windows")]
    return self::windows::read_root_directory();
}

fn read_icon(path: &Path) -> CoreResult<Vec<u8>> {
    #[cfg(not(target_os = "windows"))]
    return self::macos::NSWorkspace::sharedWorkspace()?.iconForFile(path);

    #[cfg(target_os = "windows")]
    return self::windows::read_icon(path);
}

pub fn read_directory<P>(path: P) -> CoreResult<Directory>
where
    P: AsRef<Path> + Into<PathBuf>,
{
    struct EntryStat {
        path: PathBuf,
        is_dir: bool,
        modified_time: i64,
        size: u64,
    }

    let dir = std::fs::read_dir(&path)?;
    let mut entries = Vec::new();
    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let meta = entry.metadata()?;

        let is_dir = if file_type.is_symlink() {
            let link_path = std::fs::read_link(entry.path())?;
            link_path.is_dir()
        } else {
            file_type.is_dir()
        };

        let modified_time = chrono::DateTime::<chrono::Local>::from(meta.modified()?)
            .naive_utc()
            .timestamp();

        entries.push(EntryStat {
            path: entry.path(),
            is_dir,
            modified_time,
            size: meta.len(),
        });
    }

    let entries: Vec<Entry> = entries
        .into_par_iter()
        .map(|entry| {
            let icon = read_icon(&entry.path).ok();

            Entry {
                is_dir: entry.is_dir,
                path: entry.path,
                modified_time: entry.modified_time,
                size: entry.size,
                icon,
            }
        })
        .collect();

    Ok(Directory {
        path: path.into(),
        entries,
    })
}
