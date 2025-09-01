use std::{
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

use tokio_stream::Stream;
use tokio_stream::StreamExt;

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("time is over");
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            // 模拟后台等待事件就绪，就绪后通知继续执行
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    println!("do something slow....");
                    thread::sleep(when - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}

struct Interval {
    rem: usize,
    delay: Delay,
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        if self.rem == 0 {
            // 去除计时器实现
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    let mut interval = Interval {
        rem: 3,
        delay: Delay {
            when: Instant::now() + Duration::from_secs(5),
        },
    };

    while let Some(_) = interval.next().await {
        println!("rem = {}", interval.rem);
    }
}
