use std::{sync::Arc, time::Duration};

fn test1() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hello");
        tx.send(val).unwrap();

        let rec = rx.recv().await.unwrap();
        println!("Got: {rec}");
    })
}

fn test2() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            println!("send: {val}");
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // 上面发送部分并没有异步，依然是顺序发送，只有全部send后，这里的recv才能开始接收
        while let Some(val) = rx.recv().await {
            println!("rec: {val}");
        }
    })
}

fn test3() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        let tx_fut = async {
            for val in vals {
                println!("send: {val}");
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // 目前接收端还无法自动关闭，因为没有收到发送端的关闭信号，程序无法自动停止
        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("rec: {val}");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

fn test4() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        // move将tx移动进异步代码块，当代码块结束，tx被drop，接收端关闭，程序停止（直接drop可能会丢失消息，需要控制好drop时机）
        let tx_fut = async move {
            for val in vals {
                println!("send: {val}");
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("rec: {val}");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

fn test5() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx2 = tx.clone();

        // move将tx移动进异步代码块，当代码块结束，tx被drop，接收端关闭，程序停止（直接drop可能会丢失消息，需要控制好drop时机）
        let tx_fut = async move {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                // println!("send: {val}");
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("rec: {val}");
            }
        };

        let tx_fut2 = async move {
            let vals = vec![
                String::from("tx_fut2:other"),
                String::from("tx_fut2:fut"),
                String::from("tx_fut2:send"),
            ];

            for val in vals {
                // println!("send: {val}");
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        trpl::join3(tx_fut, rx_fut, tx_fut2).await;
    });
}

fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    test5();
}
