//! Small bounded stub-resolver shared by the djbdns-compatible client tools.

use crate::{Error, Message, Name, Question, RecordType, Result};
use std::{
    fs,
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(5);

pub fn recursive(name: Name, record_type: RecordType) -> Result<Message> {
    query(name, record_type, true, &servers()?)
}

pub fn query(
    name: Name,
    record_type: RecordType,
    recursion_desired: bool,
    servers: &[SocketAddr],
) -> Result<Message> {
    let id = random_id()?;
    let wire = Message {
        id,
        flags: if recursion_desired { 0x0100 } else { 0 },
        questions: vec![Question {
            name,
            qtype: record_type,
            qclass: 1,
        }],
        ..Default::default()
    }
    .encode()?;
    let mut last_error = None;
    for server in servers
        .iter()
        .copied()
        .cycle()
        .take(servers.len().max(1) * 2)
    {
        match udp_query(server, &wire, id) {
            Ok(response) if response.flags & 0x0200 != 0 => match tcp_query(server, &wire, id) {
                Ok(response) => return Ok(response),
                Err(error) => last_error = Some(error),
            },
            Ok(response) => return Ok(response),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap_or(Error::Format("no recursive DNS servers configured")))
}

pub fn servers() -> Result<Vec<SocketAddr>> {
    if let Ok(value) = std::env::var("DNSCACHEIP") {
        return value
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(server_address)
            .collect();
    }
    let contents = fs::read_to_string("/etc/resolv.conf")?;
    let servers = contents
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            (fields.next() == Some("nameserver"))
                .then(|| fields.next())
                .flatten()
        })
        .map(server_address)
        .collect::<Result<Vec<_>>>()?;
    if servers.is_empty() {
        Err(Error::Format("no nameserver in resolv.conf"))
    } else {
        Ok(servers)
    }
}

pub fn server_address(value: &str) -> Result<SocketAddr> {
    if let Ok(address) = value.parse() {
        return Ok(address);
    }
    value
        .parse::<IpAddr>()
        .map(|address| SocketAddr::new(address, 53))
        .map_err(|_| Error::Format("invalid DNS server address"))
}

fn udp_query(server: SocketAddr, wire: &[u8], id: u16) -> Result<Message> {
    let bind = if server.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    };
    let socket = UdpSocket::bind(bind)?;
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;
    socket.connect(server)?;
    socket.send(wire)?;
    let mut response = [0; 65535];
    let length = socket.recv(&mut response)?;
    validate(Message::decode(&response[..length])?, id)
}

fn tcp_query(server: SocketAddr, wire: &[u8], id: u16) -> Result<Message> {
    let mut stream = TcpStream::connect_timeout(&server, TIMEOUT)?;
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(wire)?;
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut response = vec![0; u16::from_be_bytes(length) as usize];
    stream.read_exact(&mut response)?;
    validate(Message::decode(&response)?, id)
}

fn validate(message: Message, id: u16) -> Result<Message> {
    if message.id != id || message.flags & 0x8000 == 0 {
        Err(Error::Format("mismatched DNS response"))
    } else {
        Ok(message)
    }
}

fn random_id() -> Result<u16> {
    let mut bytes = [0; 2];
    getrandom::fill(&mut bytes)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    Ok(u16::from_ne_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RData, Record};
    use std::{net::Ipv4Addr, thread};

    #[test]
    fn truncated_udp_response_falls_back_to_tcp() {
        let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let udp_thread = thread::spawn(move || {
            let mut wire = [0; 512];
            let (length, peer) = udp.recv_from(&mut wire).unwrap();
            let request = Message::decode(&wire[..length]).unwrap();
            let response = Message {
                id: request.id,
                flags: 0x8000 | 0x0200,
                questions: request.questions,
                ..Default::default()
            }
            .encode()
            .unwrap();
            udp.send_to(&response, peer).unwrap();
        });
        let tcp_thread = thread::spawn(move || {
            let (mut stream, _) = tcp.accept().unwrap();
            let mut length = [0; 2];
            stream.read_exact(&mut length).unwrap();
            let mut wire = vec![0; u16::from_be_bytes(length) as usize];
            stream.read_exact(&mut wire).unwrap();
            let request = Message::decode(&wire).unwrap();
            let name = request.questions[0].name.clone();
            let response = Message {
                id: request.id,
                flags: 0x8000,
                questions: request.questions,
                answers: vec![Record {
                    name,
                    ttl: 60,
                    data: RData::A(Ipv4Addr::new(192, 0, 2, 1)),
                }],
                ..Default::default()
            }
            .encode()
            .unwrap();
            stream
                .write_all(&(response.len() as u16).to_be_bytes())
                .unwrap();
            stream.write_all(&response).unwrap();
        });
        let response = query("example".parse().unwrap(), RecordType::A, true, &[address]).unwrap();
        udp_thread.join().unwrap();
        tcp_thread.join().unwrap();
        assert_eq!(
            response.answers[0].data,
            RData::A(Ipv4Addr::new(192, 0, 2, 1))
        );
    }

    #[test]
    fn parses_bare_and_explicit_port_server_addresses() {
        assert_eq!(
            server_address("192.0.2.1").unwrap(),
            "192.0.2.1:53".parse().unwrap()
        );
        assert_eq!(
            server_address("127.0.0.1:5353").unwrap(),
            "127.0.0.1:5353".parse().unwrap()
        );
    }
}
