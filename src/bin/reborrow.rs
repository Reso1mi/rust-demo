#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let x = Point { x: 1, y: 1 };
    let mut p = Point { x: 0, y: 0 };

    // 1. 创建可变引用
    let r = &mut p; // 可变借用开始：独占访问整个p

    // 2. 创建再借用
    let rr = &*r; // 关键操作：解引用后重新借用 -> 不可变借用
    // 解释：
    //   *r 解引用得到 p 的值，但不是转移所有权
    //   &*r 创建指向相同数据的不可变引用
    //   rr 的存在使 r 暂时冻结（不能直接使用）

    // 3. 错误尝试（注释部分）
    // let rrr = &p;   // ❌ 直接再借用p会冲突
    // 原因：
    //   p 已被可变借用 (通过 r)，不能同时创建新的不可变借用

    println!("{:?}", rr); // ✅ 使用不可变引用
    // rr 最后一次使用 -> 不可变借用结束

    // 4. 恢复可变引用使用
    r.move_to(10, 10); // ✅ 可变引用再次可用
    r.x = 1001; // ✅ 可直接修改字段
    println!("{:?}", r); // 使用 r

    // 5. 错误尝试（注释部分）
    // println!("{:?}", rr); // ❌ rr 已失效
    // 原因：最后一次使用后已释放，且现在有活跃的可变引用
}

fn read_length(strings: &mut Vec<String>) -> usize {
    // 合法操作：内部将可变引用降级为不可变
    strings.len() // 等价于 (&*strings).len()
}
