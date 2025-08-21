pub fn main() {
    let mut s = String::from("value");
    test2(&mut s);
    println!("s = {s}");

    let s2 = String::from("value2");
    test(s2);

    let mut s3 = String::from("value3");
    test3(&mut s3);
    println!("s3 = {s3}");
}

fn test(mut s: String) {
    let mut other_string = String::from("other");
    s.push_str("123");
    println!("s = {s}");
    s = other_string;
}

fn test2(s: &mut String) {
    s.push_str("123");
    println!("s = {s}");
}

fn test3(mut s: &mut String) {
    s.push_str("123");
    println!("s = {s}");
}

fn deref_test() {
    let list = vec![String::from("value")];
    // 模式匹配解构会移动所有权，迭代器返回的其实是&String，没有所有权只是借用
    // for &s in &list {
    for s in &list {
        // 错误！无法移动 String
        println!("{}", s);
    }

    let mut kk = String::from("value");
    let z = &mut kk;

    // let c: String = *z;

    // let &s = &kk; // 非法：尝试移动 String
    // let s = *(&kk);
    // 等价于 let s = *(&kk);  // 尝试解引用获取 String 的所有权

    let num = 42;
    let n = *(&num);
    let &n = &num; // Copy类型，合法：复制值，不移动所有权
    println!("{}", n); // 42
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 返回引用，不涉及所有权
    fn get_x(&self) -> &T {
        &self.x
    }
    // ​​共享引用不能转移所有权​​（只能读，不能移动）。
    // ​​非 Copy 类型必须显式转移所有权​​（不能隐式复制）。
    // fn get_x_2(&self) -> T {
    //     self.x
    // }

    // 值传递，外部传递所有权
    fn get_x_3(self) -> T {
        self.x
    }

    fn mixup<Z, W>(self, p: Point<U, W>) -> Point<T, W> {
        Point { x: self.x, y: p.y }
    }
}
