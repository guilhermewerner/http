// Copyright (c) TribuFu. All Rights Reserved.

#![allow(non_snake_case)]

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[doc(hidden)]
pub fn Main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        HandleConnection(stream);
    }
}

fn HandleConnection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "Hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("Resources/{}", filename)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
