fn main() {
    number_conv();
    ptr_conv();
    trait_conv_error();
    dot_auto_conv();
}

fn dot_auto_conv() {
    fn do_stuff<T: Clone>(value: &T) {
        let cloned = value.clone();
    }
}

fn trait_conv_error() {
    trait Trait {}
    impl Trait for &i32 {}

    let mut a = &mut 0;
    let mut b = &0;

    fn recive_2<T: Trait>(t: T) {}

    recive_2(b);
    // &mut i32 可以转换成 &i32
    // &i32 实现了 Trait
    // 但是 &mut i32 还是不能作为 Trait
    // recive_2(a);
}

fn number_conv() {
    let a: i32 = 10;
    let b: u16 = 100;

    println!("{}", a as u16);
    println!("{}", b as i32);
    // 大转小，溢出截断
    println!("{}", 300_i32 as i8);
    if a < b as i32 {
        println!("Ten is less than one hundred.");
    }

    let c = 300_i32;
    // try_into
    let bb: i8 = match c.try_into() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e.to_string());
            -1
        }
    };
    println!("{bb}");
}

fn ptr_conv() {
    let mut values: [i32; 2] = [1, 2];
    let p1 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + std::mem::size_of::<i32>(); // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p1 = first_address as *mut i32;
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    // 非常危险的操作
    unsafe {
        *p2 += 1;
        *p1 -= 100;
    }
    println!("{:?}", values);
}
