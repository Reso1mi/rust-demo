fn main() {
    println!("Hello, World");
    another_func(8917, 'c');
    println!("five = {}", five());
    println!("99 + 1 = {}", incr(99));

    let number = 10;
    if number > 100 {
        println!("number was more than 100")
    } else {
        println!("number was less than 100")
    }

    let condition = true;

    let cn = if condition { 5 } else { 10 };

    println!("cn = {}", cn);

    let res: u32 = loop {
        break 5;
    };

    println!("res = {}", res);

    
}

fn another_func(x: u32, unit_lable: char) {
    println!("Another func, x = {}, c = {}", x, unit_lable);
}

fn five() -> u32 {
    5
}

fn incr(x: u32) -> u32 {
    x + 1
}
