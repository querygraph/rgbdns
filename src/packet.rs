use crate::{Error, Name, Result};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordType {
    A,
    Ns,
    Cname,
    Soa,
    Ptr,
    Mx,
    Txt,
    Aaaa,
    Srv,
    Opt,
    Caa,
    Ds,
    Rrsig,
    Nsec,
    Dnskey,
    Axfr,
    Any,
    Unknown(u16),
}
impl RecordType {
    pub fn code(self) -> u16 {
        match self {
            Self::A => 1,
            Self::Ns => 2,
            Self::Cname => 5,
            Self::Soa => 6,
            Self::Ptr => 12,
            Self::Mx => 15,
            Self::Txt => 16,
            Self::Aaaa => 28,
            Self::Srv => 33,
            Self::Opt => 41,
            Self::Ds => 43,
            Self::Rrsig => 46,
            Self::Nsec => 47,
            Self::Dnskey => 48,
            Self::Axfr => 252,
            Self::Caa => 257,
            Self::Any => 255,
            Self::Unknown(n) => n,
        }
    }
    pub fn from_code(n: u16) -> Self {
        match n {
            1 => Self::A,
            2 => Self::Ns,
            5 => Self::Cname,
            6 => Self::Soa,
            12 => Self::Ptr,
            15 => Self::Mx,
            16 => Self::Txt,
            28 => Self::Aaaa,
            33 => Self::Srv,
            41 => Self::Opt,
            43 => Self::Ds,
            46 => Self::Rrsig,
            47 => Self::Nsec,
            48 => Self::Dnskey,
            252 => Self::Axfr,
            257 => Self::Caa,
            255 => Self::Any,
            n => Self::Unknown(n),
        }
    }
}
impl std::str::FromStr for RecordType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_ascii_uppercase().as_str() {
            "A" => Self::A,
            "NS" => Self::Ns,
            "CNAME" => Self::Cname,
            "SOA" => Self::Soa,
            "PTR" => Self::Ptr,
            "MX" => Self::Mx,
            "TXT" => Self::Txt,
            "AAAA" => Self::Aaaa,
            "SRV" => Self::Srv,
            "OPT" => Self::Opt,
            "CAA" => Self::Caa,
            "DS" => Self::Ds,
            "RRSIG" => Self::Rrsig,
            "NSEC" => Self::Nsec,
            "DNSKEY" => Self::Dnskey,
            "AXFR" => Self::Axfr,
            "ANY" => Self::Any,
            x => Self::Unknown(
                x.parse()
                    .map_err(|_| Error::Format("unknown record type"))?,
            ),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Question {
    pub name: Name,
    pub qtype: RecordType,
    pub qclass: u16,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub name: Name,
    pub ttl: u32,
    pub data: RData,
}
impl Record {
    pub fn rr_type(&self) -> RecordType {
        self.data.rr_type()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RData {
    A(Ipv4Addr),
    Aaaa(Ipv6Addr),
    Name(RecordType, Name),
    Mx(u16, Name),
    Soa {
        mname: Name,
        admin: Name,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    Txt(Vec<Vec<u8>>),
    Srv {
        priority: u16,
        weight: u16,
        port: u16,
        target: Name,
    },
    Caa {
        flags: u8,
        tag: Vec<u8>,
        value: Vec<u8>,
    },
    Opt {
        udp_payload: u16,
        extended_rcode: u8,
        version: u8,
        flags: u16,
        options: Vec<u8>,
    },
    Opaque(RecordType, Vec<u8>),
}
impl RData {
    pub fn rr_type(&self) -> RecordType {
        match self {
            Self::A(_) => RecordType::A,
            Self::Aaaa(_) => RecordType::Aaaa,
            Self::Name(t, _) => *t,
            Self::Mx(..) => RecordType::Mx,
            Self::Soa { .. } => RecordType::Soa,
            Self::Txt(_) => RecordType::Txt,
            Self::Srv { .. } => RecordType::Srv,
            Self::Caa { .. } => RecordType::Caa,
            Self::Opt { .. } => RecordType::Opt,
            Self::Opaque(t, _) => *t,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Message {
    pub id: u16,
    pub flags: u16,
    pub questions: Vec<Question>,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}
impl<'a> Reader<'a> {
    fn u8(&mut self) -> Result<u8> {
        let x = *self
            .b
            .get(self.p)
            .ok_or(Error::Format("truncated packet"))?;
        self.p += 1;
        Ok(x)
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes([
            self.u8()?,
            self.u8()?,
            self.u8()?,
            self.u8()?,
        ]))
    }
    fn name(&mut self) -> Result<Name> {
        let mut labels = Vec::new();
        let mut pos = self.p;
        let mut jumped = false;
        let mut hops = 0;
        loop {
            if hops > 128 {
                return Err(Error::Format("compression pointer loop"));
            }
            hops += 1;
            let n = *self.b.get(pos).ok_or(Error::Format("truncated name"))?;
            if n & 0xc0 == 0xc0 {
                let b = *self
                    .b
                    .get(pos + 1)
                    .ok_or(Error::Format("truncated pointer"))?;
                let q = (((n & 0x3f) as usize) << 8) | b as usize;
                if q >= pos {
                    return Err(Error::Format("compression pointer is not backward"));
                }
                if !jumped {
                    self.p = pos + 2;
                    jumped = true;
                }
                pos = q;
                continue;
            }
            if n & 0xc0 != 0 {
                return Err(Error::Format("reserved label type"));
            }
            pos += 1;
            if n == 0 {
                if !jumped {
                    self.p = pos
                }
                break;
            }
            let end = pos + n as usize;
            if end > self.b.len() {
                return Err(Error::Format("truncated label"));
            }
            labels.push(self.b[pos..end].to_vec());
            pos = end;
        }
        Name::from_labels(labels)
    }
    fn record(&mut self) -> Result<Record> {
        let name = self.name()?;
        let typ = RecordType::from_code(self.u16()?);
        let class = self.u16()?;
        if typ != RecordType::Opt && class != 1 {
            return Err(Error::Format("non-IN record"));
        }
        let ttl = self.u32()?;
        let len = self.u16()? as usize;
        let end = self
            .p
            .checked_add(len)
            .filter(|e| *e <= self.b.len())
            .ok_or(Error::Format("truncated rdata"))?;
        let data = match typ {
            RecordType::A if len == 4 => RData::A(Ipv4Addr::new(
                self.u8()?,
                self.u8()?,
                self.u8()?,
                self.u8()?,
            )),
            RecordType::Aaaa if len == 16 => {
                let mut x = [0; 16];
                x.copy_from_slice(&self.b[self.p..end]);
                self.p = end;
                RData::Aaaa(x.into())
            }
            RecordType::Ns | RecordType::Cname | RecordType::Ptr => RData::Name(typ, self.name()?),
            RecordType::Mx => {
                let p = self.u16()?;
                RData::Mx(p, self.name()?)
            }
            RecordType::Srv => RData::Srv {
                priority: self.u16()?,
                weight: self.u16()?,
                port: self.u16()?,
                target: self.name()?,
            },
            RecordType::Soa => RData::Soa {
                mname: self.name()?,
                admin: self.name()?,
                serial: self.u32()?,
                refresh: self.u32()?,
                retry: self.u32()?,
                expire: self.u32()?,
                minimum: self.u32()?,
            },
            RecordType::Caa if len >= 2 => {
                let flags = self.u8()?;
                let tag_len = self.u8()? as usize;
                if self.p + tag_len > end {
                    return Err(Error::Format("bad CAA tag length"));
                }
                let tag = self.b[self.p..self.p + tag_len].to_vec();
                self.p += tag_len;
                let value = self.b[self.p..end].to_vec();
                self.p = end;
                RData::Caa { flags, tag, value }
            }
            RecordType::Opt => {
                if !name.is_root() {
                    return Err(Error::Format("OPT owner is not root"));
                }
                let options = self.b[self.p..end].to_vec();
                validate_edns_options(&options)?;
                self.p = end;
                RData::Opt {
                    udp_payload: class,
                    extended_rcode: (ttl >> 24) as u8,
                    version: (ttl >> 16) as u8,
                    flags: ttl as u16,
                    options,
                }
            }
            RecordType::Txt => {
                let mut v = Vec::new();
                while self.p < end {
                    let n = self.u8()? as usize;
                    if self.p + n > end {
                        return Err(Error::Format("bad TXT"));
                    }
                    v.push(self.b[self.p..self.p + n].to_vec());
                    self.p += n
                }
                RData::Txt(v)
            }
            RecordType::A | RecordType::Aaaa => {
                return Err(Error::Format("invalid address RDLENGTH"));
            }
            _ => {
                let v = self.b[self.p..end].to_vec();
                self.p = end;
                RData::Opaque(typ, v)
            }
        };
        if self.p != end {
            return Err(Error::Format("rdata length mismatch"));
        }
        Ok(Record {
            name,
            ttl: if typ == RecordType::Opt { 0 } else { ttl },
            data,
        })
    }
}

fn validate_edns_options(mut options: &[u8]) -> Result<()> {
    while !options.is_empty() {
        if options.len() < 4 {
            return Err(Error::Format("truncated EDNS option"));
        }
        let len = u16::from_be_bytes([options[2], options[3]]) as usize;
        options = options
            .get(4 + len..)
            .ok_or(Error::Format("truncated EDNS option data"))?;
    }
    Ok(())
}
impl Message {
    pub fn decode(b: &[u8]) -> Result<Self> {
        if b.len() < 12 {
            return Err(Error::Format("short header"));
        }
        let mut r = Reader { b, p: 0 };
        let id = r.u16()?;
        let flags = r.u16()?;
        let qd = r.u16()?;
        let an = r.u16()?;
        let ns = r.u16()?;
        let ar = r.u16()?;
        if qd > 64 || an > 4096 || ns > 4096 || ar > 4096 {
            return Err(Error::Format("excessive section count"));
        }
        let mut m = Self {
            id,
            flags,
            ..Self::default()
        };
        for _ in 0..qd {
            let name = r.name()?;
            m.questions.push(Question {
                name,
                qtype: RecordType::from_code(r.u16()?),
                qclass: r.u16()?,
            })
        }
        for _ in 0..an {
            m.answers.push(r.record()?)
        }
        for _ in 0..ns {
            m.authorities.push(r.record()?)
        }
        for _ in 0..ar {
            m.additionals.push(r.record()?)
        }
        if r.p != b.len() {
            return Err(Error::Format("trailing DNS packet data"));
        }
        Ok(m)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut w = Writer(Vec::with_capacity(512));
        w.u16(self.id);
        w.u16(self.flags);
        for n in [
            self.questions.len(),
            self.answers.len(),
            self.authorities.len(),
            self.additionals.len(),
        ] {
            w.u16(u16::try_from(n).map_err(|_| Error::Format("section too large"))?)
        }
        for q in &self.questions {
            w.name(&q.name)?;
            w.u16(q.qtype.code());
            w.u16(q.qclass)
        }
        for section in [&self.answers, &self.authorities, &self.additionals] {
            for r in section {
                w.record(r)?
            }
        }
        Ok(w.0)
    }
}
struct Writer(Vec<u8>);
impl Writer {
    fn u8(&mut self, n: u8) {
        self.0.push(n)
    }
    fn u16(&mut self, n: u16) {
        self.0.extend(n.to_be_bytes())
    }
    fn u32(&mut self, n: u32) {
        self.0.extend(n.to_be_bytes())
    }
    fn name(&mut self, n: &Name) -> Result<()> {
        if n.wire_len() > 255 {
            return Err(Error::Format("name too long"));
        }
        for l in n.labels() {
            self.u8(l.len() as u8);
            self.0.extend(l)
        }
        self.u8(0);
        Ok(())
    }
    fn record(&mut self, r: &Record) -> Result<()> {
        self.name(&r.name)?;
        self.u16(r.rr_type().code());
        match &r.data {
            RData::Opt {
                udp_payload,
                extended_rcode,
                version,
                flags,
                ..
            } => {
                self.u16(*udp_payload);
                self.u32(
                    u32::from(*extended_rcode) << 24
                        | u32::from(*version) << 16
                        | u32::from(*flags),
                );
            }
            _ => {
                self.u16(1);
                self.u32(r.ttl);
            }
        }
        let at = self.0.len();
        self.u16(0);
        let start = self.0.len();
        match &r.data {
            RData::A(x) => self.0.extend(x.octets()),
            RData::Aaaa(x) => self.0.extend(x.octets()),
            RData::Name(_, n) => self.name(n)?,
            RData::Mx(p, n) => {
                self.u16(*p);
                self.name(n)?
            }
            RData::Txt(v) => {
                for s in v {
                    if s.len() > 255 {
                        return Err(Error::Format("TXT chunk too long"));
                    }
                    self.u8(s.len() as u8);
                    self.0.extend(s)
                }
            }
            RData::Soa {
                mname: n,
                admin,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => {
                self.name(n)?;
                self.name(admin)?;
                for x in [serial, refresh, retry, expire, minimum] {
                    self.u32(*x)
                }
            }
            RData::Srv {
                priority,
                weight,
                port,
                target,
            } => {
                self.u16(*priority);
                self.u16(*weight);
                self.u16(*port);
                self.name(target)?
            }
            RData::Caa { flags, tag, value } => {
                self.u8(*flags);
                self.u8(tag
                    .len()
                    .try_into()
                    .map_err(|_| Error::Format("CAA tag too long"))?);
                self.0.extend(tag);
                self.0.extend(value)
            }
            RData::Opt { options, .. } => {
                validate_edns_options(options)?;
                self.0.extend(options)
            }
            RData::Opaque(_, v) => self.0.extend(v),
        }
        let len: u16 = (self.0.len() - start)
            .try_into()
            .map_err(|_| Error::Format("rdata too long"))?;
        self.0[at..at + 2].copy_from_slice(&len.to_be_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_roundtrip() {
        let m = Message {
            id: 42,
            flags: 0x100,
            questions: vec![Question {
                name: "Example.COM".parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        };
        assert_eq!(Message::decode(&m.encode().unwrap()).unwrap(), m)
    }
    #[test]
    fn rejects_pointer_loop() {
        let mut b = vec![0; 12];
        b[5] = 1;
        b.extend([0xc0, 0x0c, 0, 1, 0, 1]);
        assert!(Message::decode(&b).is_err())
    }
    #[test]
    fn rejects_trailing_packet_data_and_forward_compression() {
        let mut packet = Message {
            questions: vec![Question {
                name: "example".parse().unwrap(),
                qtype: RecordType::A,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap();
        packet.push(0);
        assert!(Message::decode(&packet).is_err());

        let mut forward = vec![0; 12];
        forward[5] = 1;
        forward.extend([0xc0, 18, 0, 1, 0, 1, 0]);
        assert!(Message::decode(&forward).is_err());
    }
    #[test]
    fn structured_records_and_edns_roundtrip() {
        let records = vec![
            Record {
                name: "example".parse().unwrap(),
                ttl: 60,
                data: RData::Soa {
                    mname: "ns.example".parse().unwrap(),
                    admin: "hostmaster.example".parse().unwrap(),
                    serial: 1,
                    refresh: 2,
                    retry: 3,
                    expire: 4,
                    minimum: 5,
                },
            },
            Record {
                name: "example".parse().unwrap(),
                ttl: 60,
                data: RData::Caa {
                    flags: 0,
                    tag: b"issue".to_vec(),
                    value: b"ca.example".to_vec(),
                },
            },
        ];
        let opt = Record {
            name: Name::root(),
            ttl: 0,
            data: RData::Opt {
                udp_payload: 1232,
                extended_rcode: 0,
                version: 0,
                flags: 0x8000,
                options: vec![0, 12, 0, 2, 0xaa, 0xbb],
            },
        };
        let message = Message {
            answers: records,
            additionals: vec![opt],
            ..Default::default()
        };
        assert_eq!(
            Message::decode(&message.encode().unwrap()).unwrap(),
            message
        );
    }
    #[test]
    fn rejects_malformed_edns_options_and_address_length() {
        let malformed_opt = [
            0, 0, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 41, 4, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0,
        ];
        assert!(Message::decode(&malformed_opt).is_err());
        let bad_a = [
            0, 0, 0x80, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 3, 1, 2, 3,
        ];
        assert!(Message::decode(&bad_a).is_err());
    }
}
