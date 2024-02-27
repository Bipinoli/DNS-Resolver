// RFC: https://datatracker.ietf.org/doc/html/rfc1035
pub mod buffer;
pub mod header;
pub mod question;
use header::Header;
use question::Question;

#[derive(Debug)]
pub struct Packet {
    header: Header,
    question: Question,
}
