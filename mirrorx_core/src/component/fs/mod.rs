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
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub hashed_icons: HashMap<HashedIcon, Option<Vec<u8>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Entry {
    pub is_dir: bool,
    pub path: PathBuf,
    pub modified_time: i64,
    pub size: u64,
    pub icon: IconType,
}

#[derive(Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum IconType {
    Hashed(HashedIcon),
    #[serde(with = "serde_bytes")]
    Bytes(Option<Vec<u8>>),
}

#[derive(Hash, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum HashedIcon {
    Ext(String),    // File with extension except .exe
    UnixExecutable, // Unix Executable File
    OrdinaryDir,    // Ordinary Directory
}

impl From<HashedIcon> for String {
    fn from(value: HashedIcon) -> Self {
        match value {
            HashedIcon::Ext(ext) => format!(".{ext}"),
            HashedIcon::UnixExecutable => String::from("UnixExecutable"),
            HashedIcon::OrdinaryDir => String::from("OrdinaryDir"),
        }
    }
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

    let dir = std::fs::read_dir(&path)?;
    let mut entry_stats = Vec::new();

    #[cfg(not(target_os = "windows"))]
    let mut unix_executable_files = Vec::new();

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
        #[cfg(not(target_os = "windows"))]
        if !is_dir {
            use std::os::unix::prelude::PermissionsExt;
            if ((&meta.permissions().mode() >> 6) & 1) == 1 {
                unix_executable_files.push(entry.path());
            }
        }

        let modified_time = chrono::DateTime::<chrono::Local>::from(meta.modified()?)
            .naive_utc()
            .timestamp();

        entry_stats.push(EntryStat {
            path: entry.path(),
            is_dir,
            modified_time,
            size: meta.len(),
        });
    }

    let entries: Vec<Entry> = entry_stats
        .into_par_iter()
        .map(|entry| {
            let hashed_icon = match entry.path.extension() {
                Some(extension) => {
                    // entry with Extensions
                    let extension = extension.to_str();

                    match extension {
                        Some(e) if e != "exe" && e != "app" && e != "dmg" => {
                            Some(HashedIcon::Ext(e.to_string()))
                        }
                        _ => None,
                    }
                }
                None => {
                    // Unix Executable File
                    #[cfg(not(target_os = "windows"))]
                    if !entry.is_dir && unix_executable_files.contains(&entry.path) {
                        Some(HashedIcon::UnixExecutable)
                    } else {
                        None
                    }

                    #[cfg(target_os = "windows")]
                    None
                }
            };

            let icon_type = match hashed_icon {
                Some(hashed) => IconType::Hashed(hashed),
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
                icon: icon_type,
            }
        })
        .collect();

    let mut hashed_icon_entities: Vec<(&Path, HashedIcon, Option<Vec<u8>>)> = Vec::new();

    // filter unique hashed icon type
    for entry in entries.iter() {
        if let IconType::Hashed(ref hashed_icon) = entry.icon {
            if hashed_icon_entities
                .iter()
                .all(|(_, element_hashed_icon, _)| element_hashed_icon != hashed_icon)
            {
                hashed_icon_entities.push((&entry.path, hashed_icon.clone(), None));
            }
        }
    }

    // parallel read icon data
    hashed_icon_entities
        .par_iter_mut()
        .for_each(|(path, _, icon_bytes)| {
            (*icon_bytes) = read_icon(path).ok();
        });

    let hashed_icons = hashed_icon_entities
        .into_iter()
        .map(|(_, hashed_icon, icon_bytes)| (hashed_icon, icon_bytes))
        .collect();

    Ok(Directory {
        path: path.into(),
        entries,
        hashed_icons,
    })
}
