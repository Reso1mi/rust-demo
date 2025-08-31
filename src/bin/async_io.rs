use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn test_read() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    // let mut buffer = [0; 10];

    // 由于 buffer 的长度限制，当次的 `read` 调用最多可以从文件中读取 10 个字节的数据
    // let n = f.read(&mut buffer[..]).await?;
    // println!("The bytes: {:?}", &buffer[..n]);

    let mut buffer = Vec::new();
    let n = f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    println!("The bytes String: {:?}", String::from_utf8(buffer));
    Ok(())
}

async fn test_write() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write(b"some test bytes").await?;

    Ok(())
}

async fn test_delete() -> io::Result<()> {
    fs::remove_file("foo.txt").await
}

async fn test_copy() -> io::Result<()> {
    let mut reader = b"hello";
    let mut f: File = File::create("copy.txt").await?;
    io::copy(&mut &reader[..], &mut f).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = test_write().await {
        eprintln!("async write error: {:?}", e);
    }

    if let Err(e) = test_read().await {
        eprintln!("async read error: {:?}", e);
    }

    if let Err(e) = test_delete().await {
        eprintln!("async delete error: {:?}", e);
    }

    if let Err(e) = test_copy().await {
        eprintln!("async copy error: {:?}", e);
    }
}
