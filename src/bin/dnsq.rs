use rgbdns::{Message, Question, RecordType};
use std::{
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};
fn main() {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 3 {
        eprintln!("usage: dnsq type name server");
        std::process::exit(100)
    }
    let q = Message {
        id: random_id(),
        flags: 0x100,
        questions: vec![Question {
            name: a[1].parse().unwrap(),
            qtype: a[0].parse::<RecordType>().unwrap(),
            qclass: 1,
        }],
        ..Default::default()
    };
    let s = UdpSocket::bind("0.0.0.0:0").unwrap();
    s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    s.send_to(&q.encode().unwrap(), server_address(&a[2]))
        .unwrap();
    let mut b = [0; 65535];
    let n = s.recv(&mut b).unwrap();
    println!("{:#?}", Message::decode(&b[..n]).unwrap())
}

fn server_address(server: &str) -> SocketAddr {
    server
        .parse()
        .ok()
        .or_else(|| (server, 53).to_socket_addrs().ok()?.next())
        .unwrap_or_else(|| {
            eprintln!("dnsq: fatal: invalid server address: {server}");
            std::process::exit(100)
        })
}
fn random_id() -> u16 {
    let mut b = [0; 2];
    getrandom::fill(&mut b).expect("OS randomness");
    u16::from_ne_bytes(b)
}
