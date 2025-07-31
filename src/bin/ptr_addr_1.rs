use std::ptr;

// 初始引用地址:
// str_opt.value 引用地址: 0x7ff7b5b82090
// str_opt.test 引用地址: 0x7ff7b5b820a8
// 初始堆地址:
// str_opt.value 堆地址: 0x6000022a8040
// str_opt.test 堆地址: 0x6000022a8050
// -----------------------------------
// 第一次调用:
// a (引用地址): 0x7ff7b5b82090
// b (引用地址): 0x7ff7b5b82090
// a 指向的堆地址: 0x6000022a8040
// b 指向的堆地址: 0x6000022a8040
// 引用地址相同: true
// 堆地址相同: true
// -----------------------------------
// 通过引用调用:
// c (引用地址): 0x7ff7b5b82090
// d (引用地址): 0x7ff7b5b82090
// c 指向的堆地址: 0x6000022a8040
// d 指向的堆地址: 0x6000022a8040
// 引用地址相同: true
// 堆地址相同: true
// -----------------------------------
// 移动所有权后:
// e (引用地址): 0x7ff7b5b827c0
// f (引用地址): 0x7ff7b5b827c0
// e 指向的堆地址: 0x6000022a8040
// f 指向的堆地址: 0x6000022a8040
// 引用地址相同: true
// 堆地址相同: true
// -----------------------------------
fn main() {
    struct StrOpt {
        value: String,
        test: String,
    }

    impl StrOpt {
        // 栈上固定地址，只要不移动，栈上地址不变
        // 即使发生移动，堆地址也不会变化
        fn get_value(&self) -> &String {
            &self.value
        }
    }

    let mut str_opt = StrOpt {
        value: "123".to_string(),
        test: "222".to_string(),
    };

    // 打印初始堆地址
    println!("初始引用地址:");
    println!("str_opt.value 引用地址: {:p}", &str_opt.value);
    println!("str_opt.test 引用地址: {:p}", &str_opt.test);
    println!("初始堆地址:");
    println!("str_opt.value 堆地址: {:p}", str_opt.value.as_ptr());
    println!("str_opt.test 堆地址: {:p}", str_opt.test.as_ptr());
    println!("-----------------------------------");

    // 获取不可变引用
    let a = str_opt.get_value();
    // a生命周期和&str_opt同步，a存在不可变借用&str_opt就存在，就无法创建可变引用
    // str_opt.test.push_str("string");
    let b = str_opt.get_value();

    println!("第一次调用:");
    println!("a (引用地址): {:p}", a);
    println!("b (引用地址): {:p}", b);
    println!("a 指向的堆地址: {:p}", a.as_ptr());
    println!("b 指向的堆地址: {:p}", b.as_ptr());
    println!("引用地址相同: {}", ptr::eq(a, b));
    println!("堆地址相同: {}", a.as_ptr() == b.as_ptr());
    println!("-----------------------------------");

    // 获取结构体的不可变引用
    let zz = &str_opt;
    let c = zz.get_value();
    let d = zz.get_value();

    println!("通过引用调用:");
    println!("c (引用地址): {:p}", c);
    println!("d (引用地址): {:p}", d);
    println!("c 指向的堆地址: {:p}", c.as_ptr());
    println!("d 指向的堆地址: {:p}", d.as_ptr());
    println!("引用地址相同: {}", ptr::eq(c, d));
    println!("堆地址相同: {}", c.as_ptr() == d.as_ptr());
    println!("-----------------------------------");

    // 移动value所有权
    // let vvvv = str_opt.value;
    // partial move occurs because str_opt.value has type String, which does not implement the Copy trait (rustc E0382)
    // let aaa = str_opt.get_value();
    // println!("vvvv (引用地址): {:p}", &vvvv); // 发生移动了，引用地址变化
    // println!("vvvv 指向的堆地址: {:p}", vvvv.as_ptr()); // 堆地址不变

    // 移动str_opt所有权
    let yy = str_opt; // 移动所有权
    let e = yy.get_value();
    let f = yy.get_value();

    println!("移动所有权后:");
    println!("e (引用地址): {:p}", e);
    println!("f (引用地址): {:p}", f);
    println!("e 指向的堆地址: {:p}", e.as_ptr());
    println!("f 指向的堆地址: {:p}", f.as_ptr());
    println!("引用地址相同: {}", ptr::eq(e, f));
    println!("堆地址相同: {}", e.as_ptr() == f.as_ptr());
    println!("-----------------------------------");
}
