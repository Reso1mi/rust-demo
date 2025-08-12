use std::sync::{Arc, Condvar, Mutex, Once};
use std::{sync::Barrier, thread, time::Duration};

fn main() {
    // thread_spawn();
    // thread_spawn_move();
    // child_child_thread();
    // thread_barrier();
    // thread_local();
    // thread_cond();
    thread_once();
}

fn thread_once() {
    static mut VAL: usize = 0;
    static INIT_FN: Once = Once::new();
    let handle1 = thread::spawn(|| {
        INIT_FN.call_once(|| unsafe {
            VAL += 1;
        });
    });

    let handle2 = thread::spawn(|| {
        INIT_FN.call_once(|| unsafe {
            VAL += 1;
        });
    });
    handle1.join();
    handle2.join();
    println!("VAL = {}", unsafe { VAL });
}

fn thread_cond() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        // let lock = &pair2.0;
        // let cvar = &pair2.1;
        // 错误，不能从Arc中转移所有权
        // let lock = pair2.0;
        let mut started = lock.lock().unwrap();
        println!("start");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

fn thread_local() {
    use std::cell::RefCell;
    use std::thread;

    // 初始值
    thread_local! {static FOO: RefCell<u32> = RefCell::new(1)}

    FOO.with(|f| {
        println!("before main: f = {}", f.borrow());
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            println!("chlid: f = {}", f.borrow());
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
        FOO.set(123123);
        println!("child mod: {}", FOO.take());
    });

    t.join();

    FOO.with(|f| {
        println!("after main: f = {}", f.borrow());
        assert_eq!(*f.borrow(), 2);
    });
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
