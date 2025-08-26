use std::cmp::min;
use std::fs;
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::task;
use async_std::task::spawn;
use futures::AsyncReadExt;
use futures::AsyncWriteExt;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let addr = "127.0.0.1:6789";
    // str 实现了ToSocketAddrs
    let Ok(listener) = TcpListener::bind("127.0.0.1:6789").await else {
        panic!("bind {addr} error")
    };

    println!("webserver is running on {}", addr);

    listener
        .incoming()
        .for_each_concurrent(None, |tcp_stream| async {
            match tcp_stream {
                Ok(tcp_stream) => match spawn(handle_conn(tcp_stream)).await {
                    Err(e) => eprintln!("handle tcp stream error: {e}"),
                    _ => {}
                },
                Err(e) => eprintln!("accept tcp stream error: {e}"),
            }
        })
        .await;
}

// pub use futures_io::AsyncRead as Read; 重导出了
async fn handle_conn(mut stream: impl Read + Write + Unpin) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (http_status_line, resource_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/static/index.html")
    } else if buffer.starts_with(sleep) {
        // 让出控制权，非阻塞线程的睡眠
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "src/static/404.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/static/404.html")
    };

    let contents = fs::read_to_string(resource_name)?;
    let response = format!("{http_status_line}{contents}");
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}
use std::marker::Unpin;

impl Unpin for MockTcpStream {}

impl Read for MockTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let size = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Poll::Ready(Ok(size))
    }
}

impl Write for MockTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        self.get_mut().write_data = Vec::from(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    let _ = handle_conn(&mut stream).await;
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).await.unwrap();

    let expected_contents = fs::read_to_string("src/static/index.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}
