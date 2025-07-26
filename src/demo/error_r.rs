use std::io::{Error, ErrorKind, Read, Write};
use std::{fs, path};
use std::{fs::File, ops::Not};

pub fn main() {
    // panic!("panic occur!");
    let v = vec![1, 2, 3, 4];
    // painc!
    // v[111];
    let path = "./test.txt";
    let f = match File::open(path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(f) => f,
                Err(e) => panic!("creat error: {:?}", e),
            },
            other => panic!("error"),
        },
    };
    println!("f = {:?}", f.metadata());

    let mut f = File::options().read(true).write(true).open(path).unwrap();
    match f.write("Tadow".as_bytes()) {
        Ok(o) => println!("ok = {:?}", o),
        Err(e) => panic!("write error, {:?}", e),
    };

    dbg!(read_name_from_text(path).unwrap());
    dbg!(read_name_from_text2(path).unwrap());
    dbg!(read_name_from_text3(path).unwrap());
}

fn read_name_from_text(path: &str) -> Result<String, Error> {
    let mut f = match File::open(path) {
        Ok(f) => f,
        // 提前返回
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_name_from_text2(path: &str) -> Result<String, Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_name_from_text3(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}
