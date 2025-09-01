use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};

use std::pin::pin;

use tokio::{
    io::AsyncWriteExt as _,
    net::{TcpListener, TcpStream},
    select,
    sync::{mpsc, oneshot},
};

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

async fn test_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    let out = tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
            // Ok(())
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    };

    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        // 用 tx1 和 tx2 干一些不为人知的事
        tx1.send(1).await.unwrap();
        tx2.send(2).await.unwrap();
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        // else
        else => {
            println!("Both channels closed");
        }
    }
}

async fn test_my_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // 使用 tx1 和 tx2

    MySelect { rx1, rx2 }.await;
}

fn process_socket(socket: TcpStream) {}

async fn test_select_2() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    // 生成一个任务，用于向 oneshot 发送一条消息
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });

    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        _ = async {
            loop {
                let (socket, addr) = listener.accept().await?;
                tokio::spawn(async move {process_socket(socket)});

            }
            // 添加 Ok::<_, io::Error>(()) 后：
            // 明确告诉编译器这个 async 块返回 Result<(), io::Error>
            // ? 操作符现在知道它可以在这个 async 块的上下文中正常工作
            // 如果 listener.accept() 出错，? 会让这个 async 块返回错误，而不是试图从外层函数返回
            #[allow(unreachable_code)]
            Ok::<_, io::Error>(())
        } => {
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
    };
    Ok(())
}

async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    let mut out = String::new();

    // 这里如果其中一个连接失败并不会直接返回，连接失败后返回的是Err无法匹配Ok所以另一个分支会继续执行
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            // 不可变借用
            socket.write_all(data).await?;
            // 可变借用
            out.push_str("模式匹配");
            println!("{:?}", out);
            // 类型标注
            Ok::<_, io::Error>(())
        } => {
            out.push_str("结果处理");
        }
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            // 不可变借用
            socket.write_all(data).await?;
            // println!("{:?}", out);
            // 这里不能再进行可变借用
            // out.push_str("模式匹配");
            // 类型标注
            Ok::<_, io::Error>(())
        } => {
            // 两个分支只会有一个执行成功，所以这里可以进行可变借用
            out.push_str("结果处理");
        }
        else => {}
    };

    Ok(())
}

async fn test_loop_select() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);
    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {}", msg);
    }
}

async fn action() {}

// 定义一个返回 Unpin Future 的异步函数
async fn unpin_action() -> u32 {
    println!("Unpin action started");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Unpin action completed");
    42
}

async fn test_select_recover() {
    let (mut tx, mut rx) = mpsc::channel(128);
    tx.send(1);

    let op = action();
    // let mut op = pin!(op);

    tokio::pin!(op);
    let mut op = op;
    // let mut op = &mut op;

    loop {
        select! {
            // &mut Pin<op> -> derefMut -> &mut op
            _ = &mut op => break,
            Some(v) = rx.recv() => {
                if v%2 == 0 {
                    break;
                }
            }
        }
    }
}

async fn test_mod_other_select() {
    async fn action(input: Option<i32>) -> Option<String> {
        // 若 input（输入）是None，则返回 None
        // 事实上也可以这么写: `let i = input?;`
        let i = match input {
            Some(input) => input,
            None => return None,
        };

        // 这里定义一些逻辑
        Some(i.to_string())
    }

    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    // 已经完成的future不能再恢复，这里通过set重新设置了新的Future
                    // `.set` 是 `Pin` 上定义的方法
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // test_select().await;
    // test_my_select().await;
    // test_select_2().await;
    test_mod_other_select().await;
}
