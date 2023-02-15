use std::time::Duration;
use std::thread;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;



fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:4200").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }


}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0;1024];
    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_code, filename) = {
        if buf.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        contents.len(),
        contents
        );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}