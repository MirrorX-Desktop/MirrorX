use crate::api_error::APIError;
use lazy_static::lazy_static;
use log::error;

static mut INNER_RUNTIME: Option<tokio::runtime::Runtime> = None;

lazy_static! {
    pub static ref RUNTIME: &'static tokio::runtime::Runtime =
        unsafe { INNER_RUNTIME.as_ref().unwrap() };
}

pub fn init_async_runtime() -> anyhow::Result<(), APIError> {
    unsafe {
        let rt = match tokio::runtime::Builder::new_multi_thread()
            .thread_name("MirrorXCoreTokioRuntime")
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(err) => {
                error!("init runtime error: {}", err);
                return Err(APIError::InternalError);
            }
        };

        INNER_RUNTIME = Some(rt);
        Ok(())
    }
}
