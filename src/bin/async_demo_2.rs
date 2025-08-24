use std::time::Duration;

fn test_1() {
    trpl::run(async {
        let fut = trpl::spawn_task(async {
            for i in 1..10 {
                println!("number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        let _ = fut.await;
    });
}

fn test_2() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // 在当前Future阻塞等待fut1和fut2
        trpl::join(fut1, fut2).await;
    });
}

fn main() {
    // test_1();
    // test_2();
}
