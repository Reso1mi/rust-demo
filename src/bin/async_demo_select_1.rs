use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
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

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
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

#[tokio::main]
async fn main() {
    // test_select().await;
    // test_my_select().await;
    // test_select_2().await;
}
