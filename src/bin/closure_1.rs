use std::{collections::HashMap, fmt::Display, hash::Hash, thread, time::Duration};

pub fn main() {
    let list = vec![1, 2, 3, 4];
    let borrow_print = || println!("close: {list:?}");
    println!("before borrow_print: {list:?}");
    borrow_print();
    println!("after borrow_print:{list:?}");

    let mut list_mut = vec![1, 2, 3, 4];
    println!("before mut_borrow_push: {list_mut:?}");
    let mut mut_borrow_push = || list_mut.push(5);
    // 无法获取不可变引用，因为闭包中获取呢可变引用
    // println!("before mut_borrow_push: {list_mut:?}");
    mut_borrow_push();
    println!("after mut_borrow_push: {list_mut:?}");

    let list_move = vec![1, 2, 3];
    println!("before list_move: {list_move:?}");
    // closure may outlive the current function, but it borrows `list`, which is owned by the current function
    // force the closure to take ownership of `list`
    thread::spawn(move || println!("From thread: {list_move:?}"))
        .join()
        .unwrap();
    // value borrowed here after move
    // println!("after list_move: {list_move:?}");
    //

    let expensive_closure = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
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
{
    fn new(calc: F) -> Self {
        Cache {
            calc: calc,
            result_map: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: K) -> &V {
        // let mut result_map = &;
        // match self.result_map.get(&arg) {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calc)(&arg);
        //         self.result_map.insert(arg.clone(), v);
        //         todo!()
        //     }
        // }
        if !self.result_map.contains_key(&arg) {
            let v = (self.calc)(&arg);
            self.result_map.insert(arg.clone(), v);
        }
        todo!()
    }
}
