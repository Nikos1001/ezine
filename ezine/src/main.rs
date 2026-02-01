
use std::net::TcpListener;
use ezine_core::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut arena = Arena::new();

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue; };
        let Some(request) = HttpRequest::read(&arena, &mut stream) else { continue; };

        let content = format!("<html>Hello, World!</br>Your URI: {}</html>", request.uri);
        let response = HttpResponse::new(200)
            .with_body(content.as_bytes())
            .with_header(&arena, "Content-Type", "text/html");
        let _ = response.write(stream);

        arena.clear();
    }

}
