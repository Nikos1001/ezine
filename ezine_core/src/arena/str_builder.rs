
use crate::Arena;

pub struct ArenaByteBuilder<'arena> {
    arena: &'arena Arena,
    start: *mut u8,
    prev_top: *mut u8
}

impl<'arena> ArenaByteBuilder<'arena> {

    pub fn new(arena: &'arena Arena) -> Self {
        let start = arena.curr.get();
        Self {
            arena,
            start,
            prev_top: start
        }
    }

    pub fn alloc(&mut self, len: usize) -> &mut [u8] {
        if self.arena.curr.get() != self.prev_top {
            panic!("cannot allocate something on arena between two writes to builder. builder interrupted.");
        }
        let alloc = self.arena.alloc_arr_default(len);
        self.prev_top = self.arena.curr.get();
        alloc
    }

    pub fn write(&mut self, data: &[u8]) {
        if self.arena.curr.get() != self.prev_top {
            panic!("cannot allocate something on arena between two writes to builder. builder interrupted.");
        }
        self.arena.alloc_arr_copy(data);
        self.prev_top = self.arena.curr.get();
    }

    pub fn finish(self) -> &'arena [u8] {
        unsafe {
            let start = self.start;
            let end = self.arena.curr.get();
            let len = end.offset_from(start) as usize;
            std::slice::from_raw_parts(start, len)
        }
    }

}

impl<'arena> std::io::Write for ArenaByteBuilder<'arena> {

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

}

impl<'arena> std::fmt::Write for ArenaByteBuilder<'arena> {

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }

}

pub struct ArenaStringBuilder<'arena> {
    byte: ArenaByteBuilder<'arena>
}

impl<'arena> ArenaStringBuilder<'arena> {

    pub fn new(arena: &'arena Arena) -> Self {
        Self {
            byte: ArenaByteBuilder::new(arena)
        }
    }

    pub fn write(&mut self, data: &str) {
        self.byte.write(data.as_bytes());
    }

    pub fn finish(self) -> &'arena str {
        unsafe {
            std::str::from_utf8_unchecked(self.byte.finish())
        }
    }

}

impl<'arena> std::fmt::Write for ArenaStringBuilder<'arena> {

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write(s);
        Ok(())
    }

}
