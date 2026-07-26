---
type: "code-fragment"
fragment_id: "rgbdns-frag-d0d89aa0192c"
source_path: "src/axfr.rs"
code_note: "DNS from First Principles/Code/src/axfr.rs.source"
language: "rust"
subsystem: "Transport and zone transfer"
crate: "rgbdns"
symbol: "tinydns_line"
kind: "fn"
start_line: 305
end_line: 356
---

# tinydns_line

- Fragment ID: `rgbdns-frag-d0d89aa0192c`
- Source file: [[DNS from First Principles/Code/src/axfr.rs.source|src/axfr.rs]]
- Lines: 305-356
- Subsystem: [[DNS from First Principles/Subsystems/Transport and zone transfer|Transport and zone transfer]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-d0d89aa0192c", "codeNote": "DNS from First Principles/Code/src/axfr.rs.source", "heading": "rgbdns-frag-d0d89aa0192c: fn tinydns_line", "sourcePath": "src/axfr.rs", "startLine": 305, "endLine": 356}
```

## Excerpt

<span id="rgbdns-frag-d0d89aa0192c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d0d89aa0192c: fn tinydns_line

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
        RData::Opaque(typ, bytes) => {
            format!(":{owner}:{}:{}:{ttl}", typ.code(), escape(bytes))
        }
        RData::Opt { .. } => return Err(Error::Format("OPT is invalid in AXFR zone data")),
        RData::Name(_, _) => return Err(Error::Format("invalid name-bearing RDATA type")),
    })
}

```
