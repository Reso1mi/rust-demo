enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Holder {
    China,
    USA,
    Japan,
}

enum Coin {
    BTC,
    ETH,
    SOL(Holder),
}

impl Coin {
    fn get_coin_value(&self) -> u32 {
        match self {
            Coin::BTC => {
                println!("Best BTC!");
                1
            }
            Coin::ETH => 2,
            Coin::SOL(holder) => {
                println!("SOL! {holder:?}!");
                3
            }
        }
    }
}

fn main() {
    let local = IpAddr::V4(String::from("127.0.0.1"));

    dbg!(local);

    let some_int = Some(32);
    let some_char = Some('b');

    let null_int: Option<i32> = None;

    let sol_coin = Coin::SOL(Holder::China);
    let v = sol_coin.get_coin_value();
    println!("v = {v}");

    fn plus_one(x: Option<u32>) -> Option<u32> {
        match x {
            Some(n) => Some(n + 1),
            None => None, // 匹配必须是穷尽的
                          // other => None, // 最后一个匹配所有未出现
        }
    }

    let x = 1;
    println!("x + 1 = {:?}", plus_one(Some(x)).expect("error"));

    let x = None;
    println!("x + 1 = {:?}", plus_one(x).unwrap_or_default());

    // if let else 语法

    let x = Some(9);
    if let Some(v) = x {
        println!("x.v = {}", v);
    };

    let Coin::SOL(hd) = sol_coin else {
        return;
    };

    println!("sol_coin holder = {hd:?}");
}
