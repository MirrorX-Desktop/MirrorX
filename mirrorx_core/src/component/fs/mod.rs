#[cfg(target_os = "macos")]
mod macos;

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

#[cfg(not(target_os = "windows"))]
pub fn read_root_directory() -> CoreResult<Directory> {
    read_directory("/")
}

#[cfg(target_os = "windows")]
pub fn read_root_directory() -> CoreResult<Directory> {
    use std::str::FromStr;
    use windows::Win32::Storage::FileSystem::GetLogicalDrives;

    let mut entries = Vec::new();

    unsafe {
        let mask = GetLogicalDrives();
        for i in 0..u32::BITS {
            if (mask >> i) & 1 == 0 {
                continue;
            }

            let disk = [b'A' + i as u8, b':', b'\\'];
            let disk_str = std::str::from_utf8_unchecked(&disk);
            let path = PathBuf::from_str(disk_str)?;

            let icon = read_icon(&path).map_or(None, |v| Some(v));

            entries.push(Entry {
                is_dir: true,
                path,
                modified_time: 0,
                size: 0,
                icon,
            });
        }
    }

    Ok(Directory {
        path: PathBuf::from(r"\"),
        entries,
    })
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

#[cfg(not(target_os = "windows"))]
fn read_icon(path: &Path) -> CoreResult<Vec<u8>> {
    self::macos::NSWorkspace::sharedWorkspace()?.iconForFile(path)
}

#[cfg(target_os = "windows")]
fn read_icon(path: &Path) -> CoreResult<Vec<u8>> {
    use crate::core_error;
    use image::ColorType;
    use scopeguard::defer;
    use std::{io::Cursor, os::raw::c_void};
    use windows::{
        core::PCWSTR,
        Win32::{
            Graphics::Gdi::{
                DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO,
                BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
            },
            Storage::FileSystem::FILE_ATTRIBUTE_NORMAL,
            UI::{
                Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON},
                WindowsAndMessaging::{DestroyIcon, GetIconInfo, ICONINFO},
            },
        },
    };

    unsafe {
        let path = widestring::WideCString::from_os_str(path)
            .map_err(|err| core_error!("convert wide string failed ({})", err))?;

        let mut psfi: SHFILEINFOW = std::mem::zeroed();

        let ret = SHGetFileInfoW(
            PCWSTR::from_raw(path.as_ptr()),
            FILE_ATTRIBUTE_NORMAL,
            Some(&mut psfi),
            std::mem::size_of::<SHFILEINFOW>() as _,
            SHGFI_ICON | SHGFI_LARGEICON,
        );

        if ret == 0 {
            return Err(core_error!("SHGetFileInfoW failed"));
        }

        defer! {
            let _ = DestroyIcon(psfi.hIcon);
        }

        let mut icon_info: ICONINFO = std::mem::zeroed();
        if !GetIconInfo(psfi.hIcon, &mut icon_info).as_bool() {
            return Err(core_error!("GetIconInfo failed"));
        }

        let dc = GetDC(None);
        let mut bitmap: BITMAP = std::mem::zeroed();
        if GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as _,
            Some(&mut bitmap as *mut _ as *mut c_void),
        ) == 0
        {
            return Err(core_error!("GetObjectW failed"));
        }

        let mut bitmap_info: BITMAPINFO = std::mem::zeroed();
        bitmap_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as _;
        bitmap_info.bmiHeader.biWidth = bitmap.bmWidth;
        bitmap_info.bmiHeader.biHeight = -bitmap.bmHeight;
        bitmap_info.bmiHeader.biPlanes = 1;
        bitmap_info.bmiHeader.biBitCount = 32;
        bitmap_info.bmiHeader.biCompression = BI_RGB;

        let nbits = (bitmap.bmWidth * bitmap.bmHeight) as usize;
        let mut color_bits = Vec::<u32>::with_capacity(nbits);
        if GetDIBits(
            dc,
            icon_info.hbmColor,
            0,
            bitmap.bmHeight as _,
            Some(color_bits.as_mut_ptr() as *mut c_void),
            &mut bitmap_info,
            DIB_RGB_COLORS,
        ) == 0
        {
            return Err(core_error!("GetDIBits failed"));
        }

        color_bits.set_len(nbits);

        let mut has_alpha = false;
        for i in 0..nbits {
            if color_bits[i] & 0xFF000000 != 0 {
                has_alpha = true;
                break;
            }
        }

        if !has_alpha {
            let mut mask_bits = Vec::<u32>::with_capacity(nbits);
            if GetDIBits(
                dc,
                icon_info.hbmMask,
                0,
                bitmap.bmHeight as _,
                Some(mask_bits.as_mut_ptr() as *mut c_void),
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) == 0
            {
                return Err(core_error!("GetDIBits failed"));
            }
            mask_bits.set_len(nbits);

            for i in 0..nbits {
                if mask_bits[i] == 0 {
                    color_bits[i] |= 0xFF000000;
                }
            }
        }

        ReleaseDC(None, dc);
        DeleteObject(icon_info.hbmColor);
        DeleteObject(icon_info.hbmMask);

        let bmp_bytes =
            std::slice::from_raw_parts_mut(color_bits.as_mut_ptr() as *mut u8, nbits * 4);

        // swap BGRA to RGBA
        for chunk in bmp_bytes.chunks_mut(4).into_iter() {
            chunk[0] = chunk[0] ^ chunk[2];
            chunk[2] = chunk[0] ^ chunk[2];
            chunk[0] = chunk[0] ^ chunk[2];
        }

        let mut png_bytes: Vec<u8> = Vec::with_capacity(nbits * 4);

        if let Err(err) = image::write_buffer_with_format(
            &mut Cursor::new(&mut png_bytes),
            bmp_bytes,
            bitmap.bmWidth as u32,
            bitmap.bmHeight as u32,
            ColorType::Rgba8,
            image::ImageOutputFormat::Png,
        ) {
            return Err(core_error!(
                "write desktop screenshot image buffer failed ({})",
                err
            ));
        }

        Ok(png_bytes)
    }
}
