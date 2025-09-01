// #[tokio::main]
// async fn main() {
//     println!("Hello world");
// }

// #[tokio::main] 只是语法糖，本质上是会替换成如下代码
// fn main() {
//     tokio::runtime::Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {
//             println!("Hello world");
//         })
// }

use std::sync::mpsc::{self, Sender};

use tokio::runtime::Builder;

use tokio::time::{Duration, sleep};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);

    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i, tx.clone())));
    }
    drop(tx);

    // 在后台任务运行的同时做一些耗费时间的事情
    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    // 等待这些后台任务的完成
    // for handle in handles {
    //     // `spawn` 方法返回一个 `JoinHandle`，它是一个 `Future`，因此可以通过  `block_on` 来等待它完成
    //     runtime.block_on(handle).unwrap();
    // }
    while let Ok(i) = rx.recv() {
        println!("recv {i} done!");
    }
    println!("Main done!");
}

async fn my_bg_task(i: u64, tx: Sender<u64>) {
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);

    sleep(Duration::from_millis(millis)).await;
    tx.send(i).unwrap();
    println!("Task {} stopping.", i);
}
