use super::AVRational;

extern "C" {
    pub fn av_rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64;
}
