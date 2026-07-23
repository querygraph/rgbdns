//! djbdns-compatible `data.cdb` compilation and bounded loading.

use crate::{Error, Message, Name, RData, Record, RecordType, Result, zone::Zone};
use std::{fs, path::Path};

const HEADER_LEN: usize = 256 * 8;
const MAX_DATABASE_SIZE: u64 = 1 << 30;

pub fn compile(zone: &Zone, path: impl AsRef<Path>) -> Result<()> {
    let filename = path.as_ref().to_string_lossy().into_owned();
    let mut writer = cdb::CDBWriter::create(filename)
        .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    for record in zone.records() {
        if record.rr_type() == RecordType::Opt {
            continue;
        }
        let (owner, marker) = match record.name.without_wildcard() {
            Some(parent) => (parent, b'*'),
            None => (record.name.clone(), b'='),
        };
        let mut value = Vec::new();
        value.extend(record.rr_type().code().to_be_bytes());
        value.push(marker);
        value.extend(record.ttl.to_be_bytes());
        value.extend([0; 8]); // no tai64 activation/expiration cutoff
        encode_rdata(&record.data, &mut value)?;
        writer
            .add(&owner.to_wire(), &value)
            .map_err(|error| Error::Io(std::io::Error::other(error)))?;
    }
    writer
        .finish()
        .map_err(|error| Error::Io(std::io::Error::other(error)))
}

pub fn load(path: impl AsRef<Path>) -> Result<Zone> {
    let metadata = fs::metadata(path.as_ref())?;
    if metadata.len() > MAX_DATABASE_SIZE {
        return Err(Error::Format("CDB exceeds configured safety limit"));
    }
    let bytes = fs::read(path)?;
    if bytes.len() < HEADER_LEN {
        return Err(Error::Format("short CDB header"));
    }
    let data_end = (0..256)
        .filter_map(|index| {
            let offset = index * 8;
            let position = le_u32(&bytes[offset..offset + 4]) as usize;
            (position >= HEADER_LEN && position <= bytes.len()).then_some(position)
        })
        .min()
        .unwrap_or(bytes.len());
    let mut position = HEADER_LEN;
    let mut records = Vec::new();
    while position < data_end {
        let header = bytes
            .get(position..position + 8)
            .ok_or(Error::Format("truncated CDB record header"))?;
        let key_len = le_u32(&header[..4]) as usize;
        let data_len = le_u32(&header[4..]) as usize;
        position += 8;
        let key_end = position
            .checked_add(key_len)
            .filter(|end| *end <= data_end)
            .ok_or(Error::Format("invalid CDB key length"))?;
        let value_end = key_end
            .checked_add(data_len)
            .filter(|end| *end <= data_end)
            .ok_or(Error::Format("invalid CDB value length"))?;
        let key = &bytes[position..key_end];
        let value = &bytes[key_end..value_end];
        position = value_end;
        if key.starts_with(b"\0%") {
            continue; // location mapping, consumed by the location-aware loader later
        }
        records.push(decode_record(key, value)?);
    }
    if position != data_end {
        return Err(Error::Format("CDB data section is misaligned"));
    }
    Ok(Zone::from_compiled_records(records))
}

fn decode_record(key: &[u8], value: &[u8]) -> Result<Record> {
    if value.len() < 15 {
        return Err(Error::Format("short tinydns CDB value"));
    }
    if value[7..15] != [0; 8] {
        return Err(Error::Format(
            "time-qualified tinydns CDB records require request-time filtering",
        ));
    }
    let mut name = decode_name(key)?;
    let typ = u16::from_be_bytes([value[0], value[1]]);
    let marker = value[2];
    let rdata_offset = match marker {
        b'=' | b'*' => 15,
        b'>' | b'+' => {
            return Err(Error::Format(
                "location-qualified tinydns CDB records require client-aware lookup",
            ));
        }
        _ => return Err(Error::Format("invalid tinydns CDB marker")),
    };
    if marker == b'*' {
        name = name.with_wildcard();
    }
    let ttl = u32::from_be_bytes([value[3], value[4], value[5], value[6]]);
    let rdata = value
        .get(rdata_offset..)
        .ok_or(Error::Format("invalid CDB RDATA offset"))?;
    let mut packet = vec![0; 12];
    packet[7] = 1; // ANCOUNT
    packet.push(0); // root owner
    packet.extend(typ.to_be_bytes());
    packet.extend(1u16.to_be_bytes());
    packet.extend(ttl.to_be_bytes());
    packet.extend(
        u16::try_from(rdata.len())
            .map_err(|_| Error::Format("CDB RDATA is too long"))?
            .to_be_bytes(),
    );
    packet.extend(rdata);
    Message::decode(&packet)?
        .answers
        .into_iter()
        .next()
        .map(|mut record| {
            record.name = name;
            record
        })
        .ok_or(Error::Format("missing decoded CDB record"))
}

fn encode_rdata(data: &RData, out: &mut Vec<u8>) -> Result<()> {
    match data {
        RData::A(address) => out.extend(address.octets()),
        RData::Aaaa(address) => out.extend(address.octets()),
        RData::Name(_, name) => out.extend(name.to_wire()),
        RData::Mx(preference, name) => {
            out.extend(preference.to_be_bytes());
            out.extend(name.to_wire());
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
            out.extend(mname.to_wire());
            out.extend(admin.to_wire());
            for value in [serial, refresh, retry, expire, minimum] {
                out.extend(value.to_be_bytes());
            }
        }
        RData::Txt(chunks) => {
            for chunk in chunks {
                for part in chunk.chunks(127) {
                    out.push(part.len() as u8);
                    out.extend(part);
                }
            }
        }
        RData::Srv {
            priority,
            weight,
            port,
            target,
        } => {
            out.extend(priority.to_be_bytes());
            out.extend(weight.to_be_bytes());
            out.extend(port.to_be_bytes());
            out.extend(target.to_wire());
        }
        RData::Caa { flags, tag, value } => {
            out.push(*flags);
            out.push(
                tag.len()
                    .try_into()
                    .map_err(|_| Error::Format("CAA tag is too long"))?,
            );
            out.extend(tag);
            out.extend(value);
        }
        RData::Opaque(_, bytes) => out.extend(bytes),
        RData::Opt { .. } => return Err(Error::Format("OPT cannot be stored in tinydns CDB")),
    }
    Ok(())
}

fn decode_name(wire: &[u8]) -> Result<Name> {
    let mut labels = Vec::new();
    let mut position = 0;
    loop {
        let length = *wire
            .get(position)
            .ok_or(Error::Format("truncated CDB owner"))? as usize;
        position += 1;
        if length == 0 {
            if position != wire.len() {
                return Err(Error::Format("trailing CDB owner data"));
            }
            break;
        }
        if length > 63 {
            return Err(Error::Format("invalid CDB owner label"));
        }
        let end = position
            .checked_add(length)
            .filter(|end| *end <= wire.len())
            .ok_or(Error::Format("truncated CDB owner label"))?;
        labels.push(wire[position..end].to_vec());
        position = end;
    }
    Name::from_labels(labels)
}

fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone::Lookup;
    use std::{net::Ipv4Addr, time::SystemTime};

    #[test]
    fn exact_cdb_roundtrip_preserves_lookup_semantics() {
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             +www.example:192.0.2.1:60\n\
             +*.wild.example:192.0.2.2:61\n\
             'example:hello\\072world:62\n\
             S_sip._tcp.example:192.0.2.7:sip.example:5060:10:20:63\n",
        )
        .unwrap();
        let path = std::env::temp_dir().join(format!(
            "rgbdns-cdb-{}-{}.cdb",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        compile(&zone, &path).unwrap();
        let loaded = load(&path).unwrap();
        fs::remove_file(path).unwrap();
        assert!(matches!(
            loaded.lookup(&"www.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
                    && records[0].ttl == 60
        ));
        assert!(matches!(
            loaded.lookup(&"x.wild.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records) if records[0].ttl == 61
        ));
        assert!(matches!(
            loaded.lookup(&"_sip._tcp.example".parse().unwrap(), RecordType::Srv),
            Lookup::Answer(_)
        ));
    }

    #[test]
    fn rejects_truncated_database() {
        let path = std::env::temp_dir().join(format!("rgbdns-short-{}.cdb", std::process::id()));
        fs::write(&path, [0; 20]).unwrap();
        assert!(load(&path).is_err());
        fs::remove_file(path).unwrap();
    }
}
