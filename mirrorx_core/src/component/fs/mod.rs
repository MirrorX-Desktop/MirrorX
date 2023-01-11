#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub mod transfer;

use crate::{core_error, error::CoreResult};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub icon_cache: HashMap<HashableIconType, Option<Vec<u8>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub is_dir: bool,
    pub path: PathBuf,
    pub modified_time: i64,
    pub size: u64,
    pub icon: IconLoad,
}

#[derive(Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum IconLoad {
    Hash(HashableIconType),
    #[serde(with = "serde_bytes")]
    Bytes(Option<Vec<u8>>),
}

#[derive(Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum HashableIconType {
    Ext(String),    // extension suffix (exclusive .exe), like: png, jpg, mp4, etc.
    UnixExecutable, // Unix Executable File
    OrdinaryDir,    // Ordinary Directory
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
    #[derive(Debug)]
    struct EntryStat {
        path: PathBuf,
        is_dir: bool,
        modified_time: i64,
        size: u64,
    }

    let mut executableFiles: Vec<PathBuf> = Vec::new();

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

        // check if it's unix executable file
        if !is_dir && ((meta.permissions().mode() >> 6) & 1) == 1 {
            executableFiles.push(entry.path());
        }

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
            // HashableIconType
            let iconType: Option<HashableIconType> = match entry.path.extension() {
                Some(extension) => {
                    // entry with Extensions
                    let extension = extension.to_str();

                    match extension {
                        Some(e) if e != "exe" => Some(HashableIconType::Ext(e.to_string())),
                        _ => None,
                    }
                }
                None => {
                    // entry without Extensions

                    // Unix Executable File
                    if !entry.is_dir && executableFiles.contains(&entry.path) {
                        Some(HashableIconType::UnixExecutable)
                    } else {
                        None
                    }
                }
            };

            let icon = match iconType {
                Some(i) => IconLoad::Hash(i),
                None => {
                    let icon = read_icon(&entry.path).ok();
                    IconLoad::Bytes(icon)
                }
            };

            Entry {
                is_dir: entry.is_dir,
                path: entry.path,
                modified_time: entry.modified_time,
                size: entry.size,
                icon,
            }
        })
        .collect();

    // icon cache (reduce repeated file operations)
    let icon_cache = Arc::new(Mutex::new(Some(HashMap::new())));
    let mut nonrepetitive_hashable_icon = Vec::new();

    entries
        .iter()
        .filter(|e| {
            if let IconLoad::Hash(_) = e.icon {
                if !nonrepetitive_hashable_icon.contains(&e.icon) {
                    nonrepetitive_hashable_icon.push(e.icon.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
        .par_bridge()
        .into_par_iter()
        .for_each(|entry| {
            let icon = read_icon(&entry.path).ok();
            if let IconLoad::Hash(ref hashable) = entry.icon {
                let mut guard = icon_cache.lock().unwrap();
                if let Some(ref mut guard) = &mut *guard {
                    (guard).insert(hashable.clone(), icon);
                }
            }
        });

    println!("icon cache: {:?}", icon_cache);
    let mut guard = icon_cache.lock().unwrap();
    if let Some(icon_cache) = guard.take() {
        Ok(Directory {
            path: path.into(),
            entries,
            icon_cache,
        })
    } else {
        Err(core_error!("icon cache is empty"))
    }
}
