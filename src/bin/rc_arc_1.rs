use std::{
    rc::Rc,
    sync::Arc,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // test_1();
    // rc_deref_mut();
    // rc_case();
    // rc_thread();
    arc_thread();
}

fn arc_thread() {
    let s = String::from("hello, world");
    let rc_1 = Arc::new(s);
    let rc_2 = Arc::clone(&rc_1);
    thread::spawn(move || println!("chlid thread {}", rc_1));
    // println!("{}", rc_1);
    sleep(Duration::from_secs(4));
    println!("main theard {}", rc_2);
}

fn rc_thread() {
    let s = String::from("hello, world");
    let c = &s;
    // argument requires that `s` is borrowed for `'static`
    // let handle = thread::spawn(move || println!("{}", c));

    println!("{s}");
    let rc = Rc::new(s);
    // Rc<String> cannot be sent between threads safely
    // let handle = thread::spawn(move || println!("{}", rc));
}

fn rc_case() {
    struct Owner {
        name: String,
    }

    struct Gadget {
        id: u32,
        owner: Rc<Owner>,
    }

    let owner = Rc::new(Owner {
        name: "Fkj".to_string(),
    });

    let g1 = Gadget {
        id: 1,
        owner: Rc::clone(&owner),
    };

    let g2 = Gadget {
        id: 2,
        owner: Rc::clone(&owner),
    };
    println!("rc-strong-count - {}", Rc::strong_count(&owner)); // 3
    drop(owner);
    println!(
        "drop owner rc-strong-count - {}",
        Rc::strong_count(&g1.owner)
    ); // 2

    println!("{} - {}", g1.id, g1.owner.name);
    println!("{} - {}", g2.id, g2.owner.name);

    let owner = Owner {
        name: "Fkj".to_string(),
    };

    let g1 = Gadget {
        id: 1,
        owner: Rc::new(owner),
    };

    let g2 = Gadget {
        id: 2,
        owner: Rc::clone(&g1.owner),
    };
    println!("rc-strong-count - {}", Rc::strong_count(&g1.owner)); // 2
    drop(g2);
    println!(
        "drop owner rc-strong-count - {}",
        Rc::strong_count(&g1.owner)
    ); // 1
}

fn rc_deref_mut() {
    let s = String::from("hello, world");
    let mut a = Rc::new(s);
    // 报错，因为Rc<T>并没有实现deref_mut，Rc本质上是一个指向底层的不可变引用
    // a.push_str("string");
    let mut b = Rc::clone(&a);
    println!("{a}-{b}");
}

fn test_1() {
    let s = String::from("hello, world");
    // s在这里被转移给a
    let a = Box::new(s);
    // s所有权已经转移
    // println!("{s}");
    // 报错！此处继续尝试将 s 转移给 b
    // let b = Box::new(s);
    //

    let s = String::from("hello, world");

    let a = Rc::new(s);
    println!("after a rc-count = {}", Rc::strong_count(&a)); // 1
    // clone指针，a,b都指向同一份数据，rc-count = 2
    let b = Rc::clone(&a);
    println!("rc-count = {}", Rc::strong_count(&a)); // 2
    println!("after b rc-count = {}", Rc::strong_count(&b)); // 2
    {
        let c = Rc::clone(&a);
        println!("after c rc-count = {}", Rc::strong_count(&a)); // 3
        // drop c
    }
    println!("after end rc-count = {}", Rc::strong_count(&a)); // 2
}
