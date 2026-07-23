use crate::{Error, Name, RData, Record, RecordType, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    net::{Ipv4Addr, Ipv6Addr},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Debug, Default)]
pub struct Zone {
    records: BTreeMap<Name, Vec<Record>>,
    authoritative: BTreeSet<Name>,
}
impl Zone {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::parse(&fs::read_to_string(path)?)
    }
    pub fn parse(text: &str) -> Result<Self> {
        let mut z = Self::default();
        for (number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
                continue;
            }
            z.add_line(line)
                .map_err(|e| Error::InvalidRecord(format!("line {}: {e}", number + 1)))?;
        }
        Ok(z)
    }
    fn add(&mut self, r: Record) {
        self.records.entry(r.name.clone()).or_default().push(r)
    }
    fn add_line(&mut self, line: &str) -> Result<()> {
        let kind = line.as_bytes()[0];
        let f = split_fields(&line[1..]);
        let name = field(&f, 0)?.parse::<Name>()?;
        let ttl = field_opt(&f, 2)
            .and_then(|x| x.parse().ok())
            .unwrap_or(match kind {
                b'.' | b'&' => 259200,
                b'Z' => 2560,
                _ => 86400,
            });
        match kind {
            b'=' | b'+' => {
                let ip = field(&f, 1)?
                    .parse::<Ipv4Addr>()
                    .map_err(|_| Error::InvalidRecord("bad IPv4".into()))?;
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::A(ip),
                });
                if kind == b'=' {
                    let rev = Name::from_str(&format!(
                        "{}.{}.{}.{}.in-addr.arpa",
                        ip.octets()[3],
                        ip.octets()[2],
                        ip.octets()[1],
                        ip.octets()[0]
                    ))?;
                    self.add(Record {
                        name: rev,
                        ttl,
                        data: RData::Name(RecordType::Ptr, name),
                    })
                }
            }
            b'6' | b'3' => {
                // IPv6 uses ':' internally, so find the longest prefix after
                // the owner that forms an address; later fields remain TTL
                // and timestamp in patched djbdns formats.
                let (ip, consumed) = (2..=f.len())
                    .rev()
                    .find_map(|end| {
                        f[1..end]
                            .join(":")
                            .parse::<Ipv6Addr>()
                            .ok()
                            .map(|ip| (ip, end))
                    })
                    .ok_or_else(|| Error::InvalidRecord("bad IPv6".into()))?;
                let ttl = field_opt(&f, consumed)
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(86400);
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::Aaaa(ip),
                });
                if kind == b'3' {
                    let hex = format!("{:032x}", u128::from(ip));
                    let rev = Name::from_str(&format!(
                        "{}.ip6.arpa",
                        hex.chars()
                            .rev()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(".")
                    ))?;
                    self.add(Record {
                        name: rev,
                        ttl,
                        data: RData::Name(RecordType::Ptr, name),
                    })
                }
            }
            b'C' | b'^' => self.add(Record {
                name,
                ttl,
                data: RData::Name(
                    if kind == b'C' {
                        RecordType::Cname
                    } else {
                        RecordType::Ptr
                    },
                    field(&f, 1)?.parse()?,
                ),
            }),
            b'@' => {
                let mx = field(&f, 1)?.parse()?;
                let pref = field_opt(&f, 2).and_then(|x| x.parse().ok()).unwrap_or(0);
                let ttl = field_opt(&f, 3)
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Mx(pref, mx),
                })
            }
            b'\'' => {
                let bytes = unescape(field(&f, 1)?)?;
                let chunks = bytes.chunks(255).map(<[u8]>::to_vec).collect();
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Txt(chunks),
                })
            }
            b':' => {
                let typ = RecordType::from_code(
                    field(&f, 1)?
                        .parse()
                        .map_err(|_| Error::InvalidRecord("bad type".into()))?,
                );
                self.add(Record {
                    name,
                    ttl: field_opt(&f, 3)
                        .and_then(|x| x.parse().ok())
                        .unwrap_or(86400),
                    data: RData::Opaque(typ, unescape(field(&f, 2)?)?),
                })
            }
            b'&' | b'.' => {
                let ip = field_opt(&f, 1).filter(|x| !x.is_empty());
                let host: Name = field_opt(&f, 2)
                    .filter(|x| !x.is_empty())
                    .unwrap_or("ns")
                    .parse()?;
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::Name(RecordType::Ns, host.clone()),
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: host.clone(),
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad glue".into()))?,
                        ),
                    })
                }
                if kind == b'.' {
                    self.authoritative.insert(name.clone());
                    let admin = format!("hostmaster.{name}").parse()?;
                    self.add(Record {
                        name,
                        ttl,
                        data: RData::Soa {
                            mname: host,
                            admin,
                            serial: 0,
                            refresh: 16384,
                            retry: 2048,
                            expire: 1048576,
                            minimum: 2560,
                        },
                    })
                }
            }
            b'Z' => {
                let ns = field(&f, 1)?.parse()?;
                let admin = field(&f, 2)?.parse()?;
                let nums = (3..8)
                    .map(|i| {
                        field_opt(&f, i)
                            .and_then(|x| x.parse().ok())
                            .unwrap_or([0, 16384, 2048, 1048576, 2560][i - 3])
                    })
                    .collect::<Vec<_>>();
                self.authoritative.insert(name.clone());
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Soa {
                        mname: ns,
                        admin,
                        serial: nums[0],
                        refresh: nums[1],
                        retry: nums[2],
                        expire: nums[3],
                        minimum: nums[4],
                    },
                })
            }
            _ => {
                return Err(Error::InvalidRecord(format!(
                    "unsupported marker {}",
                    kind as char
                )));
            }
        }
        Ok(())
    }
    pub fn lookup(&self, name: &Name, typ: RecordType) -> Lookup {
        let mut owner = name.clone();
        let mut rows = self.records.get(name);
        if rows.is_none() {
            let mut p = name.parent();
            while let Some(n) = p {
                let wc = n.wildcard();
                if let Some(r) = self.records.get(&wc) {
                    owner = wc;
                    rows = Some(r);
                    break;
                }
                p = n.parent()
            }
        }
        let zone = self
            .authoritative
            .iter()
            .filter(|z| name.is_subdomain_of(z))
            .max_by_key(|z| z.labels().count());
        let Some(records) = rows else {
            return if let Some(zone) = zone {
                Lookup::NxDomain(self.soa(zone))
            } else {
                Lookup::Refused
            };
        };
        let mut answer: Vec<Record> = records
            .iter()
            .filter(|r| {
                typ == RecordType::Any || r.rr_type() == typ || r.rr_type() == RecordType::Cname
            })
            .cloned()
            .collect();
        for r in &mut answer {
            r.name = name.clone()
        }
        if answer.is_empty() {
            Lookup::NoData(zone.and_then(|z| self.soa(z)))
        } else {
            let _ = owner;
            Lookup::Answer(answer)
        }
    }
    fn soa(&self, z: &Name) -> Option<Record> {
        self.records
            .get(z)?
            .iter()
            .find(|r| r.rr_type() == RecordType::Soa)
            .cloned()
    }
}
#[derive(Clone, Debug)]
pub enum Lookup {
    Answer(Vec<Record>),
    NoData(Option<Record>),
    NxDomain(Option<Record>),
    Refused,
}
fn split_fields(s: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut esc = false;
    for c in s.chars() {
        if c == ':' && !esc {
            out.push(String::new())
        } else {
            out.last_mut().unwrap().push(c)
        }
        esc = c == '\\' && !esc;
    }
    out
}
fn field(f: &[String], i: usize) -> Result<&str> {
    f.get(i)
        .map(String::as_str)
        .ok_or_else(|| Error::InvalidRecord(format!("missing field {}", i + 1)))
}
fn field_opt(f: &[String], i: usize) -> Option<&str> {
    f.get(i).map(String::as_str)
}
fn unescape(s: &str) -> Result<Vec<u8>> {
    let mut o = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 1;
            if i >= b.len() {
                return Err(Error::InvalidRecord("trailing escape".into()));
            }
            if i + 2 < b.len() && b[i..i + 3].iter().all(u8::is_ascii_digit) {
                let n =
                    (b[i] - 48) as u16 * 64 + (b[i + 1] - 48) as u16 * 8 + (b[i + 2] - 48) as u16;
                if n > 255 {
                    return Err(Error::InvalidRecord("octal overflow".into()));
                }
                o.push(n as u8);
                i += 3
            } else {
                o.push(b[i]);
                i += 1
            }
        } else {
            o.push(b[i]);
            i += 1
        }
    }
    Ok(o)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn common_markers() {
        let z=Zone::parse(".example:192.0.2.53:ns.example\n=www.example:192.0.2.1:60\n'example:hello\\072world\n6v6.example:2001:db8::1\n").unwrap();
        assert!(
            matches!(z.lookup(&"www.example".parse().unwrap(),RecordType::A),Lookup::Answer(x) if x[0].ttl==60)
        );
        assert!(matches!(
            z.lookup(&"missing.example".parse().unwrap(), RecordType::A),
            Lookup::NxDomain(Some(_))
        ));
        assert!(matches!(
            z.lookup(&"v6.example".parse().unwrap(), RecordType::Aaaa),
            Lookup::Answer(_)
        ))
    }
    #[test]
    fn wildcard() {
        let z = Zone::parse(".example::ns.example\n+*.example:192.0.2.4\n").unwrap();
        assert!(matches!(
            z.lookup(&"x.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ))
    }
    #[test]
    fn escaped_colon() {
        assert_eq!(unescape(r"a\072b").unwrap(), b"a:b")
    }
}
