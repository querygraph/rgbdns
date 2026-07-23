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
    delegations: BTreeSet<Name>,
}
impl Zone {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.extension().is_some_and(|extension| extension == "cdb") {
            crate::cdb::load(path)
        } else {
            Self::parse(&fs::read_to_string(path)?)
        }
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
    pub(crate) fn records(&self) -> impl Iterator<Item = &Record> {
        self.records.values().flatten()
    }
    pub(crate) fn from_compiled_records(records: Vec<Record>) -> Self {
        let mut zone = Self::default();
        for record in records {
            if record.rr_type() == RecordType::Soa {
                zone.authoritative.insert(record.name.clone());
            }
            zone.add(record);
        }
        let ns_owners = zone
            .records
            .iter()
            .filter(|(_, records)| records.iter().any(|r| r.rr_type() == RecordType::Ns))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        for owner in ns_owners {
            if !zone.authoritative.contains(&owner) {
                zone.delegations.insert(owner);
            }
        }
        zone
    }
    fn add_line(&mut self, line: &str) -> Result<()> {
        let kind = line.as_bytes()[0];
        let f = split_fields(&line[1..]);
        let name = field(&f, 0)?.parse::<Name>()?;
        match kind {
            b'Z' => ensure_unqualified(&f, 9, 10)?,
            b'.' | b'&' => ensure_unqualified(&f, 4, 5)?,
            b'+' | b'=' | b'C' | b'^' | b'\'' => ensure_unqualified(&f, 3, 4)?,
            b'@' => ensure_unqualified(&f, 5, 6)?,
            b'S' => ensure_unqualified(&f, 7, 8)?,
            b':' => ensure_unqualified(&f, 4, 5)?,
            _ => {}
        }
        match kind {
            b'=' | b'+' => {
                let ttl = number_or(&f, 2, 86400);
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
                ensure_unqualified(&f, consumed + 1, consumed + 2)?;
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
            b'C' | b'^' => {
                let ttl = number_or(&f, 2, 86400);
                self.add(Record {
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
                })
            }
            b'@' => {
                let ip = field_opt(&f, 1).filter(|value| !value.is_empty());
                let mx = expanded_target(field_opt(&f, 2).unwrap_or_default(), "mx", &name)?;
                let pref = number_or(&f, 3, 0);
                let ttl = number_or(&f, 4, 86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Mx(pref, mx.clone()),
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: mx,
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad MX glue".into()))?,
                        ),
                    });
                }
            }
            b'S' => {
                let ip = field_opt(&f, 1).filter(|x| !x.is_empty());
                let mut target = field(&f, 2)?.to_owned();
                if !target.contains('.') {
                    target = format!("{target}.srv.{name}");
                }
                let target: Name = target.parse()?;
                let number = |index: usize, default: u16| -> Result<u16> {
                    match field_opt(&f, index).filter(|x| !x.is_empty()) {
                        Some(value) => value
                            .parse()
                            .map_err(|_| Error::InvalidRecord(format!("bad SRV field {index}"))),
                        None => Ok(default),
                    }
                };
                let port = number(3, 0)?;
                let priority = number(4, 0)?;
                let weight = number(5, 0)?;
                let ttl = field_opt(&f, 6)
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Srv {
                        priority,
                        weight,
                        port,
                        target: target.clone(),
                    },
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: target,
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad SRV glue".into()))?,
                        ),
                    });
                }
            }
            b'\'' => {
                let ttl = number_or(&f, 2, 86400);
                let bytes = unescape(field(&f, 1)?)?;
                let chunks = bytes.chunks(127).map(<[u8]>::to_vec).collect();
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
                let ttl = number_or(&f, 3, 259200);
                let ip = field_opt(&f, 1).filter(|x| !x.is_empty());
                let host = expanded_target(field_opt(&f, 2).unwrap_or_default(), "ns", &name)?;
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
                        ttl: if ttl == 0 { 0 } else { 2560 },
                        data: RData::Soa {
                            mname: host,
                            admin,
                            serial: 1,
                            refresh: 16384,
                            retry: 2048,
                            expire: 1048576,
                            minimum: 2560,
                        },
                    })
                } else {
                    self.delegations.insert(name);
                }
            }
            b'Z' => {
                let ns = field(&f, 1)?.parse()?;
                let admin = field(&f, 2)?.parse()?;
                let nums = [
                    number_or(&f, 3, 1),
                    number_or(&f, 4, 16384),
                    number_or(&f, 5, 2048),
                    number_or(&f, 6, 1048576),
                    number_or(&f, 7, 2560),
                ];
                let ttl = number_or(&f, 8, 2560);
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
        if let Some(delegation) = self
            .delegations
            .iter()
            .filter(|owner| name.is_subdomain_of(owner))
            .max_by_key(|owner| owner.labels().count())
        {
            let authorities = self
                .records
                .get(delegation)
                .into_iter()
                .flatten()
                .filter(|record| record.rr_type() == RecordType::Ns)
                .cloned()
                .collect::<Vec<_>>();
            let mut additionals = Vec::new();
            for authority in &authorities {
                let RData::Name(RecordType::Ns, target) = &authority.data else {
                    continue;
                };
                if !target.is_subdomain_of(delegation) {
                    continue;
                }
                additionals.extend(
                    self.records
                        .get(target)
                        .into_iter()
                        .flatten()
                        .filter(|record| {
                            matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
                        })
                        .cloned(),
                );
            }
            return Lookup::Referral {
                authorities,
                additionals,
            };
        }
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
    Referral {
        authorities: Vec<Record>,
        additionals: Vec<Record>,
    },
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
fn number_or<T: FromStr + Copy>(fields: &[String], index: usize, default: T) -> T {
    field_opt(fields, index)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
fn expanded_target(value: &str, role: &str, owner: &Name) -> Result<Name> {
    let target = if value.contains('.') {
        value.to_owned()
    } else {
        format!("{value}.{role}.{owner}")
    };
    target.parse()
}
fn ensure_unqualified(fields: &[String], timestamp: usize, location: usize) -> Result<()> {
    if field_opt(fields, timestamp).is_some_and(|value| !value.is_empty()) {
        return Err(Error::InvalidRecord(
            "timestamp-qualified records are not supported yet".into(),
        ));
    }
    if field_opt(fields, location).is_some_and(|value| !value.is_empty()) {
        return Err(Error::InvalidRecord(
            "location-qualified records are not supported yet".into(),
        ));
    }
    Ok(())
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
            if (b'0'..=b'7').contains(&b[i]) {
                let mut n = b[i] - b'0';
                i += 1;
                for _ in 0..2 {
                    if i < b.len() && (b'0'..=b'7').contains(&b[i]) {
                        n = n.wrapping_mul(8).wrapping_add(b[i] - b'0');
                        i += 1;
                    } else {
                        break;
                    }
                }
                o.push(n)
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
    #[test]
    fn patched_srv_marker_and_glue() {
        let z = Zone::parse("S_sip._tcp.example:192.0.2.7:sip:5060:10:20:300\n").unwrap();
        assert!(matches!(
            z.lookup(
                &"_sip._tcp.example".parse().unwrap(),
                RecordType::Srv
            ),
            Lookup::Answer(records)
                if matches!(
                    &records[0].data,
                    RData::Srv { priority: 10, weight: 20, port: 5060, target }
                        if target.to_string() == "sip.srv._sip._tcp.example."
                )
        ));
        assert!(matches!(
            z.lookup(&"sip.srv._sip._tcp.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ));
    }
    #[test]
    fn original_ns_and_mx_field_positions_and_expansion() {
        let z = Zone::parse(
            ".example:192.0.2.53:a:300\n\
             @example:192.0.2.25:mail:20:400\n",
        )
        .unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Ns),
            Lookup::Answer(records)
                if records[0].ttl == 300
                    && matches!(&records[0].data, RData::Name(_, target)
                        if target.to_string() == "a.ns.example.")
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Mx),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && matches!(&records[0].data, RData::Mx(20, target)
                        if target.to_string() == "mail.mx.example.")
        ));
        assert!(matches!(
            z.lookup(&"mail.mx.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 25))
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records) if records[0].ttl == 2560
        ));
    }

    #[test]
    fn explicit_soa_uses_ttl_field_nine() {
        let z = Zone::parse("Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n").unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records)
                if records[0].ttl == 12
                    && matches!(&records[0].data, RData::Soa {
                        serial: 7,
                        refresh: 8,
                        retry: 9,
                        expire: 10,
                        minimum: 11,
                        ..
                    })
        ));
    }

    #[test]
    fn text_escapes_use_one_to_three_octal_digits() {
        assert_eq!(unescape(r"\1\12\123\8").unwrap(), [1, 10, 83, b'8']);
    }
}
