use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:6789";
    let Ok(listener) = TcpListener::bind(addr) else {
        panic!("bind {addr} error");
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => println!("conn success!"),
            Err(e) => eprintln!("conn error {e:?}"),
        }
    }
}
