struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > 10 {
            None
        } else {
            Some(self.count)
        }
    }
}

// 标准库为所有 Iterator 自动实现 IntoIterator
// impl<I: Iterator> IntoIterator for I {
//     type Item = I::Item;
//     type IntoIter = I;

//     #[inline]
//     fn into_iter(self) -> I {
//         self
//     }
// }

fn main() {
    let count = Counter::new();
    // count moved due to this implicit call to .into_iter()
    for i in count {
        // 标准库为所有 Iterator 自动实现 IntoIterator
        // for i in count.into_iter() { 等价这一行
        println!("{i}");
    }
    // println!("{:?}", count.count);
}
