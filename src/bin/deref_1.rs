use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn display(self: &mut Person, age: u8) {
        let Person { name, age } = &&&&&&&self;
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    // &String 自动解引用
    let s = String::from("value");
    receive_pstr(&s);

    // 自定义Box
    let my_box = MyBox::new(123);
    // *my_box = *(my_box.deref())
    let c = 1 + *my_box;
    println!("{c}");
    receive_i32(&my_box);

    let mut my_str_box = MyBox::new(String::from("value"));
    let cc = my_str_box.deref();
    // &MyBox -> &String --> &str
    receive_pstr(&my_str_box);
    // 如果没有deref
    receive_pstr(&(*my_str_box)[..]);

    let s1: &str = &my_str_box;
    let s1: &String = &my_str_box;
    let s1: String = my_str_box.to_string();

    let s = &mut my_str_box;
    // DerefMut &mut MyBox<String> --> &mut String
    receive_mut_box(s);
}

fn receive_mut_box(s: &mut String) {}

fn receive_pstr(s: &str) {}

fn receive_i32(a: &i32) -> i32 {
    a * 2
}
