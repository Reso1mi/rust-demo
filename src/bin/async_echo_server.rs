use std::{
    collections::HashMap,
    fmt::format,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Tiny-Redis bind {addr} error!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    println!("Tiny-redis running on {addr}");

    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.expect("Tiny-Redis Accept error!");
        println!("Conn Socket Addr: {socket_addr}");
        let db = db.clone();
        // 交给 tokio runtime
        tokio::spawn(async {
            if let Err(e) = process(tcp_stream, db).await {
                println!("Connection failed: {}", e);
            }
        });
    }
}

async fn process(tcp_stream: TcpStream, db: Db) -> Result<()> {
    let mut conn: Connection = Connection::new(tcp_stream);
    while let Some(frame) = conn.read_frame().await? {
        let response = match Command::from_frame(frame)? {
            Command::Get(get) => {
                if let Ok(db) = db.lock() {
                    if let Some(val) = db.get(get.key()) {
                        Frame::Bulk(val.clone().into())
                    } else {
                        Frame::Null
                    }
                } else {
                    Frame::Simple("Error".into())
                }
            }
            Command::Set(set) => {
                if let Ok(mut db) = db.lock() {
                    db.insert(set.key().to_string(), set.value().clone());
                    Frame::Simple("OK".into())
                } else {
                    Frame::Simple("Error".into())
                }
            }
            other_cmd => Frame::Error(format!("Unimplment: {:?}", other_cmd)),
        };

        // 响应客户端
        conn.write_frame(&response).await?;
    }

    if let Some(frame) = conn.read_frame().await? {
        println!("Get Frame: {:?}", frame);
        let resp = Frame::Error("unimpl".into());
        conn.write_frame(&resp).await?;
    };

    Ok(())
}
