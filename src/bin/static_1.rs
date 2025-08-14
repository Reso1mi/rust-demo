use lazy_static::lazy_static;
use std::{
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

const MAX_ID: usize = usize::MAX / 2;
static mut REQUEST_RECV: usize = 0;
static ATOM_REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);
// static NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

fn main() {
    println!("用户ID允许的最大值是{}", MAX_ID);

    // unsafe {
    //     REQUEST_RECV += 1;
    //     assert_eq!(REQUEST_RECV, 1);
    // };
    ATOM_REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    println!("{:?}", ATOM_REQUEST_RECV);

    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}

#[derive(Debug)]
struct Config {
    a: u8,
    b: String,
}
static mut CONFIG: Option<&'static mut Config> = None;

use std::sync::OnceLock;

static CONFIG_2: OnceLock<Config> = OnceLock::new();

fn test2() {
    // 初始化一次
    CONFIG_2.get_or_init(|| Config {
        a: 42,
        b: "Hello".to_string(),
    });

    thread::spawn(move || {
        // 安全读取
        let config = CONFIG_2.get().unwrap();
        println!("Config.a: {}", config.a);
    });

    CONFIG_2.get().unwrap();
}

fn test_box_leak() {
    let c = Box::new(Config {
        a: 1,
        b: "B".to_string(),
    });
    unsafe {
        CONFIG = Some(Box::leak(c));
        // println!("{:?}", CONFIG);
    };

    let c = 1;
    let mut v = "s13".to_string();
    // thread::spawn(move || {
    //     unsafe { &CONFIG.unwrap().a };
    //     v.push_str("string");
    // });
    // v.push_str("string");
}
