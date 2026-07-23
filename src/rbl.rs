//! djbdns-compatible IPv4 DNS block-list data and responses.

use crate::{Error, Message, Name, RData, Record, RecordType, Result};
use std::{collections::HashSet, fs, net::Ipv4Addr, path::Path};

#[derive(Clone, Debug)]
pub struct Database {
    networks: HashSet<(u32, u8)>,
    responses: Vec<(Ipv4Addr, Vec<u8>)>,
}

impl Database {
    pub fn parse(text: &str) -> Result<Self> {
        let mut database = Self {
            networks: HashSet::new(),
            responses: Vec::new(),
        };
        for (line_number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches([' ', '\t', '\r']);
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(setting) = line.strip_prefix(':') {
                let (address, message) = setting.split_once(':').ok_or_else(|| {
                    Error::InvalidRecord(format!(
                        "line {}: missing response colon",
                        line_number + 1
                    ))
                })?;
                let address = address.parse().map_err(|_| {
                    Error::InvalidRecord(format!(
                        "line {}: malformed response IPv4 address",
                        line_number + 1
                    ))
                })?;
                database
                    .responses
                    .push((address, message.as_bytes().to_vec()));
                continue;
            }
            let (address, prefix) = parse_network(line).map_err(|error| {
                Error::InvalidRecord(format!("line {}: {error}", line_number + 1))
            })?;
            database.networks.insert((address, prefix));
        }
        Ok(database)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.extension().is_some_and(|extension| extension == "cdb") {
            Self::from_cdb(path)
        } else {
            Self::parse(&fs::read_to_string(path)?)
        }
    }

    fn from_cdb(path: &Path) -> Result<Self> {
        let mut database = Self {
            networks: HashSet::new(),
            responses: Vec::new(),
        };
        for (key, value) in crate::cdb::read_entries(path)? {
            match key.len() {
                0 if value.len() >= 4 => {
                    database.responses.push((
                        Ipv4Addr::new(value[0], value[1], value[2], value[3]),
                        value[4..value.len().min(100)].to_vec(),
                    ));
                }
                5 => {
                    let address = u32::from_be_bytes([key[0], key[1], key[2], key[3]]);
                    database.networks.insert((address, key[4].min(32)));
                }
                0 => {}
                _ => return Err(Error::Format("invalid rbldns CDB key")),
            }
        }
        Ok(database)
    }

    fn listed(&self, address: Ipv4Addr) -> bool {
        let address = u32::from(address);
        (8..=32).rev().any(|prefix| {
            let mask = if prefix == 0 {
                0
            } else {
                u32::MAX << (32 - prefix)
            };
            self.networks.contains(&(address & mask, prefix))
        })
    }

    fn response(&self) -> (Ipv4Addr, &[u8]) {
        self.responses
            .first()
            .map(|(address, text)| (*address, text.as_slice()))
            .unwrap_or((Ipv4Addr::new(127, 0, 0, 2), b"Listed $"))
    }

    pub fn respond(&self, base: &Name, wire: &[u8], _limit: usize) -> Result<Vec<u8>> {
        let query = Message::decode(wire)?;
        if query.flags & 0x8000 != 0 || query.questions.len() != 1 {
            return Err(Error::Format("expected one query"));
        }
        let question = query.questions[0].clone();
        let mut response = Message {
            id: query.id,
            flags: 0x8000 | 0x0400,
            questions: vec![question.clone()],
            ..Default::default()
        };
        let wants_a = matches!(question.qtype, RecordType::A | RecordType::Any);
        let wants_txt = matches!(question.qtype, RecordType::Txt | RecordType::Any);
        let Some(labels) = numeric_prefix(&question.name, base, 4) else {
            response.flags = 0x8000 | 5;
            return response.encode();
        };
        if question.qclass != 1 || (!wants_a && !wants_txt) || labels.len() != 4 {
            response.flags = 0x8000 | 5;
            return response.encode();
        }
        let address = Ipv4Addr::new(labels[3], labels[2], labels[1], labels[0]);
        if !self.listed(address) {
            response.flags |= 3;
            return response.encode();
        }
        let (answer, configured_text) = self.response();
        if wants_a {
            response.answers.push(Record {
                name: question.name.clone(),
                ttl: 2048,
                data: RData::A(answer),
            });
        }
        if wants_txt {
            let mut text = configured_text[..configured_text.len().min(96)].to_vec();
            if text.last() == Some(&b'$') {
                text.pop();
                text.extend(address.to_string().bytes());
            }
            response.answers.push(Record {
                name: question.name,
                ttl: 2048,
                data: RData::Txt(vec![text]),
            });
        }
        response.encode()
    }
}

pub fn compile(database: &Database, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for (answer, text) in &database.responses {
        let mut value = answer.octets().to_vec();
        value.extend(text);
        writer
            .add(b"", &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    let mut networks = database.networks.iter().copied().collect::<Vec<_>>();
    networks.sort_unstable();
    for (address, prefix) in networks {
        let mut key = address.to_be_bytes().to_vec();
        key.push(prefix);
        writer
            .add(&key, b"")
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

fn parse_network(input: &str) -> std::result::Result<(u32, u8), &'static str> {
    let (address, prefix) = input.split_once('/').unwrap_or((input, "32"));
    let prefix = prefix.parse::<u8>().unwrap_or(32).min(32);
    let fields = address.split('.').collect::<Vec<_>>();
    if fields.is_empty() || fields.len() > 4 {
        return Err("malformed IPv4 network");
    }
    let mut octets = [0; 4];
    for (index, field) in fields.iter().enumerate() {
        octets[index] = field.parse().map_err(|_| "malformed IPv4 network")?;
    }
    let mask = if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    };
    Ok((u32::from_be_bytes(octets) & mask, prefix))
}

pub(crate) fn numeric_prefix(name: &Name, suffix: &Name, maximum: usize) -> Option<Vec<u8>> {
    let labels = name.labels().collect::<Vec<_>>();
    let suffix_labels = suffix.labels().collect::<Vec<_>>();
    if labels.len() < suffix_labels.len()
        || labels[labels.len() - suffix_labels.len()..] != suffix_labels
    {
        return None;
    }
    let prefix = &labels[..labels.len() - suffix_labels.len()];
    if prefix.len() > maximum {
        return None;
    }
    prefix
        .iter()
        .map(|label| {
            let text = std::str::from_utf8(label).ok()?;
            if text.len() > 1 && text.starts_with('0') {
                return None;
            }
            text.parse::<u8>().ok()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Question;
    use std::time::SystemTime;

    fn query(name: &str, record_type: RecordType) -> Vec<u8> {
        Message {
            id: 7,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: record_type,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap()
    }

    #[test]
    fn most_specific_network_and_substitution() {
        let database = Database::parse(":127.0.0.9:Blocked $\n1.2.3/24\n10/8\n").unwrap();
        let base = "rbl.example".parse().unwrap();
        let response = Message::decode(
            &database
                .respond(&base, &query("4.3.2.1.rbl.example", RecordType::Any), 512)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(response.answers.len(), 2);
        assert_eq!(
            response.answers[0].data,
            RData::A(Ipv4Addr::new(127, 0, 0, 9))
        );
        assert_eq!(
            response.answers[1].data,
            RData::Txt(vec![b"Blocked 1.2.3.4".to_vec()])
        );
        let missing = Message::decode(
            &database
                .respond(&base, &query("4.3.9.1.rbl.example", RecordType::A), 512)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(missing.flags & 15, 3);
    }

    #[test]
    fn cdb_roundtrip_preserves_rbl_answers() {
        let database = Database::parse(":127.0.0.3:Listed\n192.0.2/24\n").unwrap();
        let path = std::env::temp_dir().join(format!(
            "rgbdns-rbl-{}-{}.cdb",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        compile(&database, &path).unwrap();
        let loaded = Database::from_file(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert!(loaded.listed(Ipv4Addr::new(192, 0, 2, 44)));
        assert!(!loaded.listed(Ipv4Addr::new(192, 0, 3, 44)));
        assert_eq!(loaded.response().0, Ipv4Addr::new(127, 0, 0, 3));
    }
}
