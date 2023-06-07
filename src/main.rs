use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

#[pin_project::pin_project]
struct TimedFuture<F> {
    #[pin]
    f: F,
    start: Option<Instant>,
}

impl<F> TimedFuture<F> {
    pub fn new(f: F) -> Self {
        Self { f, start: None }
    }
}

impl<F: Future> Future for TimedFuture<F> {
    type Output = (F::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if this.start.is_none() {
            *this.start = Some(Instant::now());
        }

        match this.f.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                let elapsed = this.start.unwrap().elapsed();
                Poll::Ready((result, elapsed))
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
