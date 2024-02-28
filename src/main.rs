mod packet;
mod resolver;

use packet::buffer::Buffer;
use packet::Packet;
use resolver::resolve;
use std::fs::File;
use std::io::Read;

fn main() {
    // let mut file = File::open("dns_res_packet").expect("dns_req_packet couldn't be opened");
    // let mut buffer = Buffer::new();
    // file.read(&mut buffer.buf)
    //     .expect("couldn't read file into the buffer");
    //
    // let mut packet = Packet::from_buffer(&mut buffer);
    // dbg!(&packet);

    let packet = resolve("github.com".to_owned());
    dbg!(&packet);
}
