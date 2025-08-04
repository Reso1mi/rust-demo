struct One;
// 析构函数
impl Drop for One {
    fn drop(&mut self) {
        println!("drop one");
    }
}

struct Two;
impl Drop for Two {
    fn drop(&mut self) {
        println!("drop two");
    }
}

struct Combo {
    one: One,
    two: Two,
}
// impl Drop for Combo {
//     fn drop(&mut self) {
//         println!("drop combo");
//     }
// }
#[derive(Debug)]
struct Foo;
impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop foo");
    }
}

fn main() {
    let combo = Combo { one: One, two: Two };

    let foo = Foo;
    // foo.drop();
    // drop(foo);
    println!("Running!:{:?}", foo);

    println!("running done!");
}
