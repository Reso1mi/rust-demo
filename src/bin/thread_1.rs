use std::{thread, time::Duration};

fn main() {
    // thread_spawn();
    thread_spawn_move();
}

fn thread_spawn_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("spawn-move-v: {:?}", v);
    });
    let _ = handle.join();
    // println!("spawn-move-v: {:?}", v);
}

fn thread_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawn: hello world {}", i);
        }
    });

    // 阻塞当前线程，直到handle结束
    let _ = handle.join();

    for i in 1..5 {
        println!("main thread: {}", i);
    }
}
