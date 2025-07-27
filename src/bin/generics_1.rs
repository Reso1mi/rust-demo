use std::i32;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn mixup<Z, W>(self, p: Point<U, W>) -> Point<T, W> {
        Point { x: self.x, y: p.y }
    }
}

pub fn main() {
    let list = vec![1, 4, -1, 231, 24, 0x12345];
    println!("list largest = {}", lagerst_i32(&list));
    let p = Point { x: 1, y: 2 };
    // let p = Point { x: 1, y: 2.0 }; // 类型不匹配
}

fn lagerst_i32(list: &Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    // 模式匹配
    for &v in list {
        max = if v > max { v } else { max };
    }
    max
}

fn largest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut max = &list[0];
    // 模式匹配
    for v in list {
        max = if v > max { v } else { max };
    }
    max
}

fn largest_2<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut max = list[0];
    // 模式匹配
    for &v in list {
        max = if v > max { v } else { max };
    }
    max
}
