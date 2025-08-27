use std::{
    fs,
    io::{BufRead, BufReader, Error, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let addr = "127.0.0.1:6789";
    let Ok(listener) = TcpListener::bind(addr) else {
        panic!("bind {addr} error");
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_conn(stream) {
                Err(e) => eprintln!("handle error: {e:?}"),
                _ => {}
            },
            Err(e) => eprintln!("conn error {e:?}"),
        }
    }
}

fn handle_conn(mut stream: TcpStream) -> Result<(), Error> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/static/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/static/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/static/404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line} \r\nContent-Length: {length}\r\n\r\n\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
