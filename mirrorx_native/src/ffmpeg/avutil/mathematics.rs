use super::AVRational;

extern "C" {
    pub fn av_rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64;

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
