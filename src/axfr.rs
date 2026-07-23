//! Bounded DNS-over-TCP full-zone transfers.

use crate::{Error, Message, Name, Question, RData, Record, RecordType, Result, zone::Zone};
use ipnet::IpNet;
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::Path,
    sync::Arc,
    thread,
    time::Duration,
};

const MAX_TCP_MESSAGE: usize = u16::MAX as usize;
const MAX_TRANSFER_RECORDS: usize = 1_000_000;
const MAX_TRANSFER_MESSAGES: usize = 1_000_000;
const MAX_TRANSFER_BYTES: usize = 1 << 30;

pub fn serve(zone: Zone, address: &str, allowed: Vec<IpNet>) -> Result<()> {
    serve_listener(
        Arc::new(zone),
        TcpListener::bind(address)?,
        Arc::new(allowed),
    )
}

pub fn serve_listener(
    zone: Arc<Zone>,
    listener: TcpListener,
    allowed: Arc<Vec<IpNet>>,
) -> Result<()> {
    let mut workers = Vec::new();
    for _ in 0..16 {
        let zone = zone.clone();
        let allowed = allowed.clone();
        let listener = listener.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let peer = match stream.peer_addr() {
                    Ok(peer) => peer,
                    Err(_) => continue,
                };
                if !allowed.iter().any(|network| network.contains(&peer.ip())) {
                    continue;
                }
                let _ = stream.set_read_timeout(Some(Duration::from_secs(30)));
                let _ = stream.set_write_timeout(Some(Duration::from_secs(30)));
                let _ = serve_connection(&zone, &mut stream);
            }
        }));
    }
    for worker in workers {
        let _ = worker.join();
    }
    Ok(())
}

fn serve_connection(zone: &Zone, stream: &mut TcpStream) -> Result<()> {
    let query = read_message(stream)?;
    if query.flags & 0x8000 != 0 || query.questions.len() != 1 {
        return Err(Error::Format("expected one AXFR query"));
    }
    let question = query.questions[0].clone();
    if question.qclass != 1 || question.qtype != RecordType::Axfr {
        return write_response(stream, query.id, Some(question), 4, Vec::new());
    }
    let Some(records) = zone.transfer(&question.name) else {
        return write_response(stream, query.id, Some(question), 5, Vec::new());
    };
    let mut first = true;
    let mut batch = Vec::new();
    for record in records {
        batch.push(record);
        let candidate = response_wire(query.id, first.then(|| question.clone()), 0, batch.clone());
        if batch.len() <= 4096
            && candidate
                .as_ref()
                .is_ok_and(|wire| wire.len() <= MAX_TCP_MESSAGE)
        {
            continue;
        }
        let record = batch.pop().unwrap();
        if batch.is_empty() {
            return Err(Error::Format("AXFR record exceeds DNS TCP framing"));
        }
        write_response(
            stream,
            query.id,
            first.then(|| question.clone()),
            0,
            std::mem::take(&mut batch),
        )?;
        first = false;
        batch.push(record);
    }
    if !batch.is_empty() {
        write_response(stream, query.id, first.then_some(question), 0, batch)?;
    }
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    id: u16,
    question: Option<Question>,
    rcode: u16,
    answers: Vec<Record>,
) -> Result<()> {
    let wire = response_wire(id, question, rcode, answers)?;
    if wire.len() > MAX_TCP_MESSAGE {
        return Err(Error::Format("AXFR message exceeds DNS TCP framing"));
    }
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;
    Ok(())
}

fn response_wire(
    id: u16,
    question: Option<Question>,
    rcode: u16,
    answers: Vec<Record>,
) -> Result<Vec<u8>> {
    Message {
        id,
        flags: 0x8000 | 0x0400 | rcode,
        questions: question.into_iter().collect(),
        answers,
        ..Default::default()
    }
    .encode()
}

fn read_message(stream: &mut TcpStream) -> Result<Message> {
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut wire = vec![0; u16::from_be_bytes(length) as usize];
    stream.read_exact(&mut wire)?;
    Message::decode(&wire)
}

pub fn fetch(server: SocketAddr, zone: Name) -> Result<Vec<Record>> {
    let mut stream = TcpStream::connect_timeout(&server, Duration::from_secs(15))?;
    stream.set_read_timeout(Some(Duration::from_secs(30)))?;
    stream.set_write_timeout(Some(Duration::from_secs(30)))?;
    let id = random_id()?;
    let question = Question {
        name: zone.clone(),
        qtype: RecordType::Axfr,
        qclass: 1,
    };
    let wire = Message {
        id,
        questions: vec![question],
        ..Default::default()
    }
    .encode()?;
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;

    let mut records = Vec::new();
    let mut opening_soa = None;
    let mut transfer_bytes = 0usize;
    for message_number in 0..MAX_TRANSFER_MESSAGES {
        let response = read_message(&mut stream)?;
        transfer_bytes = transfer_bytes
            .checked_add(response.encode()?.len())
            .filter(|bytes| *bytes <= MAX_TRANSFER_BYTES)
            .ok_or(Error::Format("AXFR byte limit exceeded"))?;
        if response.id != id || response.flags & 0x8000 == 0 {
            return Err(Error::Format("invalid AXFR response"));
        }
        if response.flags & 0x000f != 0 {
            return Err(Error::Format("AXFR server returned an error"));
        }
        if response.answers.is_empty() {
            return Err(Error::Format("empty AXFR response"));
        }
        let answer_count = response.answers.len();
        for (answer_index, record) in response.answers.into_iter().enumerate() {
            if opening_soa.is_none() {
                if record.rr_type() != RecordType::Soa || record.name != zone {
                    return Err(Error::Format("AXFR does not begin with zone SOA"));
                }
                opening_soa = Some(record.clone());
            } else if opening_soa.as_ref() == Some(&record) {
                if answer_index + 1 != answer_count {
                    return Err(Error::Format("records follow closing AXFR SOA"));
                }
                records.push(record);
                return Ok(records);
            }
            records.push(record);
        }
        if records.len() > MAX_TRANSFER_RECORDS {
            return Err(Error::Format("AXFR record limit exceeded"));
        }
        if message_number + 1 == MAX_TRANSFER_MESSAGES {
            return Err(Error::Format("AXFR message limit exceeded"));
        }
    }
    Err(Error::Format("AXFR message limit exceeded"))
}

fn random_id() -> Result<u16> {
    let mut bytes = [0; 2];
    getrandom::fill(&mut bytes)
        .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
    Ok(u16::from_ne_bytes(bytes))
}

pub fn write_tinydns(records: &[Record], output: &Path, temporary: &Path) -> Result<()> {
    if output == temporary {
        return Err(Error::Format("AXFR output and temporary paths must differ"));
    }
    let mut file = File::create(temporary)?;
    let write_result: Result<()> = (|| {
        for (index, record) in records.iter().enumerate() {
            if index + 1 == records.len() && record.rr_type() == RecordType::Soa {
                continue;
            }
            writeln!(file, "{}", tinydns_line(record)?)?;
        }
        file.sync_all()?;
        Ok(())
    })();
    if let Err(error) = write_result {
        drop(file);
        let _ = fs::remove_file(temporary);
        return Err(error);
    }
    drop(file);
    fs::rename(temporary, output)?;
    Ok(())
}

fn tinydns_line(record: &Record) -> Result<String> {
    let owner = record.name.to_string();
    let ttl = record.ttl;
    Ok(match &record.data {
        RData::A(address) => format!("+{owner}:{address}:{ttl}"),
        RData::Aaaa(address) => format!("6{owner}:{address}:{ttl}"),
        RData::Name(RecordType::Ns, target) => format!("&{owner}::{target}:{ttl}"),
        RData::Name(RecordType::Cname, target) => format!("C{owner}:{target}:{ttl}"),
        RData::Name(RecordType::Ptr, target) => format!("^{owner}:{target}:{ttl}"),
        RData::Mx(preference, target) => {
            format!("@{owner}::{target}:{preference}:{ttl}")
        }
        RData::Soa {
            mname,
            admin,
            serial,
            refresh,
            retry,
            expire,
            minimum,
        } => {
            format!("Z{owner}:{mname}:{admin}:{serial}:{refresh}:{retry}:{expire}:{minimum}:{ttl}")
        }
        RData::Txt(chunks) => {
            let bytes = chunks.iter().flatten().copied().collect::<Vec<_>>();
            format!("'{owner}:{}:{ttl}", escape(&bytes))
        }
        RData::Srv {
            priority,
            weight,
            port,
            target,
        } => format!("S{owner}::{target}:{port}:{priority}:{weight}:{ttl}"),
        RData::Caa { flags, tag, value } => {
            let mut data = vec![
                *flags,
                tag.len()
                    .try_into()
                    .map_err(|_| Error::Format("CAA tag cannot be represented in tinydns data"))?,
            ];
            data.extend(tag);
            data.extend(value);
            format!(":{owner}:257:{}:{ttl}", escape(&data))
        }
        RData::Opaque(typ, bytes) => {
            format!(":{owner}:{}:{}:{ttl}", typ.code(), escape(bytes))
        }
        RData::Opt { .. } => return Err(Error::Format("OPT is invalid in AXFR zone data")),
        RData::Name(_, _) => return Err(Error::Format("invalid name-bearing RDATA type")),
    })
}

fn escape(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        if (b'!'..=b'~').contains(byte) && *byte != b':' && *byte != b'\\' {
            output.push(char::from(*byte));
        } else {
            output.push_str(&format!("\\{byte:03o}"));
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

    fn temp_path(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "rgbdns-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn live_transfer_has_matching_soa_bookends() {
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:60\n",
        )
        .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        thread::spawn(move || {
            serve_listener(
                Arc::new(zone),
                listener,
                Arc::new(vec!["127.0.0.0/8".parse().unwrap()]),
            )
            .unwrap();
        });
        let records = fetch(address, "example".parse().unwrap()).unwrap();
        assert!(records.len() >= 4);
        assert_eq!(records.first(), records.last());
        assert_eq!(records.first().unwrap().rr_type(), RecordType::Soa);
        assert!(records.iter().any(|record| {
            record.name == "www.example".parse().unwrap()
                && record.data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        }));
    }

    #[test]
    fn exported_tinydns_text_roundtrips() {
        let source = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n\
             &example:192.0.2.53:ns.example:300\n\
             @example:192.0.2.25:mail.example:20:400\n\
             'example:hello\\072world:64\n",
        )
        .unwrap();
        let records = source.transfer(&"example".parse().unwrap()).unwrap();
        let output = temp_path("axfr-data");
        let temporary = temp_path("axfr-data-tmp");
        write_tinydns(&records, &output, &temporary).unwrap();
        let imported = Zone::from_file(&output).unwrap();
        fs::remove_file(output).unwrap();
        assert!(matches!(
            imported.lookup(&"example".parse().unwrap(), RecordType::Mx),
            Lookup::Answer(records) if matches!(records[0].data, RData::Mx(20, _))
        ));
        assert!(matches!(
            imported.lookup(&"example".parse().unwrap(), RecordType::Txt),
            Lookup::Answer(_)
        ));
    }
}
