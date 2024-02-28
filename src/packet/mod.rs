// RFC: https://datatracker.ietf.org/doc/html/rfc1035
pub mod buffer;
pub mod header;
pub mod question;
use buffer::Buffer;
use header::Header;
use question::Question;

#[derive(Debug)]
pub struct Packet {
    header: Header,
    questions: Vec<Question>,
}

impl Packet {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        let header = Header::from_buffer(buffer);
        let mut questions: Vec<Question> = vec![];
        for _ in 0..header.qd_count {
            questions.push(Question::from_buffer(buffer));
        }
        Packet { header, questions }
    }
}
