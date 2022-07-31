pub mod core_foundation;
pub mod core_graphics;
pub mod core_media;
pub mod core_video;
pub mod io_surface;

const fn four_char_code(a: char, b: char, c: char, d: char) -> u32 {
    (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | (d as u32)
}
