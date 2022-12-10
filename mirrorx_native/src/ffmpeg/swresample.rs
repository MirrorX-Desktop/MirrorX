pub enum SwrContext {}

extern "C" {
    pub fn swr_alloc() -> *mut SwrContext;
    pub fn swr_init(s: *mut SwrContext) -> i32;
    pub fn swr_free(s: *mut *mut SwrContext);
    pub fn swr_close(s: *mut *mut SwrContext);
    pub fn swr_convert(
        s: *mut SwrContext,
        out: *mut *mut u8,
        out_count: i32,
        in_: *const *const u8,
        in_count: i32,
    ) -> i32;
    pub fn swr_get_delay(s: *mut SwrContext, base: i64) -> i64;
}
