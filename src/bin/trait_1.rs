use std::fmt::{Display, format};

pub trait Summary {
    fn summary(&self) -> String;

    fn more(&self) -> String {
        "test".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summary(&self) -> String {
        format!(
            "nw-summary: {}-{}-{}-{}",
            self.headline, self.location, self.author, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!(
            "tw-summary: {}-{}-{}-{}",
            self.content, self.reply, self.retweet, self.username
        )
    }
}

// trait作为参数传递
fn notify(item: impl Summary) {
    println!("notify, hi, {}", item.summary());
}

// 适合复杂场景
fn notify_2<T: Summary>(item: T) {
    println!("notify2, hi, {}", item.summary());
}

// 限制两个参数必须类型一致, 这样只能保证两个都实现了Summary
fn notify_3(item_a: impl Summary, item_b: impl Summary) {}

// 这样就能保证两个参数类型一致
fn notify_4<T: Summary>(item_a: T, item_b: T) {}

// +指定多个类型的trait
fn notify_5(item: impl Summary + Display) {}
// Trait Bound 同样适用
fn notify_6<T: Summary + Display>(item: T) {}

// where语法指定，避免函数签名过长
fn notify_7<T>(item: T)
where
    T: Summary + Display,
{
}

// trait作为返回值，单一类型可以，多个类型if-else不行
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best
//             hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn largest<T: PartialOrd>(list: &Vec<T>) -> &T {
    let mut max = &list[0];
    // 模式匹配
    for v in list {
        max = if v > max { v } else { max };
    }
    max
}

fn largest_2<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    // 需要copy
    let mut max = list[0];
    // 模式匹配，解构需要copy
    for &v in list {
        max = if v > max { v } else { max };
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new() {
        todo!()
    }
}

// 对于实现了特定trait的类，实现方法
impl<T: Summary + Display> Point<T> {
    fn test(&self) {
        todo!()
    }
}

// 对于实现了特定trait的类，实现trait
impl<T: Summary + Display> ToString for Point<T> {
    fn to_string(&self) -> String {
        todo!()
    }
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let list = vec![String::from("A"), String::from("B")];
    // let max = largest_2(&list);
    let max = largest(&list); // max 是 &String
    println!("{}", max); // "B"
}
