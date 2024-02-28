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
    pub nameserver_records: Vec<ResourceRecord>,
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
        let mut nameserver_records: Vec<ResourceRecord> = vec![];
        for _ in 0..header.ns_count {
            nameserver_records.push(ResourceRecord::from_buffer(buffer));
        }
        dbg!(&nameserver_records);
        //TODO: implement parsing additional_records
        let mut additional_records: Vec<ResourceRecord> = vec![];
        Packet {
            header,
            questions,
            answers,
            nameserver_records,
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
        for item in &self.nameserver_records {
            item.to_buffer(&mut buffer);
        }
        for item in &self.additional_records {
            item.to_buffer(&mut buffer);
        }
        buffer
    }
}
