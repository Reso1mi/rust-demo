use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn test() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        // StreamExt:Stream 扩展了 Stream trait
        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    })
}

fn test_stream_timeout() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                tx.send(format!("Message: '{message}'")).unwrap();
            }
        });
        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(msg) => println!("{msg}"),
                Err(e) => eprintln!("Problem: {e:?}"),
            }
        }
    });
}

fn test_stream_merge() {
    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                    eprintln!("Cannot send message '{message}': {send_error}");
                    break;
                }
            }
        });
        ReceiverStream::new(rx)
    }

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });
        ReceiverStream::new(rx)
    }

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Intervals: '{count}'"))
            // 限流，包装成新的流，间隔duration
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_millis(10));
        let mut merged = pin!(messages.merge(intervals).take(20));

        while let Some(result) = merged.next().await {
            match result {
                Ok(msg) => println!("{msg}"),
                Err(e) => eprintln!("Problem: {e:?}"),
            }
        }
    });
}

fn main() {
    // test();
    // test_stream_timeout();
    test_stream_merge();
}
