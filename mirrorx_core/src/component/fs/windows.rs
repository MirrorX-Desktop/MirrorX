use super::{Directory, Entry, IconType};
use crate::{core_error, error::CoreResult, HRESULT};
use image::ColorType;
use scopeguard::defer;
use std::{
    collections::HashMap,
    io::Cursor,
    os::raw::c_void,
    path::{Path, PathBuf},
    str::FromStr,
};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::SIZE,
        Graphics::Gdi::{
            DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        },
        Storage::FileSystem::GetLogicalDrives,
        UI::Shell::{IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_ICONONLY},
    },
};

static TRY_ICON_SIZES: [(i32, i32); 6] = [
    (64, 64),
    (128, 128),
    (48, 48),
    (32, 32),
    (256, 256),
    (16, 16),
];

pub fn read_root_directory() -> CoreResult<Directory> {
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

            let icon = read_icon(&path).ok();

            entries.push(Entry {
                is_dir: true,
                path,
                modified_time: 0,
                size: 0,
                icon: IconType::Bytes(icon),
            });
        }
    }

    Ok(Directory {
        path: PathBuf::from(r"\"),
        entries,
        hashed_icons: HashMap::new(),
    })
}

pub fn read_icon(path: &Path) -> CoreResult<Vec<u8>> {
    unsafe {
        let path = widestring::WideCString::from_os_str(path)
            .map_err(|err| core_error!("convert wide string failed ({})", err))?;

        let factory: IShellItemImageFactory = HRESULT!(SHCreateItemFromParsingName(
            PCWSTR::from_raw(path.as_ptr()),
            None
        ));

        let mut hbitmap = None;
        for (cx, cy) in TRY_ICON_SIZES {
            match factory.GetImage(SIZE { cx, cy }, SIIGBF_ICONONLY) {
                Ok(v) => {
                    hbitmap = Some(v);
                    break;
                }
                Err(_) => continue,
            }
        }

        let hbitmap = hbitmap.ok_or_else(|| core_error!("all sizes icon is empty"))?;

        defer! {
           let  _ = DeleteObject(hbitmap);
        }

        let dc = GetDC(None);
        let mut bitmap: BITMAP = std::mem::zeroed();
        if GetObjectW(
            hbitmap,
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
            hbitmap,
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

        ReleaseDC(None, dc);

        let bmp_bytes =
            std::slice::from_raw_parts_mut(color_bits.as_mut_ptr() as *mut u8, nbits * 4);

        // swap BGRA to RGBA
        for chunk in bmp_bytes.chunks_mut(4) {
            chunk.swap(0, 2)
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
