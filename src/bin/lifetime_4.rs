#[derive(Debug)]
struct Pair {
    name: String,
}

// CallBuilder 内部持有对 Pair 的引用
struct Call<'a> {
    pair: &'a Pair,
}

impl Pair {
    fn get_reserves(&self) -> Call {
        Call { pair: self }
    }
}

fn main() {
    // --------- 错误示例 ---------
    // 下面会报错：
    // error[E0716]: temporary value dropped while borrowed

    // let call = Pair {
    //     name: "A".to_string(),
    // }
    // .get_reserves();

    // println!("Call uses pair: {:?}", call.pair.name);

    // --------- 正确示例 ---------
    // 保存 Pair 实例
    // let mut pairs = Vec::new();
    // pairs.push(Pair {
    //     name: "A".to_string(),
    // });
    // pairs.push(Pair {
    //     name: "B".to_string(),
    // });

    // // get_reserves 的返回值依然有效
    // let calls: Vec<Call> = pairs.iter().map(|p| p.get_reserves()).collect();

    // for call in calls {
    //     println!("Call uses pair: {}", call.pair.name);
    // }
}
