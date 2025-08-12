use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // recv_1();
    // try_recv();
    trans_ownership();
}

fn trans_ownership() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        let str = String::from("123");
        // send会转移所有权，recv_1传递的是i32，实现了Copy
        s.send(str);
        // 所有权转移，无法使用
        // println!("str = {}", str);
    });

    let ms = r.recv();
    println!("ms str = {}", ms.unwrap());
}

fn try_recv() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || s.send(1));

    println!("rec = {:?}", r.try_recv()); // rec = Err(Empty)
    thread::sleep(Duration::from_secs(1));
    println!("rec = {:?}", r.try_recv()); // rec = Ok(1)
    println!("rec = {:?}", r.try_recv()); // rec = Err(Disconnected)
}

fn recv_1() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || s.send(1));

    let ret = r.recv().unwrap();
    println!("rec = {}", ret);
}
