
use super::*;

#[test]
fn simple() {
    let data = [1, 2, 3, 4, 5];
    let mut split = Splitter::new(&data, &[2]); 
    assert_eq!(split.next(), Some(&data[..1]));
    assert_eq!(split.next(), Some(&data[2..]));
    assert_eq!(split.next(), None);
}

#[test]
fn target() {
    let data = [1, 2, 3, 4, 5];
    let mut split = Splitter::new(&data, &[2, 3]); 
    assert_eq!(split.next(), Some(&data[..1]));
    assert_eq!(split.next(), Some(&data[3..]));
    assert_eq!(split.next(), None);
}

#[test]
fn long_target() {
    let data = [1, 2, 3];
    let mut split = Splitter::new(&data, &[1, 2, 3, 4, 5]); 
    assert_eq!(split.next(), Some(&data[..]));
}

#[test]
fn multi() {
    let data = [1, 2, 3, 2, 4, 5];
    let mut split = Splitter::new(&data, &[2]); 
    assert_eq!(split.next(), Some(&data[..1]));
    assert_eq!(split.next(), Some(&data[2..3]));
    assert_eq!(split.next(), Some(&data[4..]));
    assert_eq!(split.next(), None);
}

#[test]
fn left_target() {
    let data = [1, 1, 1, 2];
    let mut split = Splitter::new(&data, &[1, 1]); 
    assert_eq!(split.next(), Some(&data[..0]));
    assert_eq!(split.next(), Some(&data[2..]));
    assert_eq!(split.next(), None);
}

#[test]
fn remainder() {
    let data = [1, 2, 3, 2, 4, 5];
    let mut split = Splitter::new(&data, &[2]); 
    assert_eq!(split.next(), Some(&data[..1]));
    assert_eq!(split.remainder(), &data[2..]);
    assert_eq!(split.next(), Some(&data[2..3]));
    assert_eq!(split.remainder(), &data[4..]);
    assert_eq!(split.next(), Some(&data[4..]));
    assert_eq!(split.remainder(), &data[6..]);
    assert_eq!(split.next(), None);
    assert_eq!(split.remainder(), &data[6..]);
}
