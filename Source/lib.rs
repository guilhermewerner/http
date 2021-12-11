#![allow(non_snake_case)]

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub fn HandleConnection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "Hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("Resources/{}", filename)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
