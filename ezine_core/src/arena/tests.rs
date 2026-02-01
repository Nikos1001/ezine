
use super::*; 

#[test]
fn hello_world() {
    let arena = Arena::new_with_size(1024);
    let my_number = arena.alloc(123);
    assert_eq!(*my_number, 123);
}

#[test]
#[should_panic]
fn arena_overflow() {
    let arena = Arena::new_with_size(3);
    arena.alloc(123);
}

#[test]
fn arena_exact_size() {
    let arena = Arena::new_with_size(4);
    arena.alloc(123);
}

#[test]
fn arena_align() {
    let arena = Arena::new_with_size(1024);
    arena.alloc(123u8);
    let i32_ptr = arena.alloc(123) as *const i32;
    assert!(i32_ptr.is_aligned());
}

#[test]
fn arena_clear() {
    let mut arena = Arena::new_with_size(4);
    let n1 = arena.alloc(123);
    assert_eq!(*n1, 123);
    arena.clear();
    let n2 = arena.alloc(456);
    assert_eq!(*n2, 456);
}

#[test]
fn arena_array() {
    let arena = Arena::new_with_size(1024);
    let arr = arena.alloc_arr(5, |x| 2 * x);
    for i in 0..5 {
        assert_eq!(arr[i], 2 * i);
    }
}

#[test]
fn arena_vec() {
    let arena = Arena::new_with_size(1024);
    let mut vec = ArenaVec::new();
    for i in 0..50 {
        vec.push(&arena, 3 * i + 5);
    }
    for i in 0..50 {
        assert_eq!(vec[i], 3 * i + 5);
    }
    assert_eq!(vec.len(), 50);
    assert_eq!(vec.capacity(), 64);
}

#[test]
fn string_builder() {
    let arena = Arena::new();
    let mut builder = ArenaStringBuilder::new(&arena);
    builder.write("Hello, ");
    builder.write("World!");
    let result = builder.finish();
    assert_eq!(result, "Hello, World!");
}

#[test]
#[should_panic]
fn interrupted_string_builder() {
    let arena = Arena::new();
    let mut builder = ArenaStringBuilder::new(&arena);
    builder.write("Oh");
    arena.alloc(123);
    builder.write("No");
}
