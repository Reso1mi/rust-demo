use std::{
    collections::{HashMap, HashSet},
    i32,
};
pub fn main() {
    let mut m = HashMap::new();
    m.insert("k", 1);
    m.insert("j", 2);
    dbg!(&m);
    m.insert("z", 3);

    let v = vec![1, 2, 3];
    let sc = vec![100, 90];

    let m: HashMap<_, _> = v.iter().zip(sc.iter()).collect();
    // &m = {
    //     2: 90,
    //     1: 100,
    // }
    dbg!(&m);

    let mut m = HashMap::new();
    let first_name = "tadow".to_string();
    let first_value = 100;
    // first_name:String 所有权移交，所有权到map中
    // first_value:i32 实现了Copy trait，值拷贝到map中
    m.insert(first_name, first_value);
    // 不能再使用
    // println!("fn= {first_name}")
    println!("fv = {first_value}");

    match m.get("tadow") {
        Some(v) => println!("tadow v = {v}"),
        None => println!("tadow v is none"),
    }

    for (k, v) in &m {
        println!("k = {}, v = {}", k, v);
    }

    // update map
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Blue", 25);
    // only blue-25
    // {"Blue": 25}
    println!("{:?}", scores);

    scores.entry("Blue").or_insert(10000);
    scores.entry("Yellow").or_insert(20000);
    // {"Blue": 25, "Yellow": 20000}
    println!("{:?}", scores);

    let text = "Hello World";
    let mut count_map = HashMap::new();
    for i in text.chars() {
        // or_insert返回可变引用 &mut V
        let cnt = count_map.entry(i).or_insert(0);
        *cnt += 1;
    }
    let c = 'o';
    println!("{:?}", count_map);
    if let Some(tv) = count_map.get_mut(&c) {
        *tv += 100;
    }
    println!("{:?}", count_map);

    let a = if true { 1 } else { 2 };
}

// lc3487
pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut m = HashSet::new();
    let mut max = i32::MIN;
    for v in nums {
        if v >= 0 {
            m.insert(v);
        } else {
            max = max.max(v);
        }
    }
    if m.is_empty() { max } else { m.iter().sum() }
}
