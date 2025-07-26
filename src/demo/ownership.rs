#![allow(unused)]
pub fn main() {
    let s1 = String::from("hello");
    // 堆上数据，所有权转让，原s1失效
    // let s2 = s1; ❌
    println!("{s1}, world!");

    let mut s2 = String::from("test");
    println!("pre s2 = {s2}");
    s2 = String::from("test mod");
    println!("s2 = {s2}");

    let s3 = String::from("clone");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");

    // 栈上数据，copy trait, 拷贝快速
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // 转移
    let str = String::from("123");
    take_ownership(str);
    // println!("str = {str}"); ❌
    let num = 3;
    copy_value(num);
    println!("num = {num}");

    let str2 = String::from("456");
    let back = take_giveback(str2);
    // println!("str2 = {str2}"); ❌
    println!("back = {back}");

    // 引用reference, 不转移所有权
    let str3 = String::from("qwer");
    let len = cal_length(&str3);
    println!("str3 = {str3}, len = {len}");

    let mut str4 = String::from("asdf");
    mod_ref(&mut str4);
    println!("mut str4 = {str4}");

    // 不允许对同一变量的同时持有多个可变引用
    let mut str5 = String::from("str5");
    let r1 = &mut str5;
    println!("{r1}"); // 使用
    let r2 = &mut str5;
    // println!("{r1}");
    println!("{r2}");

    // 持有不可变引用的同时，也不能持有可变引用
    let mut str6 = String::from("str6");
    let rr1 = &str6;
    let rr2 = &str6;
    // let rr3 = &mut str6;
    println!("{rr1}");
    println!("{rr2}");
    let rr3 = &mut str6; // is ok!
    println!("{rr3}");

    // 野指针
    let p = point();
    println!("p = {p}");
}

// fn point() -> &String {
//     let s = String::from("point");
//     &s
// }

fn point() -> String {
    let s = String::from("point");
    s
}

fn mod_ref(str: &mut String) {
    str.push_str(", append!");
}

fn cal_length(str: &String) -> usize {
    str.len()
}

fn take_giveback(str: String) -> String {
    str
}

fn take_ownership(str: String) {
    println!("str = {str}");
}

fn copy_value(num: u32) {
    println!("{num}");
}
