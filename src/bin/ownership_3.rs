use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub fn main() {
    {
        struct StrOpt {
            value: Option<String>,
        }

        let str_opt = &StrOpt {
            value: Some("123".to_string()),
        };

        let v = match &str_opt.value {
            Some(v) => v,
            None => "1",
        };

        // if raw.raw_str == Some("123".to_string()) {
        //     raw.raw_str = None;
        // }
    }

    {
        struct StrOpt {
            value: Option<String>,
        }

        let str_opt = StrOpt {
            value: Some("123".to_string()),
        };

        let v = match str_opt.value {
            Some(v) => v,
            None => "1".to_string(),
        };
    }

    {
        struct StrOpt<'a> {
            value: Option<&'a str>,
        }

        let str_opt = StrOpt { value: Some("123") };

        let v = match str_opt.value {
            Some(v) => v,
            None => "1",
        };
    }

    {
        struct Int {
            raw_int: Option<u32>,
        }

        let raw = Int { raw_int: Some(123) };

        let s = &raw;

        let v = match s.raw_int {
            Some(v) => v,
            None => 1,
        };
    }

    {
        struct StrOpt {
            value: String,
            test: String,
        }

        impl StrOpt {
            fn get_value(&self) -> &String {
                &self.value
            }
        }

        let mut str_opt = StrOpt {
            value: "123".to_string(),
            test: "23".to_string(),
        };

        // 通过&self获取获取不可变引用
        // let v = str_opt.get_value();
        // move 获取所有权
        // let v = str_opt.value;
        // println!("{:?}", str_opt.value);
        // 直接获取value不可变引用
        let v = &str_opt.value;

        str_opt.test.push_str("string");

        // 最后一次使用不可变借用
        println!("{v}");

        let mut str_opt2 = &StrOpt {
            value: "123".to_string(),
            test: "23".to_string(),
        };
        // str_opt2.value;
    }
}

struct Cache<F, K, V>
where
    F: Fn(&K) -> V,
    K: Eq + Hash + Clone,
{
    calc: F,
    result_map: HashMap<K, V>,
}

impl<F, K, V> Cache<F, K, V>
where
    F: Fn(&K) -> V,
    K: Eq + Hash + Clone,
    V: Debug,
{
    fn new(calc: F) -> Self {
        Cache {
            calc: calc,
            result_map: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: K) -> &V {
        // 获取了result_map不可变引用
        let v: Option<&V> = self.result_map.get(&arg);
        // 整个 match v { ... } 被视为​​单一表达式​​
        // v 的生命周期会贯穿整个 match 块直到结束
        match v {
            Some(v) => v,
            None => {
                let z = (self.calc)(&arg);
                // self.result_map.insert(arg, z);
                // println!("{:?}", v);
                todo!()
            }
        }
        // v.unwrap();

        // if v.is_none() {
        //     let c = (self.calc)(&arg);
        //     // 借用可变引用
        //     self.result_map.insert(arg.clone(), c);
        //     // todo!()
        // }
        //      self（不可变借用）
        //         |
        //         v
        //     self.result_map（Map 的不可变借用）
        //         |
        //         v
        //     Option<&V>（值的引用）
        // 最后一次使用不可变引用
        // 这个依赖链意味着：​​只要 v 还在使用，整个链条都视为"被借用"状态​​
        // v.unwrap();
        // todo!()
    }
}
