use core_foundation::string::CFStringRef;

extern "C" {
    pub fn CMTimeMake(value: i64, time_scale: i32) -> CMTime;
}

pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub type CMTimeFlags = u32;

#[repr(C)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub time_scale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

extern "C" {
    pub static kCVPixelBufferPixelFormatTypeKey: CFStringRef;
    pub static kCVPixelBufferWidthKey: CFStringRef;
    pub static kCVPixelBufferHeightKey: CFStringRef;
}

#[allow(non_upper_case_globals)]
pub static kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange: i32 = 875704438;

#[allow(non_upper_case_globals)]
pub static kCVPixelFormatType_420YpCbCr8BiPlanarFullRange: i32 = 875704422;
