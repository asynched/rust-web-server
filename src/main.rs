use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffered_reader = BufReader::new(&mut stream);
    let request_line = buffered_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./templates/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./templates/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let res = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(res.as_bytes()).unwrap();
}
