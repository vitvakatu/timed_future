use std::{pin::Pin, task::Poll, time::Duration};

use futures::Future;
use tokio::time::Instant;

struct TimedFuture<F> {
    future: F,
    start: Instant,
}

impl<F: Future> TimedFuture<F> {
    fn new(future: F) -> Self {
        Self {
            future,
            start: Instant::now(),
        }
    }

    #[inline]
    fn pin_get_future(self: Pin<&mut Self>) -> Pin<&mut F> {
        // This is okay because `future` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|s| &mut s.future) }
    }
}

impl<F: Future> Future for TimedFuture<F> {
    type Output = (F::Output, Duration);

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let start = self.start;
        match self.pin_get_future().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(t) => {
                // print!("It took {}", self.start.duration_since(earlier))
                Poll::Ready((t, start.elapsed()))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let f = reqwest::get("https://rust-lang.org");
    let time_it = TimedFuture::new(f);
    let (_result, time_spent) = time_it.await;
    println!("Request took {} ms", time_spent.as_millis());
}
