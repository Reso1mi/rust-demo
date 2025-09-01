use tokio::time::{Duration, sleep};

use tokio::{
    signal,
    sync::mpsc::{Sender, channel},
};

async fn test() {
    // ... spawn application as separate task ...
    // 在一个单独的任务中处理应用逻辑

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    //  1、发送关闭信号给应用所在的任务，然后等待
    // let next_frame = tokio::select! {
    //     res = self.connection.read_frame() => res?,
    //     _ = self.shutdown.recv() => {
    //         // 当收到关闭信号后，直接从 `select!` 返回，此时 `select!` 中的另一个分支会自动释放，其中的任务也会结束
    //         return Ok(());
    //     }
    // };
}

#[tokio::main]
async fn main() {
    let (send, mut recv) = channel(1);

    for i in 0..10 {
        tokio::spawn(some_operation(i, send.clone()));
    }

    // 等待各个任务的完成
    //
    // 我们需要 drop 自己的发送端，因为等下的 `recv()` 调用会阻塞, 如果不 `drop` ，那发送端就无法被全部关闭
    // `recv` 也将永远无法结束，这将陷入一个类似死锁的困境
    drop(send);

    // 当所有发送端都超出作用域被 `drop` 时 (当前的发送端并不是因为超出作用域被 `drop` 而是手动 `drop` 的)
    // `recv` 调用会返回一个错误
    let _ = recv.recv().await;

    println!("Main Task down");
}

async fn some_operation(i: u64, _sender: Sender<()>) {
    sleep(Duration::from_millis(100 * i)).await;
    println!("Task {} shutting down.", i);

    // 发送端超出作用域，然后被 `drop`
}
