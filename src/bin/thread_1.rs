use std::sync::Arc;
use std::{sync::Barrier, thread, time::Duration};

fn main() {
    // thread_spawn();
    // thread_spawn_move();
    // child_child_thread();
    thread_barrier();
}

fn thread_barrier() {
    let mut handles = vec![];
    let barrier = Arc::new(Barrier::new(5));
    for i in 1..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("thread {i} before barrier");
            b.wait();
            println!("thread {i} after barrier");
        }));
    }

    handles.into_iter().for_each(|h| {
        h.join();
    });
}

fn child_child_thread() {
    // 创建一个线程A
    let new_thread = thread::spawn(move || {
        // 再创建一个线程B
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        })
    });

    // 等待新创建的线程执行完成
    new_thread.join().unwrap();
    println!("Child thread is finish!");

    thread::sleep(Duration::from_millis(100));
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
