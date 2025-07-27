#![allow(unused)]

// src/lib.rs
pub fn greet() {
    println!("Hello from the library!");
}
#[cfg(test)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod demo;
#[cfg(test)]
mod tests {

    use super::*;
    use demo::struct_1;
    use std::panic;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn bulid_user() {
        let email = String::from("email");
        let name = String::from("name");
        let u = struct_1::build_user(&email, &name, true);

        let u2;
        {
            let email2 = String::from("email");
            {
                let name2 = String::from("name");
                // 返回的u2生命周期和email2以及name2最短的一致
                u2 = struct_1::build_user(&email2, &name2, true);
            }
            // println!("{:?}", u2.email);
        }
        // println!("{:?}", u2.email);
    }
}
