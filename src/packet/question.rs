#[derive(Debug)]
pub struct Question {
    qtype: u16,
    qclass: u16,
    qname: u16,
}
