pub fn main() {
    let mut v = Vec::new();
    v.push(1);
    dbg!(v);

    let v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
    // for i in v {} 获取所有权
    let third = &v[2];
    println!("third = {third}");

    match v.get(2) {
        Some(t) => println!("match third = {t}"),
        None => println!("None"),
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let first = &v[0];
    // pub fn push(&mut self, value: T) 会创建v的可变引用，会修改v中内容
    // 当存在对某数据（或其子部分）的不可变引用时，禁止通过任何方式修改该数据或其容器
    // push可能会导致v[0]内容发生变化，所以禁止
    // v.push(7);

    println!("{first}");

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![4, 5, 6, 7, 9];
    // 获取元素i的可变引用
    for i in &mut v {
        *i += 100;
    }

    #[derive(Debug)]
    enum Coin {
        BTC(String),
        ETH(u32),
        SOL(f64),
    }

    let v = vec![
        Coin::BTC(String::from("123")),
        Coin::ETH(1423),
        Coin::SOL(123.3),
    ];
    dbg!(&v);
}

// leetcode: 2210
pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let mut last = nums[0];
    let mut ret = 0;
    for i in 1..nums.len() - 1 {
        let next = nums[i + 1];
        if nums[i] == nums[i + 1] {
            continue;
        }

        if nums[i] < last && nums[i] < next {
            ret += 1;
        }

        if nums[i] > last && nums[i] > next {
            ret += 1;
        }
        last = nums[i];
    }
    ret
}
