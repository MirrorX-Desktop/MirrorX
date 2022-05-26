use super::macros::FFERRTAG;

#[allow(non_snake_case)]
pub const fn AVERROR(e: i32) -> i32 {
    -(e)
}

/// Bitstream filter not found
pub const AVERROR_BSF_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'B', 'S', 'F');

/// Internal bug, also see AVERROR_BUG2
pub const AVERROR_BUG: i32 = FFERRTAG('B', 'U', 'G', '!');

/// Buffer too small
pub const AVERROR_BUFFER_TOO_SMALL: i32 = FFERRTAG('B', 'U', 'F', 'S');

/// Decoder not found
pub const AVERROR_DECODER_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'D', 'E', 'C');

/// Demuxer not found
pub const AVERROR_DEMUXER_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'D', 'E', 'M');

/// Encoder not found
pub const AVERROR_ENCODER_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'E', 'N', 'C');

/// End of file
pub const AVERROR_EOF: i32 = FFERRTAG('E', 'O', 'F', ' ');

/// Immediate exit was requested; the called function should not be restarted
pub const AVERROR_EXIT: i32 = FFERRTAG('E', 'X', 'I', 'T');

/// Generic error in an external library
pub const AVERROR_EXTERNAL: i32 = FFERRTAG('E', 'X', 'T', ' ');

/// Filter not found
pub const AVERROR_FILTER_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'F', 'I', 'L');

/// Invalid data found when processing input
pub const AVERROR_INVALIDDATA: i32 = FFERRTAG('I', 'N', 'D', 'A');

/// Muxer not found
pub const AVERROR_MUXER_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'M', 'U', 'X');

/// Option not found
pub const AVERROR_OPTION_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'O', 'P', 'T');

/// Not yet implemented in FFmpeg, patches welcome
pub const AVERROR_PATCHWELCOME: i32 = FFERRTAG('P', 'A', 'W', 'E');

/// Protocol not found
pub const AVERROR_PROTOCOL_NOT_FOUND: i32 = FFERRTAG(0xF8 as char, 'P', 'R', 'O');
