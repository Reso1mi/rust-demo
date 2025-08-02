use std::collections::HashMap;

fn main() {
    let mut a = [1, 2, 3];
    a.iter_mut();
    let mut v = vec![1, 2, 3];
    let v_iter = v.iter();
    let v_iter = v.iter_mut();
    let v_iter = v.into_iter();

    // for循环获取v_iter获取所有权
    for val in v_iter.into_iter() {
        println!("Got: {}", val);
    }
    // v_iter被消耗，无法再使用
    // v_iter.sum::<u32>();
    // v.into_iter() 转移了v所有权 无法再使用
    // println!("{:?}", v);

    let mut arr = [8, 9, 10];
    arr.into_iter();
    // 数组如果都是Copy类型，那么是可以再次调用的
    // The array cannot be used after calling this unless T implements Copy, so the whole array is copied.
    for v in arr.into_iter() {
        println!("{}", v);
    }
    let mut s_arr = ["8".to_string(), "9".to_string(), "10".to_string()];
    s_arr.into_iter();
    // 不可再次调用
    // for v in s_arr.into_iter() {
    //     println!("{}", v);
    // }

    //
    println!("{:?}", arr);

    let mut iter = arr.into_iter();
    println!("{:?}", iter.next());

    // 通过iter_mut获取可变迭代器
    let mut iter = arr.iter_mut();
    let a = iter.next().unwrap();
    *a = 100;
    println!("arr = {:?}", arr);

    // 消费者适配器
    comsumer_iter();
    // 迭代器适配器
    iter_iter();
    // zip
    zip_iter();
}

fn iter_iter() {
    let v: Vec<i32> = vec![1, 2, 3];
    let iter = v.iter();
    // 链式调用，最后需要一个消费者适配器来收尾
    let ret: Vec<_> = iter
        .map(|x| x + 1)
        .filter(|x| *x > 3)
        .map(|x| x.to_string())
        .collect();
    println!("{:?}", ret);
}

fn zip_iter() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);
}

fn comsumer_iter() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    // 消耗迭代器所有权
    let ret: u32 = iter.sum();
    // 无法再使用
    // iter.next();
    println!("{ret}");
    // 不影响v的使用
    println!("{:?}", v);
}

fn sim_for() {
    let values = vec![10, 20, 30];
    match values.into_iter() {
        mut iter => loop {
            match iter.next() {
                Some(x) => {
                    println!("{}", x);
                }
                None => break,
            }
        },
    };
    // into_iter获取所有权
    // print!("values={:?}", values);
}
