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
    let mut response = request_server(domain_name.clone(), "193.0.14.129".to_owned(), &socket);
    loop {
        match response.header.response_code {
            ResponseCode::NoErr => {
                if response.header.is_authorotative_ans {
                    return response;
                }
                let ns_server = response.nameserver_records[0].rdata.clone();
                response = request_server(domain_name.clone(), ns_server, &socket)
            }
            _ => {
                eprintln!("-- error response from the server");
                return response;
            }
        }
    }
}

fn request_server(domain_name: String, server: String, socket: &UdpSocket) -> Packet {
    println!(
        "-- requesting server {} to resolve domain name: {}",
        server, domain_name
    );
    let request_packet = create_request(domain_name);
    let buffer = request_packet.to_buffer();
    socket
        .send_to(buffer.trim(), (server, 53))
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
