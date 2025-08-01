// 例如，对于以下闭包：
// let x = 10;
// let closure = |a| a + x;
// 编译器会生成一个类似如下的结构体：

// struct Closure {
//     x: i32, // 按值捕获 x
// }

// impl FnOnce<(i32,)> for Closure {
//     type Output = i32;
//     fn call_once(self, (a,): (i32,)) -> i32 {
//         // 调用FnMut或Fn的实现
//         (&self).call(a) // 假设还有Fn的实现，调用它
//     }
// }

// impl FnMut<(i32,)> for Closure {
//     fn call_mut(&mut self, (a,): (i32,)) -> i32 {
//         // 调用Fn的实现
//         (*(self)).call(a)
//     }
// }

// impl Fn<(i32,)> for Closure {
//     fn call(&self, (a,): (i32,)) -> i32 {
//         a + self.x
//     }
// }

fn main() {
    fn fn_elision(x: &i32) -> &i32 {
        x
    }
    // 这种情况下编译器不太能分析出生命周期
    // let closure_slision = |x: &i32| -> &i32 { x };

    // FnOnce-String
    let z = String::from("123");
    // 捕获的变量为不可变借用，实现了Copy
    fn_once(|| println!("{}", z));
    // 强制移动所有权，String未实现Copy
    // the trait bound String: Copy is not satisfied in
    // fn_once(move || {
    //     z == "123";
    // });
    // 直接获取所有权变量，String未实现Copy
    // fn_once(|| {
    //     dbg!(z);
    // });
    // 获取可变借用，可变借用未实现Copy
    // the trait bound &mut String: Copy is not satisfied
    // Copy is implemented for &String, but not for &mut String (rustc E0277)
    // fn_once(|| z.push_str("string"));

    // FnMut1
    let mut s = String::new();
    // 需要将闭包声明为mut
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    update_string("123");
    println!("{:?}", s);
    // FnMut2
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    // update_string 所有权已经转移，无法再调用
    // exec(update_string);
    println!("{:?}", s);

    // Fn1
    let mut s = String::new();
    // fn_1(|str| s.push_str(str));
    fn_1(|str| println!("{}-{}", str, s));
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
    // fn call_mut(&mut self, args: Args) 获取可变借用，不获取所有权，可以重复调用
    f("hello");
}

fn fn_once<F>(f: F)
where
    // FnOnce能否Copy取决于内部所有捕获的变量类型是不是都实现了Copy
    F: FnOnce() + Copy,
{
    f();
    // FnOnce的闭包在调用后会转移闭包所有权，所以不能再调用，除非实现了Copy trait（闭包内如果所有变量都是都实现Copy特征，就会自动实现Copy trait）
    // 把闭包理解成一个包含了捕获了变量的结构体，看上面注释代码函数签名：call_once(self...) 会获取闭包对象的所有权
    // 调用拷贝的闭包对象 copy self
    f();
}

fn fn_once_2<F>(f: F)
where
    // FnOnce能否Copy取决于内部所有捕获的变量类型是不是都实现了Copy
    F: FnOnce(),
{
    f();
    // 无法调用
    // f();
}

fn fn_1<F>(f: F)
where
    F: Fn(&str),
{
    f("123");
}
