---
type: "code-file"
source_path: "src/server.rs"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
line_count: 833
fragment_count: 26
rgbdns_commit: "79502939"
---

# src/server.rs

- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/server.rs`
- Lines: 833
- Summary: Source file in the Authoritative service subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-d3da65fbc724|respond]]: lines 12-19
- [[DNS from First Principles/Fragments/rgbdns-frag-e0990e79e0f5|respond_from]]: lines 20-39
- [[DNS from First Principles/Fragments/rgbdns-frag-85698b1ce866|respond_over_transport]]: lines 40-120
- [[DNS from First Principles/Fragments/rgbdns-frag-413307db626e|normalize_rrsets]]: lines 196-219
- [[DNS from First Principles/Fragments/rgbdns-frag-ff4f3e36f844|negative_soa]]: lines 220-226
- [[DNS from First Principles/Fragments/rgbdns-frag-aba7f2385996|error_response]]: lines 227-238
- [[DNS from First Principles/Fragments/rgbdns-frag-8a745a4a8208|zone_lookup]]: lines 239-250
- [[DNS from First Principles/Fragments/rgbdns-frag-4625cc9215e0|expand_cname_chain]]: lines 251-291
- [[DNS from First Principles/Fragments/rgbdns-frag-cb35766a16c9|add_target_addresses]]: lines 292-318
- [[DNS from First Principles/Fragments/rgbdns-frag-b79b1695d38f|truncate]]: lines 319-360
- [[DNS from First Principles/Fragments/rgbdns-frag-0f2b54775d5e|with_tail_records_removed]]: lines 361-386
- [[DNS from First Principles/Fragments/rgbdns-frag-31d10be4bc98|serve]]: lines 387-423
- [[DNS from First Principles/Fragments/rgbdns-frag-76bf4ddcc8ad|axfr_stream_handler]]: lines 424-444
- [[DNS from First Principles/Fragments/rgbdns-frag-053b53ae57e0|serve_sockets]]: lines 445-476
- [[DNS from First Principles/Fragments/rgbdns-frag-20be16ba4c51|tests]]: lines 477-486
- [[DNS from First Principles/Fragments/rgbdns-frag-6ee0c357f63d|query]]: lines 487-514
- [[DNS from First Principles/Fragments/rgbdns-frag-138ac655395d|referral_has_no_aa_and_contains_bailiwick_glue]]: lines 515-537
- [[DNS from First Principles/Fragments/rgbdns-frag-78a254dd8c92|client_address_selects_tinydns_location]]: lines 538-572
- [[DNS from First Principles/Fragments/rgbdns-frag-8f699a31cc8b|expands_bounded_cname_chains_and_target_additionals]]: lines 573-599
- [[DNS from First Principles/Fragments/rgbdns-frag-80e9e786f228|cname_loops_return_servfail_with_bounded_work]]: lines 600-615
- [[DNS from First Principles/Fragments/rgbdns-frag-a64686cbedbb|aname_synthesizes_authoritative_addresses_without_emitting_cname]]: lines 616-677
- [[DNS from First Principles/Fragments/rgbdns-frag-5c417caa83ca|malformed_queries_get_bounded_formerr_and_unknown_opcode_gets_notimp]]: lines 678-703
- [[DNS from First Principles/Fragments/rgbdns-frag-4020f1673346|edns_negotiation_and_bad_version]]: lines 704-743
- [[DNS from First Principles/Fragments/rgbdns-frag-afd2662300da|legacy_udp_is_limited_to_512_and_keeps_whole_records]]: lines 744-758
- [[DNS from First Principles/Fragments/rgbdns-frag-6c49d3a84895|live_udp_and_tcp_service]]: lines 759-811
- [[DNS from First Principles/Fragments/rgbdns-frag-95f0d422d9b2|integrated_tcp_listener_serves_axfr]]: lines 812-833

## Full Source

```rust
use crate::{
    Error, Result,
    packet::Message,
    zone::{Lookup, Zone},
};
use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    sync::Arc,
};

pub fn respond(zone: &Zone, wire: &[u8], transport_limit: usize) -> Result<Vec<u8>> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?;
    respond_over_transport(zone, resolver.as_ref(), wire, transport_limit, true, None)
}

pub fn respond_from(
    zone: &Zone,
    wire: &[u8],
    transport_limit: usize,
    client: IpAddr,
) -> Result<Vec<u8>> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?;
    respond_over_transport(
        zone,
        resolver.as_ref(),
        wire,
        transport_limit,
        true,
        Some(client),
    )
}

fn respond_over_transport(
    zone: &Zone,
    aname_resolver: Option<&crate::aname::Resolver>,
    wire: &[u8],
    transport_limit: usize,
    is_udp: bool,
    client: Option<IpAddr>,
) -> Result<Vec<u8>> {
    // Unknown opcodes can define a body layout different from QUERY. RFC 8906
    // therefore requires NOTIMP based on the header alone, without attempting
    // to parse the body as a standard question.
    if wire.len() >= 4 {
        let flags = u16::from_be_bytes([wire[2], wire[3]]);
        if flags & 0x8000 == 0 && flags & 0x7800 != 0 {
            return error_response(wire, 4);
        }
    }
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
    if q.answers
        .iter()
        .chain(&q.authorities)
        .any(|record| record.rr_type() == crate::RecordType::Opt)
    {
        return error_response(wire, 1);
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
        let ordinary_lookup = zone_lookup(zone, &question.name, question.qtype, client);
        let lookup = if matches!(&ordinary_lookup, Lookup::NoData(_) | Lookup::NxDomain(_))
            && matches!(
                question.qtype,
                crate::RecordType::A | crate::RecordType::Aaaa
            ) {
            if let Some(aname) = zone.aname(&question.name) {
                match aname_resolver
                    .ok_or(crate::Error::Format("ANAME resolver is unavailable"))
                    .and_then(|resolver| {
                        resolver.resolve(&question.name, &aname.target, question.qtype, aname.ttl)
                    }) {
                    Ok(records) if !records.is_empty() => Lookup::Answer(records),
                    Ok(_) => ordinary_lookup,
                    Err(_) => {
                        r.flags = (r.flags & !0x000f) | 2;
                        return truncate(r, response_limit);
                    }
                }
            } else {
                ordinary_lookup
            }
        } else {
            ordinary_lookup
        };
        match lookup {
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
                    r.authorities.push(negative_soa(x))
                }
            }
            Lookup::NxDomain(soa) => {
                r.flags |= 3;
                if let Some(x) = soa {
                    r.authorities.push(negative_soa(x))
                }
            }
            Lookup::Refused => r.flags |= 5,
        }
    }
    normalize_rrsets(&mut r.answers);
    normalize_rrsets(&mut r.authorities);
    normalize_rrsets(&mut r.additionals);
    truncate(r, response_limit)
}

fn normalize_rrsets(records: &mut Vec<crate::Record>) {
    let mut ttls = HashMap::new();
    for record in records.iter() {
        ttls.entry((record.name.clone(), record.rr_type()))
            .and_modify(|ttl: &mut u32| *ttl = (*ttl).min(record.ttl))
            .or_insert(record.ttl);
    }
    for record in records.iter_mut() {
        record.ttl = ttls[&(record.name.clone(), record.rr_type())];
    }
    let mut index = 0;
    while index < records.len() {
        if records[..index].iter().any(|record| {
            record.name == records[index].name
                && record.rr_type() == records[index].rr_type()
                && record.data == records[index].data
        }) {
            records.remove(index);
        } else {
            index += 1;
        }
    }
}

fn negative_soa(mut record: crate::Record) -> crate::Record {
    if let crate::RData::Soa { minimum, .. } = &record.data {
        record.ttl = record.ttl.min(*minimum);
    }
    record
}

fn error_response(query: &[u8], rcode: u16) -> Result<Vec<u8>> {
    if query.len() < 4 {
        return Err(Error::Format("short DNS query"));
    }
    Message {
        id: u16::from_be_bytes([query[0], query[1]]),
        flags: 0x8000 | (u16::from_be_bytes([query[2], query[3]]) & 0x7900) | rcode,
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
    let removable = response
        .additionals
        .iter()
        .filter(|record| record.rr_type() != crate::RecordType::Opt)
        .count()
        + response.authorities.len()
        + response.answers.len()
        + response
            .additionals
            .iter()
            .filter(|record| record.rr_type() == crate::RecordType::Opt)
            .count()
        + usize::from(!response.questions.is_empty());

    let mut low = 0;
    let mut high = removable;
    while low < high {
        let middle = low + (high - low) / 2;
        let candidate = with_tail_records_removed(&response, middle);
        if candidate.encode()?.len() <= limit {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    while low <= removable {
        let candidate = with_tail_records_removed(&response, low);
        let wire = candidate.encode()?;
        if wire.len() <= limit {
            return Ok(wire);
        }
        low += 1;
    }
    Err(Error::Format("DNS response cannot fit transport limit"))
}

fn with_tail_records_removed(response: &Message, mut count: usize) -> Message {
    let mut candidate = response.clone();
    while count != 0
        && let Some(index) = candidate
            .additionals
            .iter()
            .rposition(|record| record.rr_type() != crate::RecordType::Opt)
    {
        candidate.additionals.remove(index);
        count -= 1;
    }
    while count != 0 && candidate.authorities.pop().is_some() {
        count -= 1;
    }
    while count != 0 && candidate.answers.pop().is_some() {
        count -= 1;
    }
    while count != 0 && candidate.additionals.pop().is_some() {
        count -= 1;
    }
    if count != 0 {
        candidate.questions.clear();
    }
    candidate
}

pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let zone = Arc::new(zone);
    let allowed = std::env::var("ALLOW_NETS")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::parse::<ipnet::IpNet>)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_err(|_| Error::Format("invalid ALLOW_NETS"))
        })
        .transpose()?
        .map(Arc::new);
    let stream_handler = allowed.map(|allowed| axfr_stream_handler(zone.clone(), allowed));
    crate::transport::serve(
        addr,
        Arc::new(move |wire, limit, client| {
            respond_over_transport(
                &zone,
                resolver.as_deref(),
                wire,
                limit,
                limit <= 4096,
                Some(client),
            )
        }),
        stream_handler,
    )
}

fn axfr_stream_handler(
    zone: Arc<Zone>,
    allowed: Arc<Vec<ipnet::IpNet>>,
) -> Arc<crate::transport::StreamHandler> {
    Arc::new(move |wire: &[u8], client: IpAddr| {
        let Ok(query) = Message::decode(wire) else {
            return Ok(None);
        };
        let is_axfr =
            query.questions.len() == 1 && query.questions[0].qtype == crate::RecordType::Axfr;
        if !is_axfr {
            return Ok(None);
        }
        if !allowed.iter().any(|network| network.contains(&client)) {
            return Err(Error::Format("AXFR client is not allowed"));
        }
        crate::axfr::response_wires(&zone, query).map(Some)
    })
}

#[cfg(test)]
fn serve_sockets(
    zone: Zone,
    udp: std::net::UdpSocket,
    tcp: std::net::TcpListener,
    allowed: Option<Vec<ipnet::IpNet>>,
) -> Result<()> {
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let zone = Arc::new(zone);
    let stream_handler =
        allowed.map(|allowed| axfr_stream_handler(zone.clone(), Arc::new(allowed)));
    crate::transport::serve_sockets(
        udp,
        tcp,
        Arc::new(move |wire, limit, client| {
            respond_over_transport(
                &zone,
                resolver.as_deref(),
                wire,
                limit,
                limit <= 4096,
                Some(client),
            )
        }),
        stream_handler,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType};
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream, UdpSocket},
        thread,
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
                None,
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
    fn aname_synthesizes_authoritative_addresses_without_emitting_cname() {
        let upstream = UdpSocket::bind("127.0.0.1:0").unwrap();
        let upstream_address = upstream.local_addr().unwrap();
        let upstream_thread = thread::spawn(move || {
            for address in [
                RData::A("192.0.2.44".parse().unwrap()),
                RData::Aaaa("2001:db8::44".parse().unwrap()),
            ] {
                let mut wire = [0; 512];
                let (length, peer) = upstream.recv_from(&mut wire).unwrap();
                let request = Message::decode(&wire[..length]).unwrap();
                let response = Message {
                    id: request.id,
                    flags: 0x8000 | 0x0100,
                    questions: request.questions.clone(),
                    answers: vec![Record {
                        name: request.questions[0].name.clone(),
                        ttl: 600,
                        data: address,
                    }],
                    ..Default::default()
                }
                .encode()
                .unwrap();
                upstream.send_to(&response, peer).unwrap();
            }
        });
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             Aexample:blog-host.example.net:120\n",
        )
        .unwrap();
        let resolver = crate::aname::Resolver::new(vec![upstream_address]);
        for record_type in [RecordType::A, RecordType::Aaaa] {
            let response = Message::decode(
                &respond_over_transport(
                    &zone,
                    Some(&resolver),
                    &query("example", record_type, None),
                    4096,
                    true,
                    None,
                )
                .unwrap(),
            )
            .unwrap();
            assert_eq!(response.flags & 0x040f, 0x0400);
            assert_eq!(response.answers.len(), 1);
            assert_eq!(response.answers[0].name, "example".parse().unwrap());
            assert_eq!(response.answers[0].rr_type(), record_type);
            assert!(response.answers[0].ttl <= 120);
            assert!(
                response
                    .answers
                    .iter()
                    .all(|record| record.rr_type() != RecordType::Cname)
            );
        }
        upstream_thread.join().unwrap();
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
        thread::spawn(move || serve_sockets(zone, udp, tcp, None).unwrap());

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
        let mut pipelined = Vec::new();
        for _ in 0..2 {
            pipelined.extend((request.len() as u16).to_be_bytes());
            pipelined.extend(&request);
        }
        tcp_client.write_all(&pipelined).unwrap();
        let mut length = [0; 2];
        for _ in 0..2 {
            tcp_client.read_exact(&mut length).unwrap();
            let mut response = vec![0; u16::from_be_bytes(length) as usize];
            tcp_client.read_exact(&mut response).unwrap();
            assert_eq!(Message::decode(&response).unwrap().answers.len(), 1);
        }

        let large_request = query("many.example", RecordType::A, None);
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

    #[test]
    fn integrated_tcp_listener_serves_axfr() {
        let tcp = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = tcp.local_addr().unwrap();
        let udp = UdpSocket::bind(address).unwrap();
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:60\n",
        )
        .unwrap();
        thread::spawn(move || {
            serve_sockets(zone, udp, tcp, Some(vec!["127.0.0.0/8".parse().unwrap()])).unwrap()
        });

        let records = crate::axfr::fetch(address, "example".parse().unwrap()).unwrap();
        assert_eq!(records.first(), records.last());
        assert!(records.iter().any(|record| {
            record.name == "www.example".parse().unwrap()
                && record.data == RData::A("192.0.2.1".parse().unwrap())
        }));
    }
}
```
