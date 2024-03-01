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
    // -- convinience fields ..
    pub is_response: bool,
    pub response_code: ResponseCode,
    pub is_query: bool,
    pub is_standard_query: bool,
    pub is_authorotative_ans: bool,
    pub is_recursion_available: bool,
}
impl Header {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        let id = buffer.read_u16();
        let flags = buffer.read_u16();
        Header {
            id,
            flags,
            qd_count: buffer.read_u16(),
            an_count: buffer.read_u16(),
            ns_count: buffer.read_u16(),
            ar_count: buffer.read_u16(),
            is_response: Header::is_response(flags),
            response_code: Header::get_response_code(flags),
            is_query: Header::is_query(flags),
            is_standard_query: Header::is_standard_query(flags),
            is_authorotative_ans: Header::is_authorotative_ans(flags),
            is_recursion_available: Header::is_recursion_available(flags),
        }
    }

    pub fn to_buffer(&self, buffer: &mut Buffer) {
        buffer.write_u16(self.id);
        buffer.write_u16(self.flags);
        buffer.write_u16(self.qd_count);
        buffer.write_u16(self.an_count);
        buffer.write_u16(self.ns_count);
        buffer.write_u16(self.ar_count);
    }

    pub fn for_query() -> Self {
        let id = 1234;
        let mut flags: u16 = 0;
        // recursion desired
        // flags = flags | (1 << 8);
        let is_response: bool = Header::is_response(flags);
        let response_code: ResponseCode = Header::get_response_code(flags);
        let is_query: bool = Header::is_query(flags);
        let is_standard_query: bool = Header::is_standard_query(flags);
        let is_authorotative_ans: bool = Header::is_authorotative_ans(flags);
        let is_recursion_available: bool = Header::is_recursion_available(flags);

        Header {
            id,
            flags,
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
            is_response,
            response_code,
            is_query,
            is_standard_query,
            is_authorotative_ans,
            is_recursion_available,
        }
    }
    fn is_response(flags: u16) -> bool {
        (flags & (1 << 15)) != 0
    }
    fn is_query(flags: u16) -> bool {
        !Header::is_response(flags)
    }
    fn is_standard_query(flags: u16) -> bool {
        let opcode = (flags & (0b1111 << 11)) >> 11;
        opcode == 0
    }
    fn is_authorotative_ans(flags: u16) -> bool {
        (flags & (1 << 10)) != 0
    }
    fn is_recursion_available(flags: u16) -> bool {
        (flags & (1 << 7)) != 0
    }
    fn get_response_code(flags: u16) -> ResponseCode {
        let rcode = 0b1111 & flags;
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
