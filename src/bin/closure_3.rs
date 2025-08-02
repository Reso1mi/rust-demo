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
    let mut print_string = || println!("{}", s);
    let f_ref: &dyn Fn() = &print_string;
    f_ref();
    // 最终其实都会指向Fn的实现
    let f_ref: &mut dyn FnMut() = &mut print_string;
    f_ref();

    exec1(print_string);
    // s是不可变引用实现了Copy，所以闭包也实现了Copy可以重复调用
    exec1(print_string);

    // FnOnce + FnMut，仅仅修改变量，无需所有权，使用可变借用捕获变量
    let update_string = || s.push_str("123");
    // s.push_str("string"); // 报错，已存在可变借用
    // let zzzz = &mut s; // 报错，已存在可变借用
    // let zzzz = &s; // 报错，已存在可变借用
    exec2(update_string);
    // 闭包使用了可变借用，结束后返回
    let zzzz = &mut s;
    zzzz.push_str("string");

    // FnOnce，将捕获的变量移出了闭包，所有权被移动，使用所有权捕获变量（因为闭包体最后返回了 s，这导致闭包以所有权方式捕获 s）
    // 变量所有权移出了闭包，所以不能再次修改
    let update_string_once = || {
        s.push_str("123");
        s
    };

    exec3(update_string_once);
    // println!("{s}"); // 报错，s所有权已经被移动（未被使用，直接drop）
    // exec3(update_string_once); // update_string_once已经被消耗

    let mut s = String::new();

    let update_string_once_2 = || {
        s.push_str("123");
        s
    };
    s = exec4(update_string_once_2);
    println!("{s}");

    // update_string_once_2 是FnOnce执行后闭包就被消耗，不可再次执行
    // exec4(update_string_once_2);
    //

    // FnOnce+Fn，实际上只实现了Fn方法，FnOnce的方法最终也是调用的Fn都call方法
    let update_string_once = |x: u32| x;
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
