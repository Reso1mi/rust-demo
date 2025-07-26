#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    println!("Hello, world! {}", "Rust");
    println!("Guess the number!");

    let secert_number = rand::thread_rng().gen_range(1..101);
    // println!("sccert_numver:{}", secert_number);

    // Rust中默认变量不可变
    let cant_modify_var = 100;
    println!("cant_modify_var:{}", cant_modify_var);
    // cant_modify_var = 200;

    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Fail to read line");

    loop {
        // mut可变变量
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secert_number) {
            Ordering::Less => println!("You guess too small!"),
            Ordering::Greater => println!("You guess too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
