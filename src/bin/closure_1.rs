use std::{collections::HashMap, fmt::Display, hash::Hash, thread, time::Duration};

pub fn main() {
    let list = vec![1, 2, 3, 4];
    let borrow_print = || println!("close: {list:?}");
    println!("before borrow_print: {list:?}");
    borrow_print();
    println!("after borrow_print:{list:?}");

    let mut list_mut = vec![1, 2, 3, 4];
    println!("before mut_borrow_push: {list_mut:?}");
    let mut mut_borrow_push = || list_mut.push(5);
    // 无法获取不可变引用，因为闭包中获取呢可变引用
    // println!("before mut_borrow_push: {list_mut:?}");
    mut_borrow_push();
    println!("after mut_borrow_push: {list_mut:?}");

    let list_move = vec![1, 2, 3];
    println!("before list_move: {list_move:?}");
    // closure may outlive the current function, but it borrows `list`, which is owned by the current function
    // force the closure to take ownership of `list`
    thread::spawn(move || println!("From thread: {list_move:?}"))
        .join()
        .unwrap();
    // value borrowed here after move
    // println!("after list_move: {list_move:?}");
    //

    let expensive_closure = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = 4;
    // 普通的函数不能捕获环境中的变量
    // fn equal_to_x(z: i32) -> bool { z == x }

    // Fn trait 闭包函数，获取不可变借用值
    let eq_x = |num: i32| num == x;
    let y = 4;
    assert!(eq_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
