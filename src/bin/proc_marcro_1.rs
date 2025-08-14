use hello_macro_derive::{HelloMacro, MyDefault_1};
use utils::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro, Debug)]
struct Sunface;

#[derive(MyDefault_1, Debug)]
struct SomeData(u32, String);

#[derive(MyDefault_1, Debug)]
struct User {
    name: String,
    data: SomeData,
}

fn main() {
    Sunfei::hello_macro();
    Sunface::hello_macro();
    println!("{:?}", User::default());
    println!("{:?}", String::default());
}
