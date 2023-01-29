use super::{buffer::AVBufferRef, frame::AVFrame};
use std::{ffi::c_void, os::raw::c_char};

pub type AVHWDeviceType = u32;
pub const AV_HWDEVICE_TYPE_NONE: AVHWDeviceType = 0;
pub const AV_HWDEVICE_TYPE_VDPAU: AVHWDeviceType = 1;
pub const AV_HWDEVICE_TYPE_CUDA: AVHWDeviceType = 2;
pub const AV_HWDEVICE_TYPE_VAAPI: AVHWDeviceType = 3;
pub const AV_HWDEVICE_TYPE_DXVA2: AVHWDeviceType = 4;
pub const AV_HWDEVICE_TYPE_QSV: AVHWDeviceType = 5;
pub const AV_HWDEVICE_TYPE_VIDEOTOOLBOX: AVHWDeviceType = 6;
pub const AV_HWDEVICE_TYPE_D3D11VA: AVHWDeviceType = 7;
pub const AV_HWDEVICE_TYPE_DRM: AVHWDeviceType = 8;
pub const AV_HWDEVICE_TYPE_OPENCL: AVHWDeviceType = 9;
pub const AV_HWDEVICE_TYPE_MEDIACODEC: AVHWDeviceType = 10;
pub const AV_HWDEVICE_TYPE_VULKAN: AVHWDeviceType = 11;

extern "C" {
    pub fn av_hwdevice_find_type_by_name(name: *const c_char) -> AVHWDeviceType;
    pub fn av_hwframe_transfer_data(dst: *mut AVFrame, src: *const AVFrame, flags: i32) -> i32;
    pub fn av_hwdevice_iterate_types(prev: AVHWDeviceType) -> AVHWDeviceType;
    pub fn av_hwdevice_get_type_name(type_: AVHWDeviceType) -> *const c_char;
    pub fn av_hwdevice_ctx_create(
        device_ctx: *mut *mut AVBufferRef,
        type_: AVHWDeviceType,
        device: *const c_char,
        opts: *mut c_void,
        flags: i32,
    ) -> i32;
}
