use crate::{
    Error, Result,
    packet::Message,
    zone::{Lookup, Zone},
};
use std::{
    collections::HashSet,
    io::{Read, Write},
    net::{IpAddr, TcpListener, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};

pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    respond_over_transport(zone, wire, transport_limit, true, None)
}

pub fn respond_from(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    client: IpAddr,
) -> Result<Vec<u8>> {
    respond_over_transport(zone, wire, transport_limit, true, Some(client))
}

fn respond_over_transport(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    is_udp: bool,
    client: Option<IpAddr>,
) -> Result<Vec<u8>> {
    let q = match Message::decode(wire) {
        Ok(query) => query,
        Err(_) if wire.len() >= 12 && wire[2] & 0x80 == 0 => {
            return error_response(wire, 1);
        }
        Err(error) => return Err(error),
    };
    if q.flags & 0x8000 != 0 {
        return Err(Error::Format("received a DNS response"));
    }
    if q.questions.len() != 1 {
        return error_response(wire, 1);
    }
    let question = q.questions[0].clone();
    if q.flags & 0x7800 != 0 {
        return error_response(wire, 4);
    }
    let options = q
        .additionals
        .iter()
        .filter_map(|record| match &record.data {
            crate::RData::Opt {
                udp_payload,
                version,
                flags,
                ..
            } => Some((*udp_payload, *version, *flags)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if options.len() > 1 {
        return error_response(wire, 1);
    }
    let opt = options.first().copied();
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
        match zone_lookup(zone, &question.name, question.qtype, client) {
            Lookup::Answer(x) => {
                r.answers = x;
                if !matches!(
                    question.qtype,
                    crate::RecordType::Cname | crate::RecordType::Any
                ) && !expand_cname_chain(zone, &mut r, question.qtype, client)
                {
                    r.flags = (r.flags & !0x000f) | 2;
                    r.answers.clear();
                }
                add_target_addresses(zone, &mut r, client);
            }
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

fn error_response(query: &[u8], rcode: u16) -> Result<Vec<u8>> {
    if query.len() < 4 {
        return Err(Error::Format("short DNS query"));
    }
    Message {
        id: u16::from_be_bytes([query[0], query[1]]),
        flags: 0x8000 | (u16::from_be_bytes([query[2], query[3]]) & 0x0100) | rcode,
        ..Default::default()
    }
    .encode()
}

fn zone_lookup(
    zone: &Zone,
    name: &crate::Name,
    record_type: crate::RecordType,
    client: Option<IpAddr>,
) -> Lookup {
    client.map_or_else(
        || zone.lookup(name, record_type),
        |address| zone.lookup_from(name, record_type, address),
    )
}

fn expand_cname_chain(
    zone: &Zone,
    response: &mut Message,
    record_type: crate::RecordType,
    client: Option<IpAddr>,
) -> bool {
    let mut visited = response
        .answers
        .iter()
        .map(|record| record.name.clone())
        .collect::<HashSet<_>>();
    for _ in 0..16 {
        if response
            .answers
            .iter()
            .any(|record| record.rr_type() == record_type)
        {
            return true;
        }
        let Some(target) = response.answers.iter().rev().find_map(|record| {
            if let crate::RData::Name(crate::RecordType::Cname, target) = &record.data {
                Some(target.clone())
            } else {
                None
            }
        }) else {
            return true;
        };
        if !visited.insert(target.clone()) {
            return false;
        }
        match zone_lookup(zone, &target, record_type, client) {
            Lookup::Answer(records) => response.answers.extend(records),
            Lookup::NoData(_) | Lookup::NxDomain(_) | Lookup::Referral { .. } | Lookup::Refused => {
                return true;
            }
        }
    }
    false
}

fn add_target_addresses(zone: &Zone, response: &mut Message, client: Option<IpAddr>) {
    let targets = response
        .answers
        .iter()
        .filter_map(|record| match &record.data {
            crate::RData::Name(crate::RecordType::Ns, target)
            | crate::RData::Mx(_, target)
            | crate::RData::Srv { target, .. } => Some(target.clone()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    for target in targets {
        for record_type in [crate::RecordType::A, crate::RecordType::Aaaa] {
            if let Lookup::Answer(records) = zone_lookup(zone, &target, record_type, client) {
                response
                    .additionals
                    .extend(records.into_iter().filter(|record| {
                        matches!(
                            record.rr_type(),
                            crate::RecordType::A | crate::RecordType::Aaaa
                        )
                    }));
            }
        }
    }
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
                && let Ok(r) = respond_over_transport(&z, &b[..n], 4096, true, Some(peer.ip()))
            {
                let _ = udp.send_to(&r, peer);
            }
        }
    });
    let mut workers = Vec::new();
    for _ in 0..32 {
        let z = zone.clone();
        let listener = tcp.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut s) = stream else {
                    continue;
                };
                let _ = s.set_read_timeout(Some(Duration::from_secs(30)));
                let _ = s.set_write_timeout(Some(Duration::from_secs(30)));
                let client = s.peer_addr().ok().map(|peer| peer.ip());
                let mut l = [0; 2];
                if s.read_exact(&mut l).is_ok() {
                    let mut b = vec![0; u16::from_be_bytes(l) as usize];
                    if s.read_exact(&mut b).is_ok()
                        && let Ok(r) = respond_over_transport(&z, &b, 65535, false, client)
                    {
                        let _ = s.write_all(&(r.len() as u16).to_be_bytes());
                        let _ = s.write_all(&r);
                    }
                }
            }
        }));
    }
    for worker in workers {
        let _ = worker.join();
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
    fn client_address_selects_tinydns_location() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             %aa:192.0.2\n\
             +www.example:192.0.2.1:60::aa\n\
             +www.example:198.51.100.1:60\n",
        )
        .unwrap();
        let response = Message::decode(
            &respond_over_transport(
                &zone,
                &query("www.example", RecordType::A, None),
                4096,
                true,
                Some("192.0.2.44".parse().unwrap()),
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            response
                .answers
                .iter()
                .any(|record| record.data == RData::A("192.0.2.1".parse().unwrap()))
        );
        assert!(
            response
                .answers
                .iter()
                .any(|record| record.data == RData::A("198.51.100.1".parse().unwrap()))
        );
    }

    #[test]
    fn expands_bounded_cname_chains_and_target_additionals() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             Calias.example:middle.example\n\
             Cmiddle.example:www.example\n\
             +www.example:192.0.2.1:60\n\
             @example:192.0.2.25:mail.example:10:300\n",
        )
        .unwrap();
        let cname = Message::decode(
            &respond(&zone, &query("alias.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(cname.answers.len(), 3);
        assert_eq!(cname.answers[2].rr_type(), RecordType::A);
        let mx = Message::decode(
            &respond(&zone, &query("example", RecordType::Mx, None), 4096).unwrap(),
        )
        .unwrap();
        assert!(
            mx.additionals
                .iter()
                .any(|record| record.data == RData::A("192.0.2.25".parse().unwrap()))
        );
    }

    #[test]
    fn cname_loops_return_servfail_with_bounded_work() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             Ca.example:b.example\n\
             Cb.example:a.example\n",
        )
        .unwrap();
        let response = Message::decode(
            &respond(&zone, &query("a.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(response.flags & 15, 2);
        assert!(response.answers.is_empty());
    }

    #[test]
    fn malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp() {
        let zone = Zone::parse(".example::ns.example\n").unwrap();
        let mut malformed = query("example", RecordType::A, None);
        malformed[5] = 2;
        let response = Message::decode(&respond(&zone, &malformed, 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 15, 1);
        assert!(response.questions.is_empty());
        assert_eq!(response.encode().unwrap().len(), 12);

        let mut opcode = query("example", RecordType::A, None);
        opcode[2] |= 0x08;
        let response = Message::decode(&respond(&zone, &opcode, 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 15, 4);

        let mut duplicate_opt =
            Message::decode(&query("example", RecordType::A, Some((1232, 0)))).unwrap();
        duplicate_opt
            .additionals
            .push(duplicate_opt.additionals[0].clone());
        let response =
            Message::decode(&respond(&zone, &duplicate_opt.encode().unwrap(), 4096).unwrap())
                .unwrap();
        assert_eq!(response.flags & 15, 1);
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
