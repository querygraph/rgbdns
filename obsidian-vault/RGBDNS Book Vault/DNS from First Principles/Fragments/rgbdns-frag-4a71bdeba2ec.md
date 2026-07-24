---
type: "code-fragment"
fragment_id: "rgbdns-frag-4a71bdeba2ec"
source_path: "src/cdb.rs"
code_note: "DNS from First Principles/Code/src/cdb.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "decode_record"
kind: "fn"
start_line: 125
end_line: 178
---

# decode_record

- Fragment ID: `rgbdns-frag-4a71bdeba2ec`
- Source file: [[DNS from First Principles/Code/src/cdb.rs.source|src/cdb.rs]]
- Lines: 125-178
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-4a71bdeba2ec", "codeNote": "DNS from First Principles/Code/src/cdb.rs.source", "heading": "rgbdns-frag-4a71bdeba2ec: fn decode_record", "sourcePath": "src/cdb.rs", "startLine": 125, "endLine": 178}
```

## Excerpt

<span id="rgbdns-frag-4a71bdeba2ec" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4a71bdeba2ec: fn decode_record

```rust
fn decode_record(key: &[u8], value: &[u8]) -> Result<(Record, RecordMetadata)> {
    if value.len() < 15 {
        return Err(Error::Format("short tinydns CDB value"));
    }
    let mut name = decode_name(key)?;
    let typ = u16::from_be_bytes([value[0], value[1]]);
    let marker = value[2];
    let (header, rdata_offset, location) = match marker {
        b'=' | b'*' => (3, 15, None),
        b'>' | b'+' if value.len() >= 17 => (5, 17, Some([value[3], value[4]])),
        b'>' | b'+' => return Err(Error::Format("short location-specific CDB value")),
        _ => return Err(Error::Format("invalid tinydns CDB marker")),
    };
    if marker == b'*' || marker == b'+' {
        name = name.with_wildcard();
    }
    let ttl = u32::from_be_bytes([
        value[header],
        value[header + 1],
        value[header + 2],
        value[header + 3],
    ]);
    let cutoff = u64::from_be_bytes(
        value[header + 4..header + 12]
            .try_into()
            .map_err(|_| Error::Format("short tinydns cutoff"))?,
    );
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
    let record = Message::decode(&packet)?
        .answers
        .into_iter()
        .next()
        .map(|mut record| {
            record.name = name;
            record
        })
        .ok_or(Error::Format("missing decoded CDB record"))?;
    Ok((record, RecordMetadata { cutoff, location }))
}

```
