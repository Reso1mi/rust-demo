use std::{
    sync::{Arc, mpsc},
    thread::{self, Thread},
    time::Duration,
};

fn main() {
    // recv_1();
    // try_recv();
    // trans_ownership();
    // send_some();
    multi_sender();
    // block_chan();
    // send_enum();
}

fn send_enum() {
    enum Animal<'a> {
        Cat { age: u8, name: String },
        Dog(&'a str),
    }
    let (s, r) = mpsc::channel();

    s.send(Animal::Cat {
        age: 8,
        name: "zaizai".to_string(),
    });
    s.send(Animal::Dog("qdd"));

    for ret in r {
        match ret {
            Animal::Cat { age, name } => println!("cat age  = {}, name = {}", age, name),
            Animal::Dog(name) => println!("dog name = {}", name),
        }
    }
}

fn block_chan() {
    let (s, r) = mpsc::sync_channel(0);

    thread::spawn(move || {
        s.send("t");
        println!("send t");
        s.send("a");
        println!("send a");
        s.send("b");
        println!("send b");
    });

    // r.recv();
    thread::sleep(Duration::from_secs(3));
    println!("{}", r.recv().unwrap());
    println!("end");
}

fn send_some() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        s.send("t");
        s.send("a");
        s.send("b");
    });

    println!("wait send");
    for ret in r {
        println!("{}", ret);
    }
}

fn test() {
    let s = "1212";
    let ss = s;
    let string = s.to_string();

    let handle = thread::spawn(move || {
        println!("{}", ss);
        println!("{}", string);
    });

    println!("{s}");
    println!("{ss}");
    // println!("{string}");

    handle.join().unwrap();
}

fn multi_sender() {
    let (s, r) = mpsc::channel();

    let s1 = s.clone();
    thread::spawn(move || {
        let str = String::from("123");
        s1.send(str)
    });

    let s2 = s.clone();
    thread::spawn(move || {
        let str = String::from("456");
        s2.send(str);
    });
    drop(s);
    for ret in r {
        println!("{}", ret);
    }
    println!("end");
}

fn trans_ownership() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        let str = String::from("123");
        // send会转移所有权，recv_1传递的是i32，实现了Copy，String未实现，转移所有权
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
