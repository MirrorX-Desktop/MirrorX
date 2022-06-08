pub type AVFieldOrder = u32;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_TT: AVFieldOrder = 2; //< Top coded_first, top displayed first
pub const AV_FIELD_BB: AVFieldOrder = 3; //< Bottom coded first, bottom displayed first
pub const AV_FIELD_TB: AVFieldOrder = 4; //< Top coded first, bottom displayed first
pub const AV_FIELD_BT: AVFieldOrder = 5; //< Bottom coded first, top displayed first
