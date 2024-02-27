use crate::packet::buffer::Buffer;

#[derive(Debug)]
pub enum ResponseCode {
    NoErr = 0,
    FormatErr = 1,
    ServerFail = 2,
    NameErr = 3,
    NotImplemented = 4,
    Refused = 5,
    Unknown,
}

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}
impl Header {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        Header {
            id: buffer.read_u16(),
            flags: buffer.read_u16(),
            qd_count: buffer.read_u16(),
            an_count: buffer.read_u16(),
            ns_count: buffer.read_u16(),
            ar_count: buffer.read_u16(),
        }
    }

    pub fn is_response(&self) -> bool {
        (self.flags & (1 << 15)) != 0
    }
    pub fn is_query(&self) -> bool {
        !self.is_response()
    }
    pub fn is_standard_query(&self) -> bool {
        let opcode = (self.flags & (0b1111 << 11)) >> 11;
        opcode == 0
    }
    pub fn is_authorotative_ans(&self) -> bool {
        (self.flags & (1 << 10)) != 0
    }
    pub fn is_recursion_available(&self) -> bool {
        (self.flags & (1 << 7)) != 0
    }
    pub fn get_response_code(&self) -> ResponseCode {
        let rcode = 0b1111 & self.flags;
        match rcode {
            0 => ResponseCode::NoErr,
            1 => ResponseCode::FormatErr,
            2 => ResponseCode::ServerFail,
            3 => ResponseCode::NameErr,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => ResponseCode::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Question {
    qtype: u16,
    qclass: u16,
    qname: u16,
}

#[derive(Debug)]
pub struct Packet {
    header: Header,
    question: Question,
}
