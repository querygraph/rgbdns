use crate::{
    Error, Result,
    packet::Message,
    zone::{Lookup, Zone},
};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

#[derive(Clone, Copy)]
struct QueryLogger {
    enabled: bool,
}

impl QueryLogger {
    fn from_env() -> Result<Self> {
        let enabled = match std::env::var("QUERY_LOG") {
            Ok(value) => parse_query_log(Some(&value))?,
            Err(std::env::VarError::NotPresent) => true,
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(Error::Format("invalid QUERY_LOG"));
            }
        };
        Ok(Self { enabled })
    }

    fn starting(self) {
        if self.enabled {
            write_log_line("starting tinydns");
        }
    }

    fn request(self, peer: SocketAddr, wire: &[u8], response: Option<&[u8]>) {
        if self.enabled {
            write_log_line(&query_log_line(peer, wire, response));
        }
    }

    fn axfr(self, peer: SocketAddr, wire: &[u8], accepted: bool) {
        if self.enabled {
            write_log_line(&query_log_line_with_code(
                peer,
                wire,
                if accepted { '+' } else { '-' },
            ));
        }
    }
}

fn parse_query_log(value: Option<&str>) -> Result<bool> {
    match value.map(str::trim).map(str::to_ascii_lowercase).as_deref() {
        None | Some("1" | "true" | "yes" | "on") => Ok(true),
        Some("0" | "false" | "no" | "off") => Ok(false),
        Some(_) => Err(Error::Format("invalid QUERY_LOG")),
    }
}

fn write_log_line(line: &str) {
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let _ = writeln!(stderr, "{line}");
}

fn query_log_line(peer: SocketAddr, wire: &[u8], response: Option<&[u8]>) -> String {
    let Ok(query) = Message::decode(wire) else {
        return malformed_log_line(peer);
    };
    if query.questions.len() != 1 {
        return malformed_log_line(peer);
    }
    let code = if query.flags & 0x8000 != 0 || query.flags & 0x7800 != 0 {
        'I'
    } else if query.questions[0].qclass != 1 && query.questions[0].qclass != 255 {
        'C'
    } else if response.is_some_and(|wire| wire.get(3).is_some_and(|flags| flags & 0x0f == 5)) {
        '-'
    } else {
        '+'
    };
    query_log_line_for_query(peer, &query, code)
}

fn query_log_line_with_code(peer: SocketAddr, wire: &[u8], code: char) -> String {
    Message::decode(wire).map_or_else(
        |_| malformed_log_line(peer),
        |query| {
            if query.questions.len() == 1 {
                query_log_line_for_query(peer, &query, code)
            } else {
                malformed_log_line(peer)
            }
        },
    )
}

fn query_log_line_for_query(peer: SocketAddr, query: &Message, code: char) -> String {
    let question = &query.questions[0];
    let display_name = question.name.to_string();
    let display_name = if display_name == "." {
        display_name.as_str()
    } else {
        display_name.strip_suffix('.').unwrap_or(&display_name)
    };
    format!(
        "{}:{:04x}:{:04x} {code} {:04x} {}",
        log_ip(peer.ip()),
        peer.port(),
        query.id,
        question.qtype.code(),
        display_name
    )
}

fn malformed_log_line(peer: SocketAddr) -> String {
    format!("{}:{:04x}:0000 / 0000 .", log_ip(peer.ip()), peer.port())
}

fn log_ip(address: IpAddr) -> String {
    match address {
        IpAddr::V4(address) => address
            .octets()
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect(),
        IpAddr::V6(address) => address
            .octets()
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect(),
    }
}

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
    let dnssec_ok = opt.is_some_and(|(_, _, flags)| flags & 0x8000 != 0);
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
    if question.qclass != 1 && question.qclass != 255 {
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
                if question.qtype == crate::RecordType::Any && !dnssec_ok {
                    r.answers.retain(|record| !is_dnssec_type(record.rr_type()));
                }
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
        if dnssec_ok {
            add_dnssec_records(zone, &question, &mut r, client);
        }
    }
    normalize_rrsets(&mut r.answers);
    normalize_rrsets(&mut r.authorities);
    normalize_rrsets(&mut r.additionals);
    truncate(r, response_limit)
}

fn is_dnssec_type(record_type: crate::RecordType) -> bool {
    matches!(
        record_type,
        crate::RecordType::Ds
            | crate::RecordType::Rrsig
            | crate::RecordType::Nsec
            | crate::RecordType::Dnskey
            | crate::RecordType::Unknown(50)
            | crate::RecordType::Unknown(51)
    )
}

fn add_dnssec_records(
    zone: &Zone,
    question: &crate::Question,
    response: &mut Message,
    client: Option<IpAddr>,
) {
    add_section_signatures(zone, &mut response.answers, client);
    add_section_signatures(zone, &mut response.authorities, client);

    match response.flags & 0x000f {
        3 => response
            .authorities
            .extend(zone.dnssec_nsec_zone(&question.name, client)),
        0 if response.answers.is_empty() && response.flags & 0x0400 != 0 => {
            let mut proof = zone.dnssec_nsec_at(&question.name, client);
            if proof.is_empty() {
                proof = zone.dnssec_nsec_zone(&question.name, client);
            }
            response.authorities.extend(proof);
        }
        _ => {}
    }

    let wildcard_answer = response.answers.iter().any(|record| {
        if let crate::RData::Opaque(crate::RecordType::Rrsig, bytes) = &record.data {
            bytes
                .get(3)
                .is_some_and(|labels| usize::from(*labels) < record.name.labels().count())
        } else {
            false
        }
    });
    if wildcard_answer {
        response
            .authorities
            .extend(zone.dnssec_nsec_zone(&question.name, client));
    }
}

fn add_section_signatures(zone: &Zone, section: &mut Vec<crate::Record>, client: Option<IpAddr>) {
    let rrsets = section
        .iter()
        .filter(|record| record.rr_type() != crate::RecordType::Rrsig)
        .fold(
            HashMap::<crate::Name, std::collections::BTreeSet<_>>::new(),
            |mut sets, record| {
                sets.entry(record.name.clone())
                    .or_default()
                    .insert(record.rr_type());
                sets
            },
        );
    for (name, covered) in rrsets {
        section.extend(zone.dnssec_signatures(&name, &covered, client));
    }
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
    loop {
        let wire = response.encode()?;
        if wire.len() <= limit {
            return Ok(wire);
        }
        if remove_tail_rrset(&mut response.additionals, true)
            || remove_tail_rrset(&mut response.authorities, false)
            || remove_tail_rrset(&mut response.answers, false)
        {
            continue;
        }
        if response.additionals.pop().is_some() {
            continue;
        }
        if !response.questions.is_empty() {
            response.questions.clear();
            continue;
        }
        return Err(Error::Format("DNS response cannot fit transport limit"));
    }
}

fn remove_tail_rrset(records: &mut Vec<crate::Record>, preserve_opt: bool) -> bool {
    let Some(index) = records
        .iter()
        .rposition(|record| !preserve_opt || record.rr_type() != crate::RecordType::Opt)
    else {
        return false;
    };
    let name = records[index].name.clone();
    let record_type = match &records[index].data {
        crate::RData::Opaque(crate::RecordType::Rrsig, bytes) if bytes.len() >= 2 => {
            crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
        }
        _ => records[index].rr_type(),
    };
    let signed = records.iter().any(|record| {
        record.name == name
            && matches!(
                &record.data,
                crate::RData::Opaque(crate::RecordType::Rrsig, bytes)
                    if bytes.len() >= 2
                        && crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
                            == record_type
            )
    });
    if !signed && records[index].rr_type() != crate::RecordType::Rrsig {
        records.remove(index);
        return true;
    }
    records.retain(|record| {
        if record.name != name {
            return true;
        }
        if record.rr_type() == record_type {
            return false;
        }
        !matches!(
            &record.data,
            crate::RData::Opaque(crate::RecordType::Rrsig, bytes)
                if bytes.len() >= 2
                    && crate::RecordType::from_code(u16::from_be_bytes([bytes[0], bytes[1]]))
                        == record_type
        )
    });
    true
}

pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let query_logger = QueryLogger::from_env()?;
    let resolver = zone
        .has_anames()
        .then(crate::aname::Resolver::from_system)
        .transpose()?
        .map(Arc::new);
    let live = crate::acme::LiveZone::new(zone.clone());
    let acme = std::env::var("ACME_UPDATE_CONFIG")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(|config| {
            let state_dir = std::env::var("ACME_STATE_DIR")
                .unwrap_or_else(|_| "/var/lib/rgbdns/tinydns".into());
            let publication = match (
                std::env::var("ACME_PUBLISH_COMMAND").ok(),
                std::env::var("ACME_PUBLISHED_DATA").ok(),
            ) {
                (Some(command), Some(data)) => {
                    Some(crate::acme::Publication::new(command.into(), data.into())?)
                }
                (None, None) => None,
                _ => {
                    return Err(Error::Format(
                        "ACME_PUBLISH_COMMAND and ACME_PUBLISHED_DATA must be set together",
                    ));
                }
            };
            crate::acme::AcmeUpdates::from_file_with_publication(
                config,
                state_dir,
                zone.clone(),
                live.clone(),
                publication,
            )
            .map(Arc::new)
        })
        .transpose()?;
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
    let stream_handler = if allowed.is_some() || acme.is_some() {
        Some(authoritative_stream_handler(
            live.clone(),
            allowed,
            acme.clone(),
            query_logger,
        ))
    } else {
        None
    };
    query_logger.starting();
    crate::transport::serve(
        addr,
        Arc::new(move |wire, limit, client| {
            if crate::acme::AcmeUpdates::is_update(wire) {
                let response = error_response(wire, 5);
                query_logger.request(client, wire, response.as_deref().ok());
                return response;
            }
            let zone = live.snapshot();
            let response = respond_over_transport(
                &zone,
                resolver.as_deref(),
                wire,
                limit,
                limit <= 4096,
                Some(client.ip()),
            );
            query_logger.request(client, wire, response.as_deref().ok());
            response
        }),
        stream_handler,
    )
}

fn authoritative_stream_handler(
    zone: crate::acme::LiveZone,
    allowed: Option<Arc<Vec<ipnet::IpNet>>>,
    acme: Option<Arc<crate::acme::AcmeUpdates>>,
    query_logger: QueryLogger,
) -> Arc<crate::transport::StreamHandler> {
    Arc::new(move |wire: &[u8], client: SocketAddr| {
        if crate::acme::AcmeUpdates::is_update(wire) {
            let response = if let Some(acme) = &acme {
                acme.handle(wire)
            } else {
                error_response(wire, 5)
            }?;
            query_logger.request(client, wire, Some(&response));
            return Ok(Some(vec![response]));
        }
        let Ok(query) = Message::decode(wire) else {
            return Ok(None);
        };
        let is_axfr =
            query.questions.len() == 1 && query.questions[0].qtype == crate::RecordType::Axfr;
        if !is_axfr {
            return Ok(None);
        }
        let Some(allowed) = &allowed else {
            return Ok(None);
        };
        if !allowed.iter().any(|network| network.contains(&client.ip())) {
            query_logger.axfr(client, wire, false);
            return Err(Error::Format("AXFR client is not allowed"));
        }
        let snapshot = zone.snapshot();
        let response = crate::axfr::response_wires(&snapshot, query).map(Some);
        query_logger.axfr(client, wire, response.is_ok());
        response
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
    let query_logger = QueryLogger { enabled: false };
    let stream_handler = allowed.map(|allowed| {
        let live = crate::acme::LiveZone::new((*zone).clone());
        authoritative_stream_handler(live, Some(Arc::new(allowed)), None, query_logger)
    });
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
                Some(client.ip()),
            )
        }),
        stream_handler,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Name, Question, RData, Record, RecordType, zone::RecordMetadata};
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

    fn signed_test_zone() -> Zone {
        let directory =
            std::env::temp_dir().join(format!("rgbdns-server-dnssec-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let key = directory.join("example.pk8");
        let zone_name = "example".parse().unwrap();
        let policy = crate::dnssec::generate_key(&zone_name, &key).unwrap();
        let source = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n\
             +*.wild.example:192.0.2.2:300\n\
             &child.example:192.0.2.54:ns.child.example:300\n",
        )
        .unwrap();
        let records = crate::dnssec::sign_zone(&source, &policy).unwrap();
        std::fs::remove_dir_all(directory).unwrap();
        Zone::from_compiled_records(
            records
                .into_iter()
                .map(|record| (record, RecordMetadata::default()))
                .collect(),
            Vec::new(),
            Vec::new(),
        )
    }

    #[test]
    fn dnssec_is_do_gated_and_serves_signed_denial_and_ds_denial() {
        let zone = signed_test_zone();
        let unsigned = Message::decode(
            &respond(&zone, &query("www.example", RecordType::A, None), 4096).unwrap(),
        )
        .unwrap();
        assert_eq!(
            unsigned
                .answers
                .iter()
                .filter(|record| record.rr_type() == RecordType::Rrsig)
                .count(),
            0
        );

        let signed = Message::decode(
            &respond(
                &zone,
                &query("www.example", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert!(
            signed
                .answers
                .iter()
                .any(|record| record.rr_type() == RecordType::A)
        );
        assert!(
            signed
                .answers
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );

        let nonexistent = Message::decode(
            &respond(
                &zone,
                &query("missing.example", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(nonexistent.flags & 0x000f, 3);
        assert!(
            nonexistent
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            nonexistent
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );

        let randomized_case = Message::decode(
            &respond(
                &zone,
                &query("MiSsInG.ExAmPlE", RecordType::A, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(randomized_case.flags & 0x000f, 3);
        assert!(
            randomized_case
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            randomized_case
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );

        let no_ds = Message::decode(
            &respond(
                &zone,
                &query("child.example", RecordType::Ds, Some((4096, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_ne!(no_ds.flags & 0x0400, 0);
        assert!(no_ds.answers.is_empty());
        assert!(
            no_ds
                .authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
    }

    #[test]
    fn query_logs_match_the_original_tinydns_ipv4_format() {
        let peer = "127.0.0.1:57876".parse().unwrap();
        let request = query("fieldnotes.es", RecordType::A, None);
        let response = respond(
            &Zone::parse(".fieldnotes.es::a.ns.cron.sh\n").unwrap(),
            &request,
            4096,
        )
        .unwrap();
        assert_eq!(
            query_log_line(peer, &request, Some(&response)),
            "7f000001:e214:1234 + 0001 fieldnotes.es"
        );

        let refused = respond(
            &Zone::parse(".example::ns.example\n").unwrap(),
            &request,
            4096,
        )
        .unwrap();
        assert_eq!(
            query_log_line(peer, &request, Some(&refused)),
            "7f000001:e214:1234 - 0001 fieldnotes.es"
        );
    }

    #[test]
    fn query_logs_cover_malformed_unimplemented_class_ipv6_and_axfr() {
        let ipv4 = "192.0.2.1:53".parse().unwrap();
        assert_eq!(
            query_log_line(ipv4, &[0, 1, 2], None),
            "c0000201:0035:0000 / 0000 ."
        );

        let mut unimplemented = Message::decode(&query("example", RecordType::Aaaa, None)).unwrap();
        unimplemented.flags |= 0x0800;
        let unimplemented = unimplemented.encode().unwrap();
        assert_eq!(
            query_log_line(ipv4, &unimplemented, None),
            "c0000201:0035:1234 I 001c example"
        );

        let mut other_class = Message::decode(&query("example", RecordType::A, None)).unwrap();
        other_class.questions[0].qclass = 3;
        let other_class = other_class.encode().unwrap();
        assert_eq!(
            query_log_line(ipv4, &other_class, None),
            "c0000201:0035:1234 C 0001 example"
        );

        let ipv6 = "[2001:db8::1]:4660".parse().unwrap();
        let axfr = query("example", RecordType::Axfr, None);
        assert_eq!(
            query_log_line_with_code(ipv6, &axfr, '+'),
            "20010db8000000000000000000000001:1234:1234 + 00fc example"
        );
    }

    #[test]
    fn query_log_names_cannot_inject_lines() {
        let peer = "127.0.0.1:53".parse().unwrap();
        let request = query("line\\010break.example", RecordType::Txt, None);
        let line = query_log_line(peer, &request, None);
        assert_eq!(line, "7f000001:0035:1234 + 0010 line\\010break.example");
        assert!(!line.contains('\n'));
        assert!(!line.contains('\r'));
    }

    #[test]
    fn query_logging_defaults_on_and_has_an_explicit_opt_out() {
        assert!(parse_query_log(None).unwrap());
        for value in ["1", "true", "YES", "on"] {
            assert!(parse_query_log(Some(value)).unwrap());
        }
        for value in ["0", "false", "NO", "off"] {
            assert!(!parse_query_log(Some(value)).unwrap());
        }
        assert!(parse_query_log(Some("sometimes")).is_err());
    }

    #[test]
    fn any_class_retains_original_tinydns_behavior() {
        let zone = Zone::parse(".example::ns.example\n+www.example:192.0.2.1\n").unwrap();
        let mut request = Message::decode(&query("www.example", RecordType::A, None)).unwrap();
        request.questions[0].qclass = 255;
        let response =
            Message::decode(&respond(&zone, &request.encode().unwrap(), 4096).unwrap()).unwrap();
        assert_eq!(response.flags & 0x000f, 0);
        assert_eq!(response.answers.len(), 1);
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
        let ds = Message::decode(
            &respond(
                &zone,
                &query("child.example", RecordType::Ds, Some((1232, 0))),
                4096,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(ds.flags & 0x0400, 0);
        assert!(
            ds.authorities
                .iter()
                .any(|record| record.rr_type() == RecordType::Ns)
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
