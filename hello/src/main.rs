extern crate hello;

use std::{fs::File, io::prelude::{Read, Write}, net::{TcpListener, TcpStream}};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(8);

    listener
        .incoming()
        .map(|stream| stream.unwrap())
        .for_each(|stream| pool.execute(|| handle_connection(stream)));

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 648];

    stream.read(&mut buffer).unwrap();

    let index = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(index) {
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
