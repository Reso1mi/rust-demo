pub fn main() {
    let mut s = String::from("value");
    test2(&mut s);
    println!("s = {s}");

    let s2 = String::from("value2");
    test(s2);

    let mut s3 = String::from("value3");
    test3(&mut s3);
    println!("s3 = {s3}");
}

fn test(mut s: String) {
    let mut other_string = String::from("other");
    s.push_str("123");
    println!("s = {s}");
    s = other_string;
}

fn test2(s: &mut String) {
    s.push_str("123");
    println!("s = {s}");
}

fn test3(mut s: &mut String) {
    s.push_str("123");
    println!("s = {s}");
}
