use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::sync::mpsc::Sender;

use crate::factory::frame::Frame;

#[allow(non_camel_case_types)]
#[repr(C)]
/// cbindgen:ignore
pub enum AVHWDeviceType {
    AV_HWDEVICE_TYPE_NONE,
    AV_HWDEVICE_TYPE_VDPAU,
    AV_HWDEVICE_TYPE_CUDA,
    AV_HWDEVICE_TYPE_VAAPI,
    AV_HWDEVICE_TYPE_DXVA2,
    AV_HWDEVICE_TYPE_QSV,
    AV_HWDEVICE_TYPE_VIDEOTOOLBOX,
    AV_HWDEVICE_TYPE_D3D11VA,
    AV_HWDEVICE_TYPE_DRM,
    AV_HWDEVICE_TYPE_OPENCL,
    AV_HWDEVICE_TYPE_MEDIACODEC,
    AV_HWDEVICE_TYPE_VULKAN,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
/// cbindgen:ignore
pub enum AVPixelFormat {
    AV_PIX_FMT_NONE = -1,
    AV_PIX_FMT_YUV420P = 0,
    AV_PIX_FMT_NV12 = 23,
    AV_PIX_FMT_NV21 = 24,
}

/// cbindgen:ignore
extern "C" {
    pub fn new_video_decoder(
        decoder_name: *const c_char,
        device_type: AVHWDeviceType,
        encode_callback: unsafe extern "C" fn(
            tx: *mut Sender<Frame>,
            width: c_int,
            height: c_int,
            pix_fmt: AVPixelFormat,
            plane_linesize: *const c_int,
            plane_buffer_address: *const *const u8,
        ),
    ) -> *const c_void;

    pub fn video_decode(
        video_decoder: *const c_void,
        tx: *mut Sender<Frame>,
        packet_data: *const u8,
        packet_size: c_int,
    ) -> c_int;

    pub fn free_video_decoder(video_decoder: *const c_void);
}
