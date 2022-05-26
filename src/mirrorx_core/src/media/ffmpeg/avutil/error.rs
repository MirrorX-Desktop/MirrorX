use super::macros::FFERRTAG;

#[allow(non_snake_case)]
pub const fn AVERROR(e: i32) -> i32 {
    -(e)
}

pub const AVERROR_EOF: i32 = FFERRTAG('E', 'O', 'F', ' ');
