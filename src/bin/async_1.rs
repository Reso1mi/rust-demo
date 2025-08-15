use futures::{executor::block_on, join};

fn main() {
    println!("123");
    // 阻塞当前线程，直到future执行完成
    let f = main_life();
    // 在当前线程上调度future
    block_on(f);
}

async fn eat() {
    println!("eat");
}

async fn drink() {
    println!("drink");
}

async fn do_something() {
    println!("go go go");
}

async fn eat_drink() {
    eat().await;
    drink().await;
}

async fn main_life() {
    let ed = eat_drink();
    let other = do_something();
    join!(ed, other);
}
