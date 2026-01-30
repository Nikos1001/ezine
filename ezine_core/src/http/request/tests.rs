
use crate::Arena;
use super::*;

#[test]
fn basic() {
    let arena = Arena::new();
    let request_str = b"GET / HTTP/1.1";
    let request = HttpRequest::parse(&arena, request_str).unwrap(); 
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.uri, "/");
    assert!(request.headers.is_empty());
    assert!(request.body.is_empty());
}

#[test]
fn methods() {
    let arena = Arena::new();

    let get_str = b"GET / HTTP/1.1";
    let get = HttpRequest::parse(&arena, get_str).unwrap(); 
    assert_eq!(get.method, HttpMethod::GET);

    let post_str = b"POST / HTTP/1.1";
    let post = HttpRequest::parse(&arena, post_str).unwrap(); 
    assert_eq!(post.method, HttpMethod::POST);

    let put_str = b"PUT / HTTP/1.1";
    let put = HttpRequest::parse(&arena, put_str).unwrap(); 
    assert_eq!(put.method, HttpMethod::PUT);

    let delete_str = b"DELETE / HTTP/1.1";
    let delete = HttpRequest::parse(&arena, delete_str).unwrap(); 
    assert_eq!(delete.method, HttpMethod::DELETE);

    let bogus_str = b"BOGUS / HTTP/1.1";
    assert!(HttpRequest::parse(&arena, bogus_str).is_none()); 
}

#[test]
fn headers() {
    let arena = Arena::new();
    let request_str = b"GET / HTTP/1.1\r\nTest: 123\r\nAnother: 12:34";
    let request = HttpRequest::parse(&arena, request_str).unwrap(); 
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.uri, "/");
    assert_eq!(request.headers.as_slice(), &[
        ("Test", "123"),
        ("Another", "12:34")
    ]);
    assert!(request.body.is_empty());   
}

#[test]
fn body() {
    let arena = Arena::new();
    let request_str = b"GET / HTTP/1.1\r\n\r\nThe Body";
    let request = HttpRequest::parse(&arena, request_str).unwrap(); 
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.uri, "/");
    assert!(request.headers.is_empty());
    assert_eq!(request.body, b"The Body");
}

#[test]
fn headers_and_body() {
    let arena = Arena::new();
    let request_str = b"GET / HTTP/1.1\r\nTest: 123\r\nAnother: 12:34\r\n\r\nA body!";
    let request = HttpRequest::parse(&arena, request_str).unwrap(); 
    assert_eq!(request.method, HttpMethod::GET);
    assert_eq!(request.uri, "/");
    assert_eq!(request.headers.as_slice(), &[
        ("Test", "123"),
        ("Another", "12:34")
    ]);
    assert_eq!(request.body, b"A body!");
}
