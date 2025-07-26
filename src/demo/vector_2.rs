#![allow(unused)]

use std::fmt::format;
pub fn main() {
    let m = String::new();

    let l = "test".to_string();
    let ll = String::from("test");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = "A".to_string();
    let cc = "B";
    // &str不获取所有权
    s.push_str(cc);
    // 单字符
    s.push('C');
    println!("{s}-{cc}");

    let s1 = "s1";
    let s2 = "s2";
    let s3 = s1.to_string() + s2;
    // s1, s2都是&str,无所有权
    println!("{s1}-{s2}-{s3}");

    let s1 = "s1".to_string();
    let s2 = "s2".to_string();
    let s3 = s1 + &s2;
    println!("{s3}");
    println!("{s2}");
    // s1已经不能使用了 +对应方法中获取了s1所有权
    // println!("{s1}");

    let s3 = format!("{}-{}", s2, s3);
    println!("{s3}");

    // String底层是对 Vec<u8> 对封装，对于多字节字符，获取单个字节没有意义，单字节也同样被禁止
    // let cs3 = s3[0];
    let emg = "😊";
    // 4
    println!("{}", emg.len());

    let e = &emg[0..4];
    println!("{e}");
    //panicked byte index 1 is not a char boundary; it is inside '😊' (bytes 0..4) of `😊`
    // let e = &emg[0..1];
    println!("------");
    for c in emg.chars() {
        println!("{c}");
    }
}
