// FnOnce
//      适用于只能被调用一次的闭包。
//      所有闭包至少都实现了这个 trait，因为所有闭包都能被调用。
//      一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait，而不会实现其他 Fn 相关的 trait，因为它只能被调用一次。
// FnMut
//      适用于不会将捕获的值移出闭包体，But it is possible to modify the closure of the captured value.这类闭包可以被调用多次。
// Fn
//      适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。这类闭包可以被多次调用而不会改变其环境，这在会多次并发调用闭包的场景中十分重要

// FnOnce：承担消费型操作的单次执行
// FnMut：专注可安全修改的多轮调用
// Fn：提供无副作用的纯函数式操作
fn main() {
    let mut s = String::new();

    // FnOnce + FnMut + Fn
    let print_string = || println!("{}", s);

    exec1(print_string);
    // s是不可变引用实现了Copy，所以闭包也实现了Copy可以重复调用
    exec1(print_string);

    // FnOnce + FnMut
    let update_string = || s.push_str("123");
    exec2(update_string);

    // FnOnce，将捕获的变量移出了闭包，所以没有实现FnMut
    // 变量所有权移出了闭包，并且没有再接收值，所以不能再次修改，违背了FnMut的初衷
    let update_string_once = || {
        s.push_str("123");
        s
    };

    exec3(update_string_once);
    // println!("{s}");
    // exec3(update_string_once);

    let mut s = String::new();

    let update_string_once_2 = || {
        s.push_str("123");
        s
    };
    s = exec4(update_string_once_2);
    println!("{s}");

    // 为什么报错？
    // exec4(update_string_once_2);
}

fn exec1<F: FnOnce() + FnMut() + Fn()>(f: F) {
    f()
}

fn exec2<F: FnOnce() + FnMut()>(mut f: F) {
    f()
}

fn exec3<F: FnOnce() -> String>(f: F) {
    f();
}

fn exec4<F: FnOnce() -> String>(f: F) -> String {
    f()
}
