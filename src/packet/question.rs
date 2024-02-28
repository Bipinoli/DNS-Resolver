use crate::packet::buffer::Buffer;

#[derive(Debug)]
pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}
impl Question {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        Question {
            qname: Question::read_qname(buffer),
            qtype: buffer.read_u16(),
            qclass: buffer.read_u16(),
        }
    }

    pub fn to_buffer(&self, buffer: &mut Buffer) {
        self.write_qname(buffer);
        buffer.write_u16(self.qtype);
        buffer.write_u16(self.qclass);
    }

    fn read_qname(buffer: &mut Buffer) -> String {
        let mut qname = String::from("");
        let mut is_first = true;
        loop {
            let len = buffer.read_u8();
            if len == 0 {
                break;
            }
            if is_first {
                is_first = false;
            } else {
                qname.push('.');
            }
            for _ in 0..len {
                let c: char = buffer.read_u8() as char;
                qname.push(c);
            }
        }
        qname
    }

    fn write_qname(&self, buffer: &mut Buffer) {
        let splits: Vec<&str> = self.qname.split(".").collect();
        for word in splits {
            let len = word.len() as u8;
            buffer.write_u8(len);
            for c in word.chars() {
                buffer.write_u8(c as u8);
            }
        }
        buffer.write_u8(0);
    }
}
