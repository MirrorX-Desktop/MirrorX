use futures::Future;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

pub struct RuntimeProvider {
    runtime: Runtime,
}

impl RuntimeProvider {
    pub fn new() -> anyhow::Result<Self> {
        let runtime = Builder::new_multi_thread()
            .thread_name("MirrorXCoreTokioRuntime")
            .enable_all()
            .build()?;

        Ok(RuntimeProvider { runtime })
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn spawn_blocking<F, R>(&self, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.runtime.spawn_blocking(func)
    }
}
