use std::rc::Rc;

fn main() {
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
    println!("rc-count = {}", Rc::strong_count(&a)); // 1

    // clone指针，a,b都指向同一份数据，rc-count = 2
    let b = Rc::clone(&a);
    println!("rc-count = {}", Rc::strong_count(&a)); // 2
    println!("rc-count = {}", Rc::strong_count(&b)); // 2
}
