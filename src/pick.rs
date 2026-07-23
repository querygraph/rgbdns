//! Location-aware short-TTL address selection compatible with `pickdns`.

use crate::{Error, Message, Name, RData, Record, RecordType, Result};
use std::{
    collections::BTreeMap,
    fs,
    net::{IpAddr, Ipv4Addr},
    path::Path,
};

#[derive(Clone, Debug, Default)]
pub struct Database {
    addresses: BTreeMap<([u8; 2], Name), Vec<Ipv4Addr>>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
}

impl Database {
    pub fn parse(text: &str) -> Result<Self> {
        let mut database = Self::default();
        for (number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches([' ', '\t', '\r']);
            if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
                continue;
            }
            let fields = line[1..].split(':').collect::<Vec<_>>();
            match line.as_bytes()[0] {
                b'+' => {
                    let name = fields
                        .first()
                        .ok_or_else(|| bad_line(number, "missing name"))?
                        .parse()?;
                    let address = fields
                        .get(1)
                        .ok_or_else(|| bad_line(number, "missing address"))?
                        .parse()
                        .map_err(|_| bad_line(number, "malformed IPv4 address"))?;
                    let location = location(fields.get(2).copied().unwrap_or_default());
                    database
                        .addresses
                        .entry((location, name))
                        .or_default()
                        .push(address);
                }
                b'%' => {
                    let location = location(fields.first().copied().unwrap_or_default());
                    let prefix = ip_prefix(fields.get(1).copied().unwrap_or_default())
                        .map_err(|message| bad_line(number, message))?;
                    database.locations.push((prefix, location));
                }
                _ => return Err(bad_line(number, "unrecognized leading character")),
            }
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
        let mut database = Self::default();
        for (key, value) in crate::cdb::read_entries(path)? {
            if key.first() == Some(&b'%') && key.len() <= 5 {
                if value.len() != 2 {
                    return Err(Error::Format("invalid pickdns location value"));
                }
                database
                    .locations
                    .push((key[1..].to_vec(), [value[0], value[1]]));
            } else if key.first() == Some(&b'+') && key.len() >= 4 {
                if value.len() % 4 != 0 {
                    return Err(Error::Format("invalid pickdns address value"));
                }
                let name = crate::cdb::decode_name(&key[3..])?;
                let addresses = value
                    .chunks_exact(4)
                    .map(|bytes| Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]))
                    .collect();
                database
                    .addresses
                    .insert(([key[1], key[2]], name), addresses);
            } else {
                return Err(Error::Format("invalid pickdns CDB key"));
            }
        }
        Ok(database)
    }

    pub fn respond(&self, wire: &[u8], _limit: usize, client: IpAddr) -> Result<Vec<u8>> {
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
        if question.qclass != 1
            || !matches!(
                question.qtype,
                RecordType::A | RecordType::Mx | RecordType::Any
            )
        {
            response.flags = 0x8000 | 5;
            return response.encode();
        }
        let selected = self.client_location(client);
        let addresses = self
            .addresses
            .get(&(selected, question.name.clone()))
            .or_else(|| self.addresses.get(&([0, 0], question.name.clone())));
        let Some(addresses) = addresses else {
            response.flags = 0x8000 | 5;
            return response.encode();
        };
        if matches!(question.qtype, RecordType::A | RecordType::Any) {
            let mut addresses = addresses.clone();
            shuffle(&mut addresses)?;
            response
                .answers
                .extend(addresses.into_iter().take(3).map(|address| Record {
                    name: question.name.clone(),
                    ttl: 5,
                    data: RData::A(address),
                }));
        }
        response.encode()
    }

    fn client_location(&self, client: IpAddr) -> [u8; 2] {
        let IpAddr::V4(client) = client else {
            return [0, 0];
        };
        let octets = client.octets();
        let mut selected = [0, 0];
        let mut length = None;
        for (prefix, location) in &self.locations {
            if octets.starts_with(prefix) && length.is_none_or(|current| prefix.len() > current) {
                selected = *location;
                length = Some(prefix.len());
            }
        }
        selected
    }
}

pub fn compile(database: &Database, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for (prefix, location) in &database.locations {
        let mut key = vec![b'%'];
        key.extend(prefix);
        writer
            .add(&key, location)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    for ((location, name), addresses) in &database.addresses {
        let mut key = vec![b'+'];
        key.extend(location);
        key.extend(name.to_wire());
        let value = addresses
            .iter()
            .flat_map(|address| address.octets())
            .collect::<Vec<_>>();
        writer
            .add(&key, &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

fn location(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}

fn ip_prefix(value: &str) -> std::result::Result<Vec<u8>, &'static str> {
    let prefix = value
        .split('.')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().map_err(|_| "malformed IPv4 prefix"))
        .collect::<std::result::Result<Vec<u8>, _>>()?;
    if prefix.len() > 4 {
        Err("IPv4 prefix has more than four octets")
    } else {
        Ok(prefix)
    }
}

fn bad_line(number: usize, message: &str) -> Error {
    Error::InvalidRecord(format!("line {}: {message}", number + 1))
}

fn shuffle<T>(values: &mut [T]) -> Result<()> {
    for end in (1..values.len()).rev() {
        let upper = end as u64 + 1;
        let threshold = u64::MAX - (u64::MAX % upper);
        let index = loop {
            let mut bytes = [0; 8];
            getrandom::fill(&mut bytes)
                .map_err(|_| Error::Io(std::io::Error::other("OS randomness unavailable")))?;
            let value = u64::from_ne_bytes(bytes);
            if value < threshold {
                break (value % upper) as usize;
            }
        };
        values.swap(end, index);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Question;
    use std::time::SystemTime;

    fn query(name: &str) -> Vec<u8> {
        Message {
            id: 22,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap()
    }

    #[test]
    fn location_selection_falls_back_and_caps_answers() {
        let database = Database::parse(
            "%aa:192.0.2\n\
             +www.example:192.0.2.1:\n\
             +www.example:192.0.2.2:aa\n\
             +www.example:192.0.2.3:aa\n\
             +www.example:192.0.2.4:aa\n\
             +www.example:192.0.2.5:aa\n",
        )
        .unwrap();
        let local = Message::decode(
            &database
                .respond(&query("www.example"), 512, "192.0.2.44".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert_eq!(local.answers.len(), 3);
        assert!(local.answers.iter().all(|record| record.ttl == 5));
        let fallback = Message::decode(
            &database
                .respond(&query("www.example"), 512, "198.51.100.1".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
        assert_eq!(fallback.answers.len(), 1);
        assert_eq!(
            fallback.answers[0].data,
            RData::A(Ipv4Addr::new(192, 0, 2, 1))
        );
    }

    #[test]
    fn cdb_roundtrip_preserves_locations_and_addresses() {
        let database = Database::parse("%aa:192.0.2\n+www.example:192.0.2.1:aa\n").unwrap();
        let path = std::env::temp_dir().join(format!(
            "rgbdns-pick-{}-{}.cdb",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        compile(&database, &path).unwrap();
        let loaded = Database::from_file(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert_eq!(
            loaded.addresses[&([b'a', b'a'], "www.example".parse().unwrap())],
            [Ipv4Addr::new(192, 0, 2, 1)]
        );
    }
}
