#![allow(unused)]
pub fn main() {
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }
    //

    let a = String::from("value");
    let b = "abcdeef";
    let ret = long(&a, b);
    println!("{ret}");

    let ret;
    {
        // 编译时直接嵌入二进制的字符串字面量
        // 生命周期是 'static 整个程序运行期间有效
        // 变量 a 和 b 只是这些静态字符串的引用
        let a = "String::from()";
        let b = "abcdeef";
        ret = long(a, b);
    } // 大括号结束时，变量 a 和 b 被销毁（它们是栈上的引用变量）真正的字符串数据在静态存储区，不受影响
    println!("ret = {ret}");

    // let ret;
    // {
    //     let a = "test".to_string();
    //     let b = "testtest".to_string();
    //     ret = long(&a, &b); //a, b生命周期 < ret
    // }
    // println!("ret = {ret}");
    // ret指向释放的内存

    // let ret;
    // let a = "test".to_string();
    // {
    //     let b = "testtest".to_string();
    //     ret = long(&a, &b); // b lifttime < ret
    // }
    // println!("ret = {ret}");
    //

    // 限制User对象的生命周期不能比name和email的还要长
    struct User<'a> {
        name: &'a str,
        email: &'a str,
    }

    // let u;
    // {
    //     let s = "tadow".to_string();
    //     let e = "private@qq.com".to_string();
    //     u = User {
    //         name: &s,
    //         email: &e,
    //     };
    // }
    // println!("{}-{}", u.email, u.name);
    // 作用域结束后s,e失效，u中持有引用
}

// 加了生命周期限制后，确保 输出的引用生命周期>=min(s1, s2)
fn long<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 直接返回悬垂引用，编译失败，返回值生命周期和参数没有关联
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 生命周期省略规则：
// 编译器按顺序应用以下三条规则推断生命周期：
// 规则 1：为每个引用参数分配独立生命周期（如 fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)）。
// 规则 2：若只有一个输入引用参数，将其生命周期赋给所有输出引用。
// 规则 3：若方法有 &self/&mut self，将其生命周期赋给所有输出引用。
// 若应用后仍有未确定的生命周期，则编译报错，需手动标注。
//
// 1.若函数仅有一个引用参数，编译器自动将返回引用的生命周期与该参数绑定
//   - 不可能在函数内重新产生一个引用并且返回，这样就会成为悬垂指针）
// 2.方法中有 &self 或 &mut self，编译器自动尝试将输出引用的生命周期和self绑定
//   - 返回值的来源：
//      - 直接来自结构体自身（如 &self.field）
//      - 通过结构体内部数据计算得出（如self.data.get_ref()）
//      - 两种来源都决定了通过self返回的引用不可能比结构体实例本身存活更久
// 3.不返回引用，或者返回自有类型
//
struct MyStruct {
    name: String,
}
impl MyStruct {
    // 返回的引用来自外部输入，与self无关
    // 自动推导结果为self的lifetime，实际返回的是external，冲突，需要手动标注
    fn combine<'a>(&self, external: &'a str) -> &'a str {
        external
    }

    // 返回引用生命周期可以直接推导
    fn combine2(&self, external: &str) -> &str {
        &self.name
    }

    fn combine3<'a>(&'a self, external: &'a str, b: bool) -> &'a str {
        if b { external } else { &self.name }
    }
}

pub struct User<'a> {
    pub active: bool,
    pub name: &'a str,
    pub email: &'a str,
    // username: &str,
    pub sign_in_count: u64,
}

// 返回值生命周期 ≤ 'a
pub fn build_user<'a>(email: &'a str, name: &'a str, active: bool) -> User<'a> {
    User {
        active,
        email: email,
        name: name,
        sign_in_count: 1,
    }
}

fn bulid_user_test() {
    let email = String::from("email");
    let name = String::from("name");
    let u = build_user(&email, &name, true);

    let u2;
    {
        let email2 = String::from("email");
        {
            let name2 = String::from("name");
            // 返回的u2生命周期和email2以及name2最短的一致
            u2 = build_user(&email2, &name2, true);
        }
        // println!("{:?}", u2.email);
    }
    // println!("{:?}", u2.email);
}
