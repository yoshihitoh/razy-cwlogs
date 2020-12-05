use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::{ready, StreamExt};
use tokio::stream::Stream;
use tokio::time::Interval;

pub struct TickStream {
    inner: Interval,
}

pub fn tick_stream(interval: Duration) -> TickStream {
    let inner = tokio::time::interval(interval);
    TickStream { inner }
}

impl Stream for TickStream {
    type Item = Instant;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let inst = ready!(self.inner.poll_next_unpin(cx));
        Poll::Ready(inst.map(|i| i.into_std()))
    }
}
