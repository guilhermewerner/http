#![allow(non_snake_case)]

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        Http::HandleConnection(stream.unwrap());
    }
}
