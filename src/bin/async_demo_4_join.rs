use std::{
    pin::{Pin, pin},
    thread,
    time::Duration,
};

fn test1() {
    trpl::run(async {
        let f1 = async {};

        let f2 = async {};

        let f3 = async {};

        // trpl::join!(f1, f2, f3);

        // 编译器会各个异步代码块都会生成对应的struct类型，所以这里无法将不同的struct放入同一个vec中
        // let fv = vec![f1, f2, f3];
        // let fv = vec![Box::new(f1), Box::new(f2), Box::new(f3)];
        // let fv: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(f1), Box::new(f2), Box::new(f3)];
        let fv: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(f1), Box::pin(f2), Box::pin(f3)];
        trpl::join_all(fv).await;
    })
}

fn test2() {
    trpl::run(async {
        let f1 = pin!(async {});

        let f2 = pin!(async {});

        let f3 = pin!(async {});
        // 和上面不是同一个类型
        let c = pin!(async { true });

        // trpl::join!(f1, f2, f3);

        // 编译器会各个异步代码块都会生成对应的struct类型，所以这里无法将不同的struct放入同一个vec中
        // let fv = vec![f1, f2, f3];
        // let fv = vec![Box::new(f1), Box::new(f2), Box::new(f3)];
        // let fv: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(f1), Box::new(f2), Box::new(f3)];
        let fv: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![f1, f2, f3];
        trpl::join_all(fv).await;
    })
}

fn test3_race() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    })
}

fn test_yidld() {
    fn slow(name: &str, ms: u64) {
        // 使用thread.sleep模拟cpu密集工作，这里实际上是忙等待，不同于trpl中的sleep，这里会阻塞当前线程
        thread::sleep(Duration::from_millis(ms));
        println!("'{name}' ran for {ms}ms");
    }

    trpl::run(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            // 通过sleep await休眠交还控制权，否者会一直执行完，b无法插入任务
            // trpl::sleep(one_ms).await;
            // 直接交还，不通过sleep
            trpl::yield_now().await;
            slow("a", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow("a", 20);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            println!("'a' finished");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow("b", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow("b", 15);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow("b", 500);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;
            println!("'b' finished");
        };

        trpl::race(a, b).await;
    })
}

fn main() {
    // test3_race();
    test_yidld();
}
