#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub mod transfer;

use crate::error::CoreResult;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    ffi::OsStr,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use math

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
    pub icon: IconType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum IconType {
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

        // check if it's unix executable file
        // TODO mode() -> base 10 to base 8
        if meta.permissions().mode() % 1000 / 100 % 2 == 1 {
            executableFiles.push(entry.path());
        }

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

    // icon cache (reduce repeated file operations)
    let icon_cache = HashMap::new();

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

            println!("path: {:?}, iconType: {:?}", entry.path, iconType);

            let icon = match iconType {
                Some(i) => IconType::Hash(i),
                None => {
                    let icon = read_icon(&entry.path).ok();
                    IconType::Bytes(icon)
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

    Ok(Directory {
        path: path.into(),
        entries,
        icon_cache,
    })
}

#[cfg(test)]
mod tests {
    // use super::read_directory;

    // #[test]
    // fn check_extension() {
    // _ = read_directory("/Users/fujianbang/Downloads");
    // }

    #[test]
    fn check_permission() {
        let perm: u32 = 100755;

        println!("{:b}", perm);
    }
}
