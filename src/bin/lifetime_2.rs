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
// 其实并不是因为rehash什么的导致借用整体，
// 编译器并没有对集合类型特殊处理，而仅仅是因为使用get方法时，传入了&self，返回了&V，
// 返回的时候创建的新的引用依赖于输入的&self，二者周期绑定，编译器认为当&V存在时，&self也肯定要存在，
// 所以直到&V结束周期，都不能再创建self的不可变引用
//

// 和上面main方法逻辑一致
fn test() {
    struct StrOpt {
        value: String,
        test: String,
    }

    impl StrOpt {
        fn get_value(&self) -> &String {
            &self.value
        }
    }

    let str_opt = &mut StrOpt {
        value: "123".to_string(),
        test: "23".to_string(),
    };

    // 通过&self获取获取不可变引用，返回引用生命周期和输入绑定
    let v = str_opt.get_value();
    // move 获取所有权
    // let v = str_opt.value;
    println!("{:?}", str_opt.value);
    // 直接获取value不可变引用
    // let v = &str_opt.value;

    // 报错
    // str_opt.test.push_str("string");

    // 最后一次使用不可变借用
    println!("{v}");
}
