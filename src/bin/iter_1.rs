fn main() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    // for循环获取v_iter获取所有权
    for val in v_iter {
        println!("Got: {}", val);
    }

    let v = vec!["1", "2", "3"];
    let mut v_iter = v.iter();
    let mut c = v_iter.next();
    c.insert(&"2");
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v);

    let mut v_iter = v.into_iter();
    let mut c = v_iter.next();
    c.insert("1");

    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    // println!("{:?}", v);
}
