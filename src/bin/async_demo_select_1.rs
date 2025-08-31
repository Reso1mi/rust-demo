use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::oneshot;

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    // let (tx1, rx1) = oneshot::channel();
    // let (tx2, rx2) = oneshot::channel();

    // tokio::spawn(async {
    //     let _ = tx1.send("one");
    // });

    // tokio::spawn(async {
    //     let _ = tx2.send("two");
    // });

    // tokio::select! {
    //     val = rx1 => {
    //         println!("rx1 completed first with {:?}", val);
    //     }
    //     val = rx2 => {
    //         println!("rx2 completed first with {:?}", val);
    //     }
    // }

    // 任何一个 select 分支结束后，都会继续执行接下来的代码

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // 使用 tx1 和 tx2

    MySelect { rx1, rx2 }.await;
}
