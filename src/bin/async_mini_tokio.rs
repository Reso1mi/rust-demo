use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

use futures::task;

struct MiniTokio {
    tasks: VecDeque<Task>,
}

type Task = Pin<Box<dyn Future<Output = ()> + Send>>;

impl MiniTokio {
    fn new() -> MiniTokio {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    /// 生成一个 Future并放入 mini-tokio 实例的任务队列中
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        let waker = task::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}

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

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(5);
    let f = Delay { when };

    let ret = f.await;

    assert_eq!(ret, "done");
}
