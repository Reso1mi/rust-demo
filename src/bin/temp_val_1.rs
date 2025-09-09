// fn main() -> () {
//     let mut _0: ();                      // 返回值占位符
//     let _1: &main::Test<'_>;             // 对应变量 t (引用)
//     let _2: main::Test<'_>;              // 编译器生成的隐藏变量 (拥有所有权)
//     let mut _3: std::string::String;      // "123".to_string() 的结果
//     let mut _4: &str;                    // "123" 字面量
//     let mut _5: &str;                    // "444" 字面量

//     scope 1 {
//         debug t => _1;                   // 调试信息：t 对应 _1
//         let _6: &str;                    // 对应变量 ccc
//         scope 2 {
//             debug ccc => _6;             // 调试信息：ccc 对应 _6
//         }
//     }

//     bb0: {
//         _4 = const "123";                // 加载 "123" 字面量
//         _3 = <str as ToString>::to_string(move _4) -> bb1; // 调用 to_string()
//     }

//     bb1: {
//         _5 = const "444";                // 加载 "444" 字面量
//         // 创建 Test 实例并移动到 _2 (隐藏变量)
//         _2 = Test::<'_> { a: move _3, c: move _5 };
//         _1 = &_2;                        // t 是对 _2 的引用
//         _6 = copy ((*_1).1: &str);       // 复制 t.c 到 ccc
//         drop(_2) -> bb2;                 // 在作用域结束时 drop _2
//     }

//     bb2: {
//         return;                          // 函数返回
//     }
// }
fn main() {
    struct Test<'a> {
        a: String,
        c: &'a str,
    }

    // 这里会有一个隐藏的临时变量，所有权归属于这个临时变量，生命周期和t一致
    let t = &Test {
        a: "123".to_string(),
        c: "444",
    };

    // let aaa = t.a;
    let ccc = t.c;
}
