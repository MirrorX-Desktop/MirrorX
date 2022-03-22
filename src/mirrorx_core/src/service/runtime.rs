use lazy_static::lazy_static;

static mut INNER_RUNTIME: Option<tokio::runtime::Runtime> = None;

lazy_static! {
    pub static ref RUNTIME: &'static tokio::runtime::Runtime =
        unsafe { INNER_RUNTIME.as_ref().unwrap() };
}

pub fn init_async_runtime() -> anyhow::Result<()> {
    unsafe {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .thread_name("MirrorXCoreTokioRuntime")
            .enable_all()
            .build()?;

        INNER_RUNTIME = Some(rt);
        Ok(())
    }
}
