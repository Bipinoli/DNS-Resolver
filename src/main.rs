mod packet;
mod resolver;

use packet::buffer::Buffer;
use packet::Packet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use crate::resolver::{proxy_resolve, recursive_resolve};

fn main() {
    // let mut file = File::open("dns_res_packet").expect("dns_req_packet couldn't be opened");
    // let mut buffer = Buffer::new();
    // file.read(&mut buffer.buf)
    //     .expect("couldn't read file into the buffer");
    //
    // let mut packet = Packet::from_buffer(&mut buffer);
    // dbg!(&packet);

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("please provide the domain name as an argument");
        exit(1);
    }
    let domain_name = args[1].clone();
    let packet = proxy_resolve(domain_name);
    dbg!(&packet);
}
