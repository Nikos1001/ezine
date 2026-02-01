
use super::Arena;
use std::{fmt::Debug, mem, ops::{Index, IndexMut}};

pub struct ArenaVec<'arena, T> {
    buf: &'arena mut [mem::MaybeUninit<T>],
    len: usize,
}

impl<'arena, T> ArenaVec<'arena, T> {

    pub fn new() -> Self {
        Self {
            buf: &mut [],
            len: 0,
        }
    }

    pub fn push(&mut self, arena: &'arena Arena, val: T) {
        if self.len == self.buf.len() {
            let new_cap = if self.buf.len() == 0 { 8 } else { 2 * self.buf.len() };
            let new_buf = arena.alloc_arr(new_cap, |i| if i < self.len {
                unsafe { std::mem::transmute_copy(&self.buf[i]) }
            } else {
                mem::MaybeUninit::uninit()
            });
            self.buf = new_buf;
        }
        self.buf[self.len] = mem::MaybeUninit::new(val);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    unsafe fn as_ptr(&self) -> *const T {
        self.buf.as_ptr() as *const T
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr(), self.len)
        }
    }

}

impl<T> Index<usize> for ArenaVec<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { self.buf[index].assume_init_ref() }
    }
}

impl<T> IndexMut<usize> for ArenaVec<'_, T> {

    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { self.buf[index].assume_init_mut() }
    }

}

impl<T: Debug> Debug for ArenaVec<'_, T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }

}

impl<T: PartialEq> PartialEq for ArenaVec<'_, T> {

    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }

        for i in 0..self.len {
            unsafe {
                if self.buf[i].assume_init_ref() != other.buf[i].assume_init_ref() {
                    return false;
                }
            }
        }

        true
    }
    
}

impl<T: Eq> Eq for ArenaVec<'_, T> {}

impl<'arena, 'vec: 'arena, T> IntoIterator for &'vec ArenaVec<'arena, T> {
    type Item = &'arena T;

    type IntoIter = std::slice::Iter<'vec, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
