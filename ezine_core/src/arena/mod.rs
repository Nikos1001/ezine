
use std::{alloc, cell::Cell};

mod vec;
pub use vec::*;

mod str_builder;
pub use str_builder::*;

pub struct Arena {
    size: usize,
    head: *mut u8,
    tail: *mut u8,
    curr: Cell<*mut u8>,
}

impl Arena {

    fn layout(size: usize) -> alloc::Layout {
        alloc::Layout::from_size_align(size, 8).unwrap()
    }

    pub fn new() -> Self {
        Self::new_with_size(16 * 1024 * 1024)
    }

    pub fn new_with_size(size: usize) -> Self {
        unsafe {
            let head = alloc::alloc(Self::layout(size));
            let tail = head.add(size);
            let curr = head;
            Self {
                size,
                head,
                tail,
                curr: Cell::new(curr),
            }
        }
    }

    unsafe fn alloc_mem(&self, layout: alloc::Layout) -> *mut u8 {
        let align_padding = self.curr.get().align_offset(layout.align());
        let new_curr = self.curr.get().wrapping_add(align_padding + layout.size());
        if new_curr > self.tail {
            panic!("arena allocator out of memory.");
        }
        let alloc = self.curr.get().wrapping_add(align_padding);
        self.curr.set(new_curr);
        alloc
    }

    pub fn alloc<T>(&self, val: T) -> &mut T {
        unsafe {
            let alloc = self.alloc_mem(alloc::Layout::new::<T>());
            let alloc = alloc as *mut T;
            *alloc = val;
            &mut *alloc
        }
    }

    pub fn alloc_arr<T, F: FnMut(usize) -> T>(&self, len: usize, mut f: F) -> &mut [T] {
        unsafe  {
            let alloc = self.alloc_mem(alloc::Layout::array::<T>(len).unwrap());
            let alloc = std::slice::from_raw_parts_mut(alloc as *mut T, len);
            for i in 0..len {
                alloc[i] = f(i);
            }
            alloc
        }
    }

    pub fn alloc_arr_copy<'arena, T: Copy>(&'arena self, arr: &[T]) -> &'arena mut [T] {
        self.alloc_arr(arr.len(), |idx| arr[idx])
    }

    pub fn alloc_arr_default<T: Default>(&self, len: usize) -> &mut [T] {
        self.alloc_arr(len, |_| T::default())
    }

    pub fn alloc_str(&self, s: &str) -> &mut str {
        let str = self.alloc_arr_copy(s.as_bytes());
        unsafe { str::from_utf8_unchecked_mut(str) }
    }

    pub fn clear(&mut self) {
        self.curr.set(self.head);
    }
    
}

impl Drop for Arena {

    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.head, Self::layout(self.size))
        };
    }

}

#[cfg(test)]
mod tests;
