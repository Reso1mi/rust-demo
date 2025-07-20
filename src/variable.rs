// 常量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // 1.mut
    // lex x = 9;
    let mut x = 9;
    println!("x = {}", x);
    x = 5;
    println!("x = {}", x);

    println!("const = {}", THREE_HOURS_IN_SECONDS);

    // 2.shadow
    let b = 4;
    {
        // 内部作用域
        let b = b * 2;
        println!("b = {}", b);
    }
    println!("b = {}", b);

    let space = "        ";
    let space = space.len();
    println!("space = {}", space);

    // 不能改变mut变量类型
    // let mut space = "        ";
    // space = space.len(); ❌
    // println!("space = {}", space);

    // 3. 数据类型
    // let c = 34157i8; ❌
    // println!("c = {}", c);

    let f = 2.0; // 默认f64
    let y: f32 = 3.0;

    let sum = f + y;
    let diff = f - y;
    let product = f * y;
    let quotient = f / y;
    let floor = 2 / 3;

    let remainder = 12 / 5;

    println!(
        "sum = {}, diff = {}, pr = {}, q = {}, f = {}, r = {}",
        sum, diff, product, quotient, floor, remainder
    );

    let cc = 'z';
    let cz = 'ℤ';
    let heart_eyed_cat = '😻';
    // let strs = '131231'; ❌
    let strs = "12314a";
    println!(
        "cc = {}, cz = {}, h = {}, strs = {}",
        cc, cz, heart_eyed_cat, strs
    );

    // 元组 tup
    let tup = (1, 32.12, false);
    let (xx, yy, zz) = tup;
    println!("tup = {}, xx = {}, yy= {}, zz = {}", tup.2, xx, yy, zz);
    // tup.0 = 11; ❌

    // 数组
    let array = [1, 2, 3, 4, 5];
    println!("array[0] = {}", array[0]);

    let a1: [u32; 5] = [1, 2, 3, 4, 5];
    println!("a1 = {}", a1[0]);

    let a2 = [99; 10];
    println!("a2[2] = {}", a2[2]);
}
