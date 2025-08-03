use std::convert::TryFrom;

enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

fn main() {
    // 将枚举转换成整数，顺利通过
    let x = MyEnum::C as i32;
    println!("{x}");
    // 将整数转换为枚举，失败
    // match x {
    //     MyEnum::A => {}
    //     MyEnum::B => {}
    //     MyEnum::C => {}
    //     _ => {}
    // }

    // 使用 try_into
    match x.try_into() {
        Ok(MyEnum::A) => println!("A"),
        Ok(MyEnum::B) => println!("B"),
        Ok(MyEnum::C) => println!("C"),
        _ => {}
    }
}
