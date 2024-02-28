use core::panic;
use std::usize;

const BUF_SIZE: usize = 512;

#[derive(Debug)]
pub struct Buffer {
    pub buf: [u8; BUF_SIZE],
    pub cursor: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buf: [0; BUF_SIZE],
            cursor: 0,
        }
    }
    pub fn read_u8(&mut self) -> u8 {
        if self.cursor + 8 >= BUF_SIZE {
            panic!("attempted to read outside the buffer");
        }
        let retval = self.buf[self.cursor];
        self.cursor += 1;
        retval
    }
    pub fn read_u16(&mut self) -> u16 {
        if self.cursor + 16 >= BUF_SIZE {
            panic!("attempted to read outside the buffer");
        }
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }
    pub fn read_u32(&mut self) -> u32 {
        let first_chunk: u32 = self.read_u16() as u32;
        let second_chunk: u32 = self.read_u16() as u32;
        (first_chunk << 16) | second_chunk
    }
    pub fn move_cursor(&mut self, offset: u16) {
        if (offset as usize) >= BUF_SIZE {
            panic!("attempted to move cursor outside the buffer");
        }
        self.cursor = offset as usize;
    }
}
