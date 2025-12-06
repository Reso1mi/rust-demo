use std::ops::Deref;

// 没有实现 Copy 的类型
// #[derive(Clone, Copy)]
struct NoCopy {
    data: u8,
}

impl Deref for NoCopy {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn main() {
    let no_copy = NoCopy { data: 1 };
    let ref_no_copy = &no_copy;
    let moved_value = *no_copy; // 解引用成功，但发生了 Move

    println!("{}", no_copy.data);
}

// println!("{}", no_copy.data); // 编译错误：no_copy 已被移动
