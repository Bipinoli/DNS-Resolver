use std::net::UdpSocket;

use crate::packet::{
    buffer::Buffer,
    header::{Header, ResponseCode},
    question::Question,
    Packet,
};

pub fn proxy_resolve(domain_name: String) -> Packet {
    let socket = UdpSocket::bind("0.0.0.0:4321").expect("couldn't bind udp socket to the address");
    let response = request_server(domain_name, "1.1.1.1".to_owned(), &socket);
    if let ResponseCode::NoErr = response.header.response_code {
        return response;
    }
    eprintln!("-- 1.1.1.1 couldn't provide us the right information");
    response
}

pub fn recursive_resolve(domain_name: String) -> Packet {
    let socket = UdpSocket::bind("0.0.0.0:4321").expect("couldn't bind udp socket to the address");
    // root servers: https://root-servers.org/
    let root_server_resp = request_server(domain_name.clone(), "193.0.14.129".to_owned(), &socket);
    if let ResponseCode::NoErr = root_server_resp.header.response_code {
        let name_server = root_server_resp.nameserver_records[0].rdata[0].clone();
        let ns_server_resp = request_server(domain_name.clone(), name_server, &socket);
        return ns_server_resp;
    }
    eprintln!("-- root server coulndn't provide the right information");
    root_server_resp
}

fn request_server(domain_name: String, server: String, socket: &UdpSocket) -> Packet {
    println!(
        "-- requesting server {} to resolve domain name: {}",
        server, domain_name
    );
    let request_packet = create_request(domain_name);
    let buffer = request_packet.to_buffer();
    socket
        .send_to(&buffer.buf, (server, 53))
        .expect("couldn't send the udp packet to the server");
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
        nameserver_records: vec![],
        additional_records: vec![],
    }
}
