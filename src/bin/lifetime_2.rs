#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Foo {
        // self
        &*self
    }
    fn share(&self) {}
}

fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // 编译器认为输入输出生命周期一致，所以 foo可变借用 周期和 不可变借用loan 一致，持续到loan最后一次使用
    // foo.share();
    println!("{:?}", loan);
    foo.share();
}

// b的生命周期大于a
struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}

// T的生命周期大于a
struct Ref<'a, T: 'a> {
    r: &'a T,
}
