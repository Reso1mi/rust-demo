use std::pin::pin;

use futures::{
    executor::block_on,
    future::{self, FutureExt},
    pin_mut, select,
};

async fn task_one() { /* ... */
}
async fn task_two() { /* ... */
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    // let t1 = t1;

    // 先完成先执行，不会等待另一个
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    };

    // let t1 = task_one();
    // let t2 = task_two();

    // pin_mut!(t1, t2);

    // // let t1 = t1;

    // // 先完成先执行，不会等待另一个
    // select! {
    //     () = t1 => println!("任务1率先完成"),
    //     () = t2 => println!("任务2率先完成"),
    // }
}

fn select_2() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        // complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合 loop 使用，loop 用于循环完成所有的 Future
        // default 分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => panic!(), // 该分支永远不会运行，因为 `Future` 会先运行，然后是 `complete`
        };
    }
    assert_eq!(total, 10);
}

fn main() {
    block_on(race_tasks());
}
