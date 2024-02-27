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
        if self.cursor >= BUF_SIZE {
            panic!("attempted to read over the buffer");
        }
        let retval = self.buf[self.cursor];
        self.cursor += 1;
        retval
    }
    pub fn read_u16(&mut self) -> u16 {
        if self.cursor >= BUF_SIZE {
            panic!("attempted to read over the buffer");
        }
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }
}
