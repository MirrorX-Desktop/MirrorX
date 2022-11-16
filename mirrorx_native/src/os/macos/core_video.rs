use super::{four_char_code, io_surface::IOSurfaceRef};
use core_foundation::{
    base::CFTypeRef, dictionary::CFDictionaryRef, mach_port::CFAllocatorRef, string::CFStringRef,
};
use std::os::raw::c_void;

pub static kCVPixelFormatType_420YpCbCr8Planar: u32 = four_char_code('y', '4', '2', '0');
pub static kCVPixelFormatType_420YpCbCr8PlanarFullRange: u32 = four_char_code('f', '4', '2', '0');

pub static kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange: u32 =
    four_char_code('4', '2', '0', 'v');
pub static kCVPixelFormatType_420YpCbCr8BiPlanarFullRange: u32 = four_char_code('4', '2', '0', 'f');
pub static kCVPixelFormatType_32BGRA: u32 = four_char_code('B', 'G', 'R', 'A');

pub type CVImageBufferRef = *mut c_void;
pub type CVPixelBufferRef = CVImageBufferRef;

extern "C" {
    pub static kCVPixelBufferPixelFormatTypeKey: CFStringRef;
    pub static kCVPixelBufferWidthKey: CFStringRef;
    pub static kCVPixelBufferHeightKey: CFStringRef;
    pub static kCVImageBufferYCbCrMatrixKey: CFStringRef;
    pub static kCVPixelBufferMetalCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferOpenGLCompatibilityKey: CFStringRef;

    pub static kCVImageBufferYCbCrMatrix_ITU_R_601_4: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_709_2: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_ITU_R_2020: CFStringRef;
    pub static kCVImageBufferYCbCrMatrix_SMPTE_240M_1995: CFStringRef;
}

extern "C" {
    pub fn CVPixelBufferGetPixelFormatType(pixel_buffer: CVPixelBufferRef) -> u32;
    pub fn CVPixelBufferLockBaseAddress(pixel_buffer: CVPixelBufferRef, lock_flags: u32) -> i32;
    pub fn CVPixelBufferUnlockBaseAddress(pixel_buffer: CVPixelBufferRef, unlock_flags: u32)
        -> i32;
    pub fn CVPixelBufferGetWidth(pixel_buffer: CVPixelBufferRef) -> usize;
    pub fn CVPixelBufferGetHeight(pixel_buffer: CVPixelBufferRef) -> usize;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixel_buffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> usize;
    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixel_buffer: CVPixelBufferRef,
        planeIndex: usize,
    ) -> *mut c_void;
    pub fn CVPixelBufferGetHeightOfPlane(pixel_buffer: CVPixelBufferRef, planeIndex: u32) -> u32;
    pub fn CVPixelBufferRetain(texture: CVPixelBufferRef) -> CVPixelBufferRef;
    pub fn CVPixelBufferRelease(texture: CVPixelBufferRef);
    pub fn CVBufferGetAttachment(
        buffer: *mut c_void,
        key: CFStringRef,
        attachmentMode: *mut c_void,
    ) -> CFTypeRef;

    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixel_buffer_attributes: CFDictionaryRef,
        pixel_buffer_out: *mut CVPixelBufferRef,
    ) -> i32;
}
