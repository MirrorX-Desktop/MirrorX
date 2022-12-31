use crate::error::CoreResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub sub_dirs: Vec<DirEntry>,
    pub files: Vec<FileEntry>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub modified_time: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub modified_time: i64,
    pub size: u64,
    #[serde(with = "serde_bytes")]
    pub icon: Option<Vec<u8>>,
}

#[cfg(not(target_os = "windows"))]
pub fn read_root_directory() -> CoreResult<Directory> {
    read_directory("/")
}

#[cfg(target_os = "windows")]
pub fn read_root_directory() -> CoreResult<Directory> {
    use std::str::FromStr;
    use windows::Win32::Storage::FileSystem::GetLogicalDrives;

    let mut sub_dirs = Vec::new();

    unsafe {
        let mask = GetLogicalDrives();
        for i in 0..u32::BITS {
            if (mask >> i) & 1 == 0 {
                continue;
            }

            let disk = [b'A' + i as u8, b':', b'\\'];

            sub_dirs.push(DirEntry {
                path: PathBuf::from_str(String::from_utf8_lossy(&disk).as_ref())?,
                modified_time: 0,
            });
        }
    }

    Ok(Directory {
        path: PathBuf::from_str("\\")?,
        sub_dirs,
        files: Vec::new(),
    })
}

pub fn read_directory<P>(path: P) -> CoreResult<Directory>
where
    P: AsRef<Path> + Into<PathBuf>,
{
    let dir = std::fs::read_dir(&path)?;

    let mut sub_dirs = Vec::new();
    let mut files = Vec::new();

    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let meta = entry.metadata()?;
        let modified_time = chrono::DateTime::<chrono::Local>::from(meta.modified()?)
            .naive_utc()
            .timestamp();

        if file_type.is_dir() {
            sub_dirs.push(DirEntry {
                path: entry.path(),
                modified_time,
            });
        } else {
            files.push(FileEntry {
                path: entry.path(),
                modified_time,
                size: meta.len(),
                icon: None,
            });
        }
    }

    Ok(Directory {
        path: path.into(),
        sub_dirs,
        files,
    })
}
