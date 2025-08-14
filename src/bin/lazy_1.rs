use std::{sync::LazyLock, thread};

fn main() {
    // 子线程中调用
    let handle = thread::spawn(|| {
        let logger = &LOGGER;
        logger.log("thread message".to_string());
    });

    // 主线程调用
    let logger = &LOGGER;
    logger.log("some message".to_string());

    let logger2 = &LOGGER;
    logger2.log("other message".to_string());

    handle.join().unwrap();
}

#[derive(Debug)]
struct Logger;

static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

impl Logger {
    fn new() -> Logger {
        println!("Logger is being created...");
        Logger
    }

    fn log(&self, message: String) {
        println!("{}", message)
    }
}
