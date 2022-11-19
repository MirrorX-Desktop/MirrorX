use std::ffi::c_void;

pub type AVChannelOrder = i32;
pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;

#[repr(C)]
pub union AVChannelLayout_u {
    pub mask: u64,
    pub map: *const c_void,
}

#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: i32,
    pub u: AVChannelLayout_u,
    pub opaque: *mut c_void,
}

extern "C" {
    pub fn av_channel_layout_check(channel_layout: *const AVChannelLayout) -> i32;
}
