// RFC: https://datatracker.ietf.org/doc/html/rfc1035
pub mod buffer;
pub mod header;
pub mod question;
pub mod resource_record;
use buffer::Buffer;
use header::Header;
use question::Question;
use resource_record::ResourceRecord;

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>,
}

impl Packet {
    pub fn from_buffer(buffer: &mut Buffer) -> Self {
        let header = Header::from_buffer(buffer);
        dbg!(&header);
        let mut questions: Vec<Question> = vec![];
        for _ in 0..header.qd_count {
            questions.push(Question::from_buffer(buffer));
        }
        dbg!(&questions);
        let mut answers: Vec<ResourceRecord> = vec![];
        for _ in 0..header.an_count {
            answers.push(ResourceRecord::from_buffer(buffer));
        }
        dbg!(&answers);
        let mut authority_records: Vec<ResourceRecord> = vec![];
        for _ in 0..header.ar_count {
            authority_records.push(ResourceRecord::from_buffer(buffer));
        }
        dbg!(&authority_records);
        let mut additional_records: Vec<ResourceRecord> = vec![];
        for _ in 0..header.ar_count {
            additional_records.push(ResourceRecord::from_buffer(buffer));
        }
        dbg!(&additional_records);
        Packet {
            header,
            questions,
            answers,
            authority_records,
            additional_records,
        }
    }

    pub fn to_buffer(&self) -> Buffer {
        let mut buffer = Buffer::new();
        self.header.to_buffer(&mut buffer);
        for item in &self.questions {
            item.to_buffer(&mut buffer);
        }
        for item in &self.answers {
            item.to_buffer(&mut buffer);
        }
        for item in &self.authority_records {
            item.to_buffer(&mut buffer);
        }
        for item in &self.additional_records {
            item.to_buffer(&mut buffer);
        }
        buffer
    }
}
