
use crate::Arena;

pub struct ArenaStringBuilder<'arena> {
    arena: &'arena Arena,
    start: *mut u8,
    prev_top: *mut u8
}

impl<'arena> ArenaStringBuilder<'arena> {

    pub fn new(arena: &'arena Arena) -> Self {
        let start = arena.curr.get();
        Self {
            arena,
            start,
            prev_top: start
        }
    }

    pub fn write(&mut self, s: &str) {
        if self.arena.curr.get() != self.prev_top {
            panic!("cannot allocate something on arena between two writes to string builder. string builder interrupted.");
        }
        self.arena.alloc_str(s);
        self.prev_top = self.arena.curr.get();
    }

    pub fn finish(self) -> &'arena str {
        unsafe {
            let start = self.start;
            let end = self.arena.curr.get();
            let len = end.offset_from(start) as usize;
            let slice = std::slice::from_raw_parts(start, len);
            std::str::from_utf8_unchecked(slice)
        }
    }

}

impl<'arena> std::fmt::Write for ArenaStringBuilder<'arena> {

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write(s);
        Ok(())
    }

}
