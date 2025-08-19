use std::{
    marker::PhantomPinned,
    mem::swap,
    ops::Deref,
    pin::{self, Pin},
};

fn main() {
    // raw_point_value_move();
    // pin_test();
    pin_test2();
}

// fn dangerous_deref<T>(r: &T) -> T {
//     *r // 当 T 未实现 Copy 时被编译器阻止
// }

fn pin_test2() {
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(s: &str) -> Self {
            Test {
                a: s.to_string(),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            }
        }

        fn init(pin_self: Pin<&mut Self>) {
            let this = unsafe { pin_self.get_unchecked_mut() };
            this.b = &this.a as *const String;
        }

        fn a(pin_self: Pin<&Self>) -> &str {
            // 等价于 &(*(pin_self.deref())).a
            // Pin<&Self> deref结果是 &Self
            // &(*(pin_self.deref())).a
            //             // 等价展开
            // let temp_ref: &Self = pin_self.deref();   // 步骤1: 获取结构体引用
            // let temp_val: Self = *temp_ref;           // 步骤2: 解引用结构体
            // &temp_val.a                              // 返回临时结构体的字段引用
            // &pin_self.a
            &pin_self.get_ref().a
        }

        fn b(pin_self: Pin<&Self>) -> &String {
            unsafe { &*pin_self.b }
        }
    }

    let mut test1 = Test::new("test1111");
    // 这里需要用pin遮蔽test1，否则会出现pin生命周期结束后test1依然存在的情况
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2222");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };

    Test::init(test2.as_mut());

    println!(
        "test1.a: {}, test1.b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );

    println!(
        "test2.a: {}, test2.b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );

    // swap(&mut test1.as_ref().a, &mut test1.as_ref().a);

    let mut this1 = unsafe { test1.get_unchecked_mut() }; // 绕过编译器检查
    let mut this2 = unsafe { test2.get_unchecked_mut() }; // 绕过编译器检查
    swap(this1, this2);
}

fn pin_test() {
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(s: &str) -> Self {
            Test {
                a: s.to_string(),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            self.b = &self.a as *const String;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            unsafe { &*self.b }
        }
    }

    let mut test1 = Test::new("test1111");
    test1.init();
    let mut test2 = Test::new("test2222");
    test2.init();

    println!("test1.a: {}, test1.b: {}", test1.a(), test1.b());

    println!("test2.a: {}, test2.b: {}", test2.a(), test2.b());

    // swap会将内存中数据进行交换，a,b都是指针，但是b是指向指针的指针，所以交换后test2.b从指向test2.a的指针变成了指向test1.a的指针
    swap(&mut test1, &mut test2);
    test1.a = "I've totally changed now!".to_string();

    // swap-test1.a: I've totally changed now!, swap-test1.b: test1111
    println!("swap-test1.a: {}, swap-test1.b: {}", test1.a(), test1.b());

    // 交换后test2.a的String指向test1.a的String
    // test2.b指向之前的test2.a，test1和test2交换，test2.b还是指向之前的test2.a
    // swap-test2.a: test1111, swap-test2.b: I've totally changed now!
    println!("swap-test2.a: {}, swap-test2.b: {}", test2.a(), test2.b());
}

fn raw_point_value_move() {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        pointer_to_value: *mut String,
    }

    let mut s = SelfRef {
        value: "123".to_string(),
        pointer_to_value: std::ptr::null_mut(),
    };

    s.pointer_to_value = &mut s.value as *mut String;

    println!("移动前堆地址: {:p}", s.value.as_ptr());
    println!("移动前引用地址: {:p}", &s.value);

    println!("移动前：{:?}", s);
    // 移动s所有权（浅拷贝），引用地址发生变化，原引用地址不会立即失效，但是随时会失效
    let z: String = s.value;
    println!("移动前裸指针值：{:?}", unsafe {
        &*s.pointer_to_value
    });
    // drop之后就无法再通过裸指针访问
    // drop(z);
    println!("裸指针地址（移动前引用地址）{:?}", s.pointer_to_value);

    println!("移动后堆地址: {:p}", z.as_ptr());
    println!("移动后引用地址: {:p}", &z);

    println!("移动后裸指针值：{:?}", unsafe {
        &*s.pointer_to_value
    });
    println!("裸指针地址（移动后引用地址）{:?}", s.pointer_to_value);
}
