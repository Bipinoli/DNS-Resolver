use crate::packet::buffer::Buffer;

#[derive(Debug)]
pub struct Question {
    qname: String,
    qtype: u16,
    qclass: u16,
}
impl Question {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        Question {
            qname: Question::read_qname(buffer),
            qtype: buffer.read_u16(),
            qclass: buffer.read_u16(),
        }
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
}
