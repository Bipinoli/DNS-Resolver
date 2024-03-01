use crate::Buffer;

#[repr(u16)]
#[derive(Debug)]
pub enum RrType {
    A = 1,
    NS = 2,
    Unknown(u16),
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub rr_type: RrType,
    pub class: u16,
    pub ttl: u32,
    pub rd_length: u16,
    pub rdata: String,
}

impl ResourceRecord {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        let before = buffer.cursor;
        let name = Self::read_label(buffer);
        // dbg!(&buffer.buf[before..buffer.cursor]);
        let rr_type = Self::read_rrtype(buffer);
        let class = buffer.read_u16();
        let ttl = buffer.read_u32();
        let rd_length = buffer.read_u16();
        let rdata = Self::read_rdata(rd_length, &rr_type, buffer);
        ResourceRecord {
            name,
            rr_type,
            class,
            ttl,
            rd_length,
            rdata,
        }
    }

    pub fn to_buffer(&self, buffer: &mut Buffer) {
        self.write_label(buffer);
        self.write_rrtype(buffer);
        buffer.write_u16(self.class);
        buffer.write_u32(self.ttl);
        buffer.write_u16(self.rd_length);
        self.write_rdata(buffer);
    }

    fn read_label(buffer: &mut Buffer) -> String {
        // compression scheme allows for message to be represented as:
        // - a sequence of labels ending in a zero octet
        // - a pointer
        // - a sequence of labels ending with a pointer
        let mut retval = String::new();
        loop {
            let byte = buffer.read_u8();
            let is_pointer = { (byte & 0b11000000) != 0 };
            if is_pointer {
                let next_byte = buffer.read_u8();
                let offset: u16 = ((byte as u16) << 8) | (next_byte as u16);
                let offset = offset & 0b00111111_11111111;
                let cursor = buffer.cursor;
                buffer.move_cursor(offset);
                let label = Self::read_label(buffer);
                if retval.len() > 0 {
                    retval.push('.');
                }
                retval = format!("{}{}", retval, label);
                buffer.move_cursor(cursor as u16);
                return retval;
            }
            let len = byte;
            if len == 0 {
                return retval;
            }
            if retval.len() > 0 {
                retval.push('.');
            }
            for _ in 0..len {
                let c: char = buffer.read_u8() as char;
                retval.push(c);
            }
        }
    }

    fn write_label(&self, buffer: &mut Buffer) {
        // TODO: use pointer compression method to save space
        let splits: Vec<&str> = self.name.split(".").collect();
        for word in splits {
            let len = word.len() as u8;
            buffer.write_u8(len);
            for c in word.chars() {
                buffer.write_u8(c as u8);
            }
        }
        buffer.write_u8(0);
    }

    fn read_rrtype(buffer: &mut Buffer) -> RrType {
        match buffer.read_u16() {
            1 => RrType::A,
            2 => RrType::NS,
            v => RrType::Unknown(v),
        }
    }

    fn write_rrtype(&self, buffer: &mut Buffer) {
        let v = match self.rr_type {
            RrType::A => 1,
            RrType::NS => 2,
            RrType::Unknown(v) => v,
        };
        buffer.write_u16(v);
    }

    fn read_rdata(rd_length: u16, rr_type: &RrType, buffer: &mut Buffer) -> String {
        match rr_type {
            RrType::A => Self::read_ip(rd_length, buffer),
            RrType::NS => Self::read_label(buffer),
            RrType::Unknown(v) => panic!("rdata of unkonwn type: {}", v),
        }
    }

    fn write_rdata(&self, buffer: &mut Buffer) {
        match self.rr_type {
            RrType::A => self.write_ip(buffer),
            RrType::NS => self.write_label(buffer),
            RrType::Unknown(v) => panic!("rdata of unkonwn type: {}", v),
        }
    }

    fn read_ip(length: u16, buffer: &mut Buffer) -> String {
        let mut nums: Vec<String> = Vec::new();
        for _ in 0..length {
            nums.push(format!("{}", buffer.read_u8()));
        }
        nums.join(".")
    }

    fn write_ip(&self, buffer: &mut Buffer) {
        let splits: Vec<&str> = self.rdata.split(".").collect();
        for word in splits {
            let num: u8 = word.parse::<u8>().unwrap();
            buffer.write_u8(num);
        }
    }
}
