
use crate::Arena;
use super::*;

#[test]
fn curr_next() {
    let arena = Arena::new();
    let data: &[u8] = &[1, 2, 3];
    let mut reader = BufReader::new_with_capacity(&arena, data, 2);
    assert_eq!(reader.curr().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.curr().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.curr().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.curr().unwrap(), None);
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn cap_1() {
    let arena = Arena::new();
    let data: &[u8] = &[1, 2, 3];
    let mut reader = BufReader::new_with_capacity(&arena, data, 1);
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn skip_to() {
    let arena = Arena::new();
    let data: &[u8] = &[1, 2, 3, 4];
    let mut reader = BufReader::new(&arena, data);
    reader.skip_to(3).unwrap();
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), Some(4));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn tokens() {
    let arena = Arena::new();
    let data: &[u8] = b"Hello World!\t\r\n12 34 56";
    let mut reader = BufReader::new(&arena, data);
    assert_eq!(reader.read_token(&arena).unwrap(), b"Hello");
    assert_eq!(reader.read_token(&arena).unwrap(), b"World!");
    assert_eq!(reader.read_token(&arena).unwrap(), b"12");
    assert_eq!(reader.read_token(&arena).unwrap(), b"34");
    assert_eq!(reader.read_token(&arena).unwrap(), b"56");
    assert_eq!(reader.read_token(&arena).unwrap(), &[]);
}
