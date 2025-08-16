struct Test {
    a: String,
    b: u8,
}
fn main() {
    let t = Test {
        a: "123".to_string(),
        b: 1,
    };

    let z = t.a;

    println!("{}", t.b);

    // let x = t.a;
    // println!("{z}");
    // println!("{x}");
}

// fn get_test(t: Test) -> &'static str {
//     &t.a
// }
