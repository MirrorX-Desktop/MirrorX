use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Future, TryFutureExt};
use tokio::sync::oneshot::Receiver;

pub struct TransporterFuture {
    rx: Receiver<(u16, Vec<u8>)>,
}

impl TransporterFuture {
    pub fn new(rx: Receiver<(u16, Vec<u8>)>) -> TransporterFuture {
        TransporterFuture { rx }
    }
}

impl Future for TransporterFuture {
    type Output = anyhow::Result<(u16, Vec<u8>)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(res) = self.rx.try_poll_unpin(cx) {
            return Poll::Ready(res.or_else(|err| Err(anyhow::anyhow!(err))));
        }

        return std::task::Poll::Pending;
    }
}
