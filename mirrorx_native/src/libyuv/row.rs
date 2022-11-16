#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
#[repr(C)]
#[allow(non_snake_case)]
pub struct YuvConstants {
    kUVCoeff: [u8; 16],
    kRGBCoeffBias: [i16; 8],
}

#[cfg(all(not(target_arch = "aarch64"), not(target_arch = "arm")))]
#[repr(C)]
#[allow(non_snake_case)]
pub struct YuvConstants {
    kUVToB: [u8; 32],
    kUVToG: [u8; 32],
    kUVToR: [u8; 32],
    kYToRgb: [u8; 16],
    kYBiasToRgb: [u8; 16],
}
