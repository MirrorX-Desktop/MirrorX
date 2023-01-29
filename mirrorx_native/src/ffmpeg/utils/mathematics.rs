use super::rational::AVRational;

pub type AVRounding = i32;

pub const AV_ROUND_ZERO: AVRounding = 0;
pub const AV_ROUND_INF: AVRounding = 1;
pub const AV_ROUND_DOWN: AVRounding = 2;
pub const AV_ROUND_UP: AVRounding = 3;
pub const AV_ROUND_NEAR_INF: AVRounding = 5;
pub const AV_ROUND_PASS_MINMAX: AVRounding = 8192;

extern "C" {
    pub fn av_rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64;
    pub fn av_rescale_rnd(a: i64, b: i64, c: i64, rnd: AVRounding) -> i64;
}

#[inline]
pub const fn av_inv_q(q: AVRational) -> AVRational {
    AVRational {
        num: q.den,
        den: q.num,
    }
}

#[inline]
pub fn av_q2d(a: AVRational) -> f64 {
    (a.num as f64) / (a.den as f64)
}
