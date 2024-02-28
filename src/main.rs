mod packet;
use packet::buffer::Buffer;
use packet::header::Header;
use packet::Packet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("dns_req_packet").expect("dns_req_packet couldn't be opened");
    let mut buffer = Buffer::new();
    file.read(&mut buffer.buf)
        .expect("couldn't read file into the buffer");

    let mut packet = Packet::from_buffer(&mut buffer);
    dbg!(&packet);
}
