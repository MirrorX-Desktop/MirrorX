#[repr(C)]
#[derive(Debug, Clone)]
pub struct AVRational {
    pub num: i32,
    pub den: i32,
}

impl Copy for AVRational {}

extern "C" {
    // pub fn av_inv_q(q: AVRational) -> AVRational;
}
