---
type: "code-fragment"
fragment_id: "rgbdns-frag-2080b97a017a"
source_path: "src/zone.rs"
code_note: "DNS from First Principles/Code/src/zone.rs.source"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
symbol: "add_line"
kind: "fn"
start_line: 219
end_line: 299
---

# add_line

- Fragment ID: `rgbdns-frag-2080b97a017a`
- Source file: [[DNS from First Principles/Code/src/zone.rs.source|src/zone.rs]]
- Lines: 219-299
- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-2080b97a017a", "codeNote": "DNS from First Principles/Code/src/zone.rs.source", "heading": "rgbdns-frag-2080b97a017a: fn add_line", "sourcePath": "src/zone.rs", "startLine": 219, "endLine": 299}
```

## Excerpt

<span id="rgbdns-frag-2080b97a017a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2080b97a017a: fn add_line

```rust
    fn add_line(&mut self, line: &str) -> Result<()> {
        let kind = line.as_bytes()[0];
        let f = split_fields(&line[1..]);
        if kind == b'%' {
            let location = location_code(field_opt(&f, 0).unwrap_or_default());
            let prefix = field_opt(&f, 1)
                .unwrap_or_default()
                .split('.')
                .filter(|field| !field.is_empty())
                .map(|field| {
                    field
                        .parse::<u8>()
                        .map_err(|_| Error::InvalidRecord("bad location IP prefix".into()))
                })
                .collect::<Result<Vec<_>>>()?;
            if prefix.len() > 4 {
                return Err(Error::InvalidRecord(
                    "location IP prefix has more than four octets".into(),
                ));
            }
            self.locations.push((prefix, location));
            return Ok(());
        }
        let name = field(&f, 0)?.parse::<Name>()?;
        self.current_metadata = match kind {
            b'Z' => record_metadata(&f, 9, 10),
            b'.' | b'&' => record_metadata(&f, 4, 5),
            b'+' | b'=' | b'C' | b'^' | b'\'' => record_metadata(&f, 3, 4),
            b'@' => record_metadata(&f, 5, 6),
            b'S' => record_metadata(&f, 7, 8),
            b':' => record_metadata(&f, 4, 5),
            _ => RecordMetadata::default(),
        };
        match kind {
            b'A' => {
                let target = field(&f, 1)?.parse::<Name>()?;
                let ttl = number_or(&f, 2, 300);
                if ttl == 0 {
                    return Err(Error::InvalidRecord("ANAME TTL must be positive".into()));
                }
                if let Some(existing) = self.anames.insert(name.clone(), Aname { target, ttl })
                    && self.anames[&name] != existing
                {
                    return Err(Error::InvalidRecord(format!(
                        "multiple ANAME targets at {name}"
                    )));
                }
                let mut node = Some(name);
                while let Some(value) = node {
                    self.nodes.insert(value.clone());
                    self.unqualified_nodes.insert(value.clone());
                    node = value.parent();
                }
            }
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
                // fefe's djbdns IPv6 patch deliberately uses a flat 32-digit
```
