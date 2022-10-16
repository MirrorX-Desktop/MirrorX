use futures::future::BoxFuture;
use mirrorx_core::error::{CoreError, CoreResult};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct PromiseValue<T: Send + 'static> {
    value: Option<T>,
    err: Option<CoreError>,
    tx: Sender<CoreResult<T>>,
    rx: Receiver<CoreResult<T>>,
    could_spawn: bool,
}

impl<T: Send + 'static> PromiseValue<T> {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        Self {
            value: None,
            err: None,
            tx,
            rx,
            could_spawn: true,
        }
    }

    pub fn spawn_update(
        &mut self,
        f: impl std::future::Future<Output = CoreResult<T>> + 'static + Send,
    ) {
        if self.could_spawn {
            let tx = self.tx.clone();
            tokio::spawn(async move {
                if let Err(err) = tx.send(f.await).await {
                    // this error shouldn't be happen because of PromiseValue designed logic,
                    // when it happened, there must be something wrong
                    panic!("PromiseValue send async result failed ({})", err);
                }
            });
            self.could_spawn = false;
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub fn take_value(&mut self) -> Option<T> {
        self.value.take()
    }

    pub fn error(&self) -> Option<&CoreError> {
        self.err.as_ref()
    }

    pub fn take_error(&mut self) -> Option<CoreError> {
        self.err.take()
    }

    pub fn poll(&mut self) {
        match self.rx.try_recv() {
            Ok(value) => {
                match value {
                    Ok(value) => self.value = Some(value),
                    Err(err) => self.err = Some(err),
                };
                self.could_spawn = true;
            }
            Err(err) => {
                if err == tokio::sync::mpsc::error::TryRecvError::Disconnected {
                    // this error shouldn't be happen because of PromiseValue designed logic,
                    // when it happened, there must be something wrong
                    panic!("PromiseValue inner value channel is disconnected")
                }
            }
        }
    }
}

type OneWayUpdatePromiseUpdateFn<T> = Box<dyn Fn(&mut PromiseValue<T>)>;

pub struct OneWayUpdatePromiseValue<T: Send + 'static> {
    promise_value: PromiseValue<T>,
    update_fn: OneWayUpdatePromiseUpdateFn<T>,
}

impl<T: Send + 'static> OneWayUpdatePromiseValue<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> BoxFuture<'static, CoreResult<T>> + 'static,
    {
        let update_fn = move |pv: &mut PromiseValue<T>| {
            let future = (f)();
            pv.spawn_update(future);
        };

        Self {
            promise_value: PromiseValue::new(),
            update_fn: Box::new(update_fn),
        }
    }

    pub fn update(&mut self) {
        (self.update_fn)(&mut self.promise_value)
    }

    pub fn value(&self) -> Option<&T> {
        self.promise_value.value()
    }

    pub fn take_value(&mut self) -> Option<T> {
        self.promise_value.take_value()
    }

    pub fn error(&self) -> Option<&CoreError> {
        self.promise_value.error()
    }

    pub fn take_error(&mut self) -> Option<CoreError> {
        self.promise_value.take_error()
    }

    pub fn poll(&mut self) {
        self.promise_value.poll()
    }
}
