use crate::Buffer;

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub rr_type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rd_length: u16,
    pub rdata: String,
}

impl ResourceRecord {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        let name = Self::read_label(buffer);
        let rr_type = buffer.read_u16();
        let class = buffer.read_u16();
        let ttl = buffer.read_u32();
        let rd_length = buffer.read_u16();
        let rdata = Self::read_rdata(rd_length, buffer);
        ResourceRecord {
            name,
            rr_type,
            class,
            ttl,
            rd_length,
            rdata,
        }
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

    fn read_rdata(rd_length: u16, buffer: &mut Buffer) -> String {
        let mut nums: Vec<String> = Vec::new();
        for _ in 0..rd_length {
            nums.push(format!("{}", buffer.read_u8()));
        }
        nums.join(".")
    }
}
