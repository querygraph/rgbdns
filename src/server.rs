use crate::{
    Error, Result,
    packet::Message,
    zone::{Lookup, Zone},
};
use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
    sync::Arc,
    thread,
};

pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    respond_over_transport(zone, wire, transport_limit, true)
}

fn respond_over_transport(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    is_udp: bool,
) -> Result<Vec<u8>> {
    let q = Message::decode(wire)?;
    if q.flags & 0x8000 != 0 || q.questions.len() != 1 {
        return Err(Error::Format("expected one query"));
    }
    let question = q.questions[0].clone();
    let opt = q.additionals.iter().find_map(|record| match &record.data {
        crate::RData::Opt {
            udp_payload,
            version,
            flags,
            ..
        } => Some((*udp_payload, *version, *flags)),
        _ => None,
    });
    let response_limit = if is_udp {
        opt.map_or(512, |(size, _, _)| usize::from(size).max(512))
            .min(transport_limit)
    } else {
        transport_limit
    };
    let mut r = Message {
        id: q.id,
        flags: 0x8000 | 0x0400 | (q.flags & 0x0100),
        questions: vec![question.clone()],
        ..Default::default()
    };
    if let Some((payload, version, flags)) = opt {
        let bad_version = version != 0;
        r.additionals.push(crate::Record {
            name: crate::Name::root(),
            ttl: 0,
            data: crate::RData::Opt {
                udp_payload: payload.min(4096),
                extended_rcode: u8::from(bad_version),
                version: 0,
                flags: flags & 0x8000,
                options: Vec::new(),
            },
        });
        if bad_version {
            return r.encode();
        }
    }
    if question.qclass != 1 {
        r.flags |= 4
    } else {
        match zone.lookup(&question.name, question.qtype) {
            Lookup::Answer(x) => r.answers = x,
            Lookup::Referral {
                authorities,
                additionals,
            } => {
                r.flags &= !0x0400;
                r.authorities = authorities;
                r.additionals.extend(additionals);
            }
            Lookup::NoData(soa) => {
                if let Some(x) = soa {
                    r.authorities.push(x)
                }
            }
            Lookup::NxDomain(soa) => {
                r.flags |= 3;
                if let Some(x) = soa {
                    r.authorities.push(x)
                }
            }
            Lookup::Refused => r.flags |= 5,
        }
    }
    truncate(r, response_limit)
}

fn truncate(mut response: Message, limit: usize) -> Result<Vec<u8>> {
    let full = response.encode()?;
    if full.len() <= limit {
        return Ok(full);
    }
    response.flags |= 0x0200;
    loop {
        let wire = response.encode()?;
        if wire.len() <= limit {
            return Ok(wire);
        }
        if let Some(index) = response
            .additionals
            .iter()
            .rposition(|record| record.rr_type() != crate::RecordType::Opt)
        {
            response.additionals.remove(index);
        } else {
            let removed = response.authorities.pop().is_some()
                || response.answers.pop().is_some()
                || response.additionals.pop().is_some();
            if removed {
                continue;
            }
            // A valid question can itself exceed an unusually small caller
            // limit. Return a header-only truncated response in that case.
            if response.questions.is_empty() {
                return Ok(wire);
            }
            response.questions.clear();
        }
    }
}

pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let udp = UdpSocket::bind(addr)?;
    let tcp = TcpListener::bind(addr)?;
    serve_sockets(zone, udp, tcp)
}

fn serve_sockets(zone: Zone, udp: UdpSocket, tcp: TcpListener) -> Result<()> {
    let zone = Arc::new(zone);
    let z = zone.clone();
    thread::spawn(move || {
        let mut b = [0u8; 65535];
        loop {
            if let Ok((n, peer)) = udp.recv_from(&mut b)
                && let Ok(r) = respond(&z, &b[..n], 4096)
            {
                let _ = udp.send_to(&r, peer);
            }
        }
    });
    for stream in tcp.incoming() {
        let z = zone.clone();
        thread::spawn(move || {
            if let Ok(mut s) = stream {
                let mut l = [0; 2];
                if s.read_exact(&mut l).is_ok() {
                    let mut b = vec![0; u16::from_be_bytes(l) as usize];
                    if s.read_exact(&mut b).is_ok()
                        && let Ok(r) = respond_over_transport(&z, &b, 65535, false)
                    {
                        let _ = s.write_all(&(r.len() as u16).to_be_bytes());
                        let _ = s.write_all(&r);
                    }
                }
            }
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType};
    use std::{
        io::{Read, Write},
        net::{TcpStream, UdpSocket},
        time::Duration,
    };

    fn query(name: &str, typ: RecordType, opt: Option<(u16, u8)>) -> Vec<u8> {
        let mut message = Message {
            id: 0x1234,
            flags: 0x0100,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: typ,
                qclass: 1,
            }],
            ..Default::default()
        };
        if let Some((payload, version)) = opt {
            message.additionals.push(Record {
                name: Name::root(),
                ttl: 0,
                data: RData::Opt {
                    udp_payload: payload,
                    extended_rcode: 0,
                    version,
                    flags: 0x8000,
                    options: Vec::new(),
                },
            });
        }
        message.encode().unwrap()
    }

    #[test]
    fn referral_has_no_aa_and_contains_bailiwick_glue() {
        let zone = Zone::parse(".example::ns.example\n&child.example:192.0.2.2:ns.child.example\n")
            .unwrap();
        let response = Message::decode(
            &respond(
                &zone,
                &query("host.child.example", RecordType::A, None),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(response.flags & 0x0400, 0);
        assert_eq!(response.authorities.len(), 1);
        assert!(
            response
                .additionals
                .iter()
                .any(|record| record.rr_type() == RecordType::A)
        );
    }

    #[test]
    fn edns_negotiation_and_bad_version() {
        let zone = Zone::parse(".example::ns.example\n+www.example:192.0.2.1\n").unwrap();
        let response = Message::decode(
            &respond(
                &zone,
                &query("www.example", RecordType::A, Some((1232, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            response.additionals[0].data,
            RData::Opt {
                udp_payload: 1232,
                version: 0,
                ..
            }
        ));
        let bad_version = Message::decode(
            &respond(
                &zone,
                &query("www.example", RecordType::A, Some((1232, 1))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(matches!(
            bad_version.additionals[0].data,
            RData::Opt {
                extended_rcode: 1,
                version: 0,
                ..
            }
        ));
        assert!(bad_version.answers.is_empty());
    }

    #[test]
    fn legacy_udp_is_limited_to_512_and_keeps_whole_records() {
        let mut data = ".example::ns.example\n".to_owned();
        for index in 0..80 {
            data.push_str(&format!("+many.example:192.0.2.{}\n", index % 250 + 1));
        }
        let zone = Zone::parse(&data).unwrap();
        let wire = respond(&zone, &query("many.example", RecordType::A, None), 4096).unwrap();
        assert!(wire.len() <= 512);
        let response = Message::decode(&wire).unwrap();
        assert_ne!(response.flags & 0x0200, 0);
        assert!(!response.answers.is_empty());
        assert!(response.answers.len() < 80);
    }

    #[test]
    fn live_udp_and_tcp_service() {
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let mut data = ".example::ns.example\n+www.example:192.0.2.1\n".to_owned();
        for index in 0..80 {
            data.push_str(&format!("+many.example:192.0.2.{}\n", index % 250 + 1));
        }
        let zone = Zone::parse(&data).unwrap();
        thread::spawn(move || serve_sockets(zone, udp, tcp).unwrap());

        let request = query("www.example", RecordType::A, None);
        let udp_client = UdpSocket::bind("127.0.0.1:0").unwrap();
        udp_client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        udp_client.send_to(&request, address).unwrap();
        let mut buffer = [0; 2048];
        let size = udp_client.recv(&mut buffer).unwrap();
        assert_eq!(Message::decode(&buffer[..size]).unwrap().answers.len(), 1);

        let mut tcp_client = TcpStream::connect(address).unwrap();
        tcp_client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        tcp_client
            .write_all(&(request.len() as u16).to_be_bytes())
            .unwrap();
        tcp_client.write_all(&request).unwrap();
        let mut length = [0; 2];
        tcp_client.read_exact(&mut length).unwrap();
        let mut response = vec![0; u16::from_be_bytes(length) as usize];
        tcp_client.read_exact(&mut response).unwrap();
        assert_eq!(Message::decode(&response).unwrap().answers.len(), 1);

        let large_request = query("many.example", RecordType::A, None);
        let mut tcp_client = TcpStream::connect(address).unwrap();
        tcp_client
            .write_all(&(large_request.len() as u16).to_be_bytes())
            .unwrap();
        tcp_client.write_all(&large_request).unwrap();
        tcp_client.read_exact(&mut length).unwrap();
        let mut response = vec![0; u16::from_be_bytes(length) as usize];
        tcp_client.read_exact(&mut response).unwrap();
        let response = Message::decode(&response).unwrap();
        assert_eq!(response.flags & 0x0200, 0);
        assert_eq!(response.answers.len(), 80, "{response:#?}");
    }
}
