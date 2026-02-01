
use std::io::Read;

use crate::{Arena, ArenaByteBuilder};

pub struct BufReader<'arena, R: Read> {
    buffer: &'arena mut [u8],
    to_read: (usize, usize),
    eof: bool,
    reader: R
}

impl<'arena, R: Read> BufReader<'arena, R> {

    pub fn new(arena: &'arena Arena, reader: R) -> Self {
        Self::new_with_capacity(arena, reader, 2048)
    }
    
    pub fn new_with_capacity(arena: &'arena Arena, reader: R, capacity: usize) -> Self {
        Self {
            buffer: arena.alloc_arr_default(capacity),
            to_read: (0, 0),
            eof: false,
            reader,
        }
    }

    pub fn curr(&mut self) -> std::io::Result<Option<u8>> {
        if self.eof {
            return Ok(None);
        }

        if self.to_read.0 == self.to_read.1 {
            let read = self.reader.read(self.buffer)?;
            if read == 0 {
                self.eof = true;
                return Ok(None);
            }
            self.to_read = (0, read);
        }
        Ok(Some(self.buffer[self.to_read.0]))
    }

    pub fn next(&mut self) -> std::io::Result<Option<u8>> {
        let Some(byte) = self.curr()? else {
            return Ok(None);
        };

        if self.to_read.0 < self.to_read.1 {
            self.to_read.0 += 1;
        }

        Ok(Some(byte))
    }

    pub fn skip_to_pred<F: Fn(u8) -> bool>(&mut self, pred: F) -> std::io::Result<()> {
        while !self.curr()?.map(&pred).unwrap_or(true) {
            self.next()?;
        }
        Ok(())
    }

    pub fn skip_to(&mut self, byte: u8) -> std::io::Result<()> {
        self.skip_to_pred(|other| other == byte)
    }

    pub fn skip_after(&mut self, byte: u8) -> std::io::Result<()> {
        self.skip_to(byte)?;
        self.next()?;
        Ok(())
    }

    pub fn build_until_pred<F: Fn(u8) -> bool>(&mut self, builder: &mut ArenaByteBuilder, pred: F) -> std::io::Result<()> {
        while !self.curr()?.map(&pred).unwrap_or(true) {
            builder.write(&[self.next()?.unwrap()]);
        }
        Ok(())
    }

    pub fn read_until_pred<'a, F: Fn(u8) -> bool>(&mut self, arena: &'a Arena, pred: F) -> std::io::Result<&'a [u8]> {
        let mut builder = ArenaByteBuilder::new(arena);
        self.build_until_pred(&mut builder, pred)?;
        Ok(builder.finish())
    }

    pub fn read_until<'a>(&mut self, arena: &'a Arena, byte: u8) -> std::io::Result<&'a [u8]> {
        Ok(self.read_until_pred(arena, |other| other == byte)?)
    }

    pub fn read_token<'a>(&mut self, arena: &'a Arena) -> std::io::Result<&'a [u8]> {
        self.skip_to_pred(|c| !c.is_ascii_whitespace())?;
        self.read_until_pred(arena, |c| c.is_ascii_whitespace())
    }

    pub fn read_n<'a>(&mut self, arena: &'a Arena, n: usize) -> std::io::Result<&'a [u8]> {
        let mut builder = ArenaByteBuilder::new(arena);
        for _ in 0..n {
            let Some(byte) = self.next()? else {
                break;
            };
            builder.write(&[byte]);
        }
        Ok(builder.finish())
    }

}

impl<'arena, R: Read> Read for BufReader<'arena, R> {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut read = 0;
        for byte in buf {
            let Some(next) = self.next()? else {
                break;
            };
            *byte = next;
            read += 1;
        }
        Ok(read) 
    }

}

#[cfg(test)]
mod tests;
