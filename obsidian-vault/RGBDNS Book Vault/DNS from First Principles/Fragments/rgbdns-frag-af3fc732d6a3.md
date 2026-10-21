---
type: "code-fragment"
fragment_id: "rgbdns-frag-af3fc732d6a3"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tinydns_line"
kind: "fn"
start_line: 396
end_line: 451
---

# tinydns_line

- Fragment ID: `rgbdns-frag-af3fc732d6a3`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 396-451
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-af3fc732d6a3", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "sourcePath": "src/axfr.rs", "startLine": 396, "endLine": 451}
```

## Excerpt

<span id="rgbdns-frag-af3fc732d6a3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-af3fc732d6a3: fn tinydns_line

```rust
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
        RData::Opaque(RecordType::Unknown(RGBDNS_ANAME_RECORD), bytes)
            if bytes.starts_with(RGBDNS_ANAME_MAGIC) =>
        {
            let target = decode_aname_target(&bytes[RGBDNS_ANAME_MAGIC.len()..])?;
            format!("A{owner}:{}:{ttl}", tinydns_name(&target))
        }
        RData::Opaque(typ, bytes) => format!(":{owner}:{}:{}:{ttl}", typ.code(), escape(bytes)),
        RData::Opt { .. } => return Err(Error::Format("OPT is invalid in AXFR zone data")),
        RData::Name(_, _) => return Err(Error::Format("invalid name-bearing RDATA type")),
    })
}

```
