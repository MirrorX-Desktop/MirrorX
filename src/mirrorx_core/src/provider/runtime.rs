use anyhow::bail;
use futures::Future;
use once_cell::sync::OnceCell;
use tokio::{
    runtime::{Builder, Runtime},
    task::JoinHandle,
};

static CURRENT_RUNTIME_PROVIDER: OnceCell<RuntimeProvider> = OnceCell::new();

pub struct RuntimeProvider {
    runtime: Runtime,
}

impl RuntimeProvider {
    pub fn current() -> anyhow::Result<&'static RuntimeProvider> {
        CURRENT_RUNTIME_PROVIDER
            .get()
            .ok_or_else(|| anyhow::anyhow!("RuntimeProvider: uninitialized"))
    }

    pub fn make_current() -> anyhow::Result<()> {
        match CURRENT_RUNTIME_PROVIDER.get_or_try_init(|| -> anyhow::Result<RuntimeProvider> {
            let runtime = Builder::new_multi_thread()
                .thread_name("MirrorXCoreTokioRuntime")
                .enable_all()
                .build()?;

            let provider = RuntimeProvider { runtime };

            Ok(provider)
        }) {
            Ok(_) => Ok(()),
            Err(err) => bail!("RuntimeProvider: make current failed: {}", err),
        }
    }

    #[inline(always)]
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    #[inline(always)]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}
