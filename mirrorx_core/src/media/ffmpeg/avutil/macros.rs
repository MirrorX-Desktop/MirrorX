#[allow(non_snake_case)]
pub const fn MKTAG(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a | b << 8 | c << 16 | d << 24
}

#[allow(non_snake_case)]
pub const fn FFERRTAG(a: char, b: char, c: char, d: char) -> i32 {
    -(MKTAG(a as u32, b as u32, c as u32, d as u32) as i32)
}
