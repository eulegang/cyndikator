use chrono::{DateTime, Local};
use futures::{
    stream::{FusedStream, Stream},
    task::{Context, Poll},
};
use std::{
    pin::Pin,
    thread,
    time::{Duration, Instant},
};

pub struct Ticker {
    expire: Option<Instant>,
    dur: Duration,
}

impl Ticker {
    pub fn new(dur: Duration) -> Ticker {
        let expire = None;

        Ticker { dur, expire }
    }
}

impl Stream for Ticker {
    type Item = DateTime<Local>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<DateTime<Local>>> {
        let expire = self.expire.unwrap_or_else(|| Instant::now() + self.dur);

        self.expire = Some(expire);

        if expire <= Instant::now() {
            Poll::Ready(Some(Local::now()))
        } else {
            let waker = ctx.waker().clone();

            thread::spawn(move || {
                let now = Instant::now();

                if now < expire {
                    thread::sleep(expire - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}

impl FusedStream for Ticker {
    fn is_terminated(&self) -> bool {
        false
    }
}
