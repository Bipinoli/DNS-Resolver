use std::net::UdpSocket;

use crate::packet::{
    buffer::Buffer,
    header::Header,
    question::{self, Question},
    Packet,
};

pub fn resolve(domain_name: String) -> Packet {
    let socket = UdpSocket::bind("0.0.0.0:4321").expect("couldn't bind udp socket to the address");
    let request_packet = create_request(domain_name);
    let buffer = request_packet.to_buffer();
    socket
        .send_to(&buffer.buf, ("1.1.1.1", 53))
        .expect("couldn't send the udp packet to the 1.1.1.1");
    let mut resp_buffer = Buffer::new();
    socket
        .recv_from(&mut resp_buffer.buf)
        .expect("couldn't read the udp packets from the socket");
    Packet::from_buffer(&mut resp_buffer)
}

fn create_request(domain_name: String) -> Packet {
    let header = Header::for_query();
    let question = Question {
        qname: domain_name,
        qtype: 1,  // A (host address)
        qclass: 1, // IN (internet)
    };
    Packet {
        header,
        questions: vec![question],
        answers: vec![],
        authority_records: vec![],
        additional_records: vec![],
    }
}
