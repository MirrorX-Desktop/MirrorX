use futures::{Future, TryFutureExt};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::oneshot::Receiver;

use super::proto::ProtoMessage;

pub struct CallFuture {
    rx: Receiver<Box<dyn ProtoMessage>>,
}

impl CallFuture {
    pub fn new(rx: Receiver<Box<dyn ProtoMessage>>) -> CallFuture {
        CallFuture { rx }
    }
}

impl Future for CallFuture {
    type Output = anyhow::Result<Box<dyn ProtoMessage>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(res) = self.rx.try_poll_unpin(cx) {
            return Poll::Ready(res.or_else(|err| Err(anyhow::anyhow!(err))));
        }

        return std::task::Poll::Pending;
    }
}
