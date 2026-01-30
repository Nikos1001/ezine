
use std::{io::Write, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let content = "<html>Hello, World!</html>";
    let msg = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{content}", content.len());

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue; };
        stream.write(msg.as_bytes()).unwrap();
    }

}
