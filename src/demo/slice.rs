pub fn main() {
    let mut s = String::from("first world!");
    let len = first_word1(&s);
    println!("len = {len}");
    s.clear();
    println!("len = {len}");

    let s = String::from("hello");

    let len = s.len();
    println!("len = {len}");

    let slice = &s[1..];
    println!("slice = {slice}");
    let slice = &s[..2];
    println!("slice = {slice}");

    let mut ss = String::from("ss2 dd");
    let slice2 = first_word2(&ss);
    println!("slice2 = {slice2}");
    // 获取可变引用
    ss.clear();

    println!("slice2 = {ss}");

    let mut s_multi = String::from("s_multi");
    let a = &mut s_multi;
    println!("a = {a}");
    let b = &mut s_multi;
    // println!("a = {a}"); // 第一次创建可变引用到最后一次使用中间不能再创建引用
    println!("b = {b}");
}

fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
